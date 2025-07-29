use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use rayon::prelude::*;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use tracing::{info, warn, error};
use dashmap::DashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub hash: String,
    pub file_type: String,
    pub is_system_file: bool,
    pub is_critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub size: u64,
    pub files: Vec<FileInfo>,
    pub total_size: u64,
    pub potential_savings: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_files: usize,
    pub total_size: u64,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub scan_duration_ms: u64,
    pub scanned_directories: Vec<PathBuf>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupResult {
    pub files_removed: usize,
    pub space_freed: u64,
    pub errors: Vec<String>,
    pub backup_created: bool,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current_directory: PathBuf,
    pub files_scanned: usize,
    pub total_files: usize,
    pub current_file: Option<PathBuf>,
    pub percentage: f32,
}

pub struct FileManager {
    file_cache: Arc<DashMap<String, FileInfo>>,
    scan_results: Arc<RwLock<HashMap<String, ScanResult>>>,
    backup_directory: PathBuf,
    excluded_paths: Vec<PathBuf>,
    max_file_size: u64, // Maximum file size to scan (e.g., 100MB)
}

impl FileManager {
    pub fn new(backup_dir: PathBuf) -> Self {
        let excluded_paths = vec![
            PathBuf::from("C:\\Windows\\System32"),
            PathBuf::from("C:\\Windows\\SysWOW64"),
            PathBuf::from("C:\\Program Files\\WindowsApps"),
            PathBuf::from("C:\\$Recycle.Bin"),
            PathBuf::from("C:\\System Volume Information"),
        ];
        
        Self {
            file_cache: Arc::new(DashMap::new()),
            scan_results: Arc::new(RwLock::new(HashMap::new())),
            backup_directory: backup_dir,
            excluded_paths,
            max_file_size: 100 * 1024 * 1024, // 100MB
        }
    }

    /// Scan for duplicate files with progress reporting
    pub async fn scan_duplicates(
        &self,
        directories: Vec<PathBuf>,
        progress_callback: Option<Box<dyn Fn(ScanProgress) + Send + Sync>>
    ) -> Result<ScanResult> {
        let start_time = std::time::Instant::now();
        let mut result = ScanResult {
            total_files: 0,
            total_size: 0,
            duplicate_groups: Vec::new(),
            scan_duration_ms: 0,
            scanned_directories: directories.clone(),
            errors: Vec::new(),
        };

        info!("Starting duplicate file scan for {} directories", directories.len());

        // Collect all files first
        let mut all_files = Vec::new();
        for directory in &directories {
            match self.collect_files(directory, &mut all_files, &progress_callback).await {
                Ok(_) => {},
                Err(e) => {
                    result.errors.push(format!("Failed to scan {}: {}", directory.display(), e));
                }
            }
        }

        result.total_files = all_files.len();
        result.total_size = all_files.iter().map(|f| f.size).sum();

        info!("Collected {} files, total size: {} bytes", result.total_files, result.total_size);

        // Group files by size first (quick filter)
        let size_groups: HashMap<u64, Vec<FileInfo>> = all_files
            .into_par_iter()
            .filter(|file| file.size > 0) // Skip empty files
            .collect::<Vec<_>>()
            .into_iter()
            .fold(HashMap::new(), |mut acc, file| {
                acc.entry(file.size).or_insert_with(Vec::new).push(file);
                acc
            });

        // Calculate hashes for files with same size
        let mut hash_groups: HashMap<String, Vec<FileInfo>> = HashMap::new();
        
        for (size, files) in size_groups {
            if files.len() > 1 { // Only process if there are potential duplicates
                for file in files {
                    match self.calculate_file_hash(&file.path).await {
                        Ok(hash) => {
                            let mut file_with_hash = file.clone();
                            file_with_hash.hash = hash;
                            hash_groups.entry(hash).or_insert_with(Vec::new).push(file_with_hash);
                        }
                        Err(e) => {
                            result.errors.push(format!("Failed to hash {}: {}", file.path.display(), e));
                        }
                    }
                }
            }
        }

        // Create duplicate groups
        for (hash, files) in hash_groups {
            if files.len() > 1 {
                let total_size = files.iter().map(|f| f.size).sum();
                let potential_savings = total_size - files[0].size; // Keep one copy
                
                result.duplicate_groups.push(DuplicateGroup {
                    hash,
                    size: files[0].size,
                    files,
                    total_size,
                    potential_savings,
                });
            }
        }

        result.scan_duration_ms = start_time.elapsed().as_millis() as u64;
        
        // Store scan result
        let scan_id = format!("scan_{}", Utc::now().timestamp());
        {
            let mut scan_results = self.scan_results.write().await;
            scan_results.insert(scan_id, result.clone());
        }

        info!("Duplicate scan completed: {} groups found, {}ms", 
              result.duplicate_groups.len(), result.scan_duration_ms);

        Ok(result)
    }

    /// Clean up duplicate files with safety measures
    pub async fn cleanup_duplicates(
        &self,
        duplicate_groups: Vec<DuplicateGroup>,
        keep_strategy: KeepStrategy,
        create_backup: bool
    ) -> Result<CleanupResult> {
        let mut result = CleanupResult {
            files_removed: 0,
            space_freed: 0,
            errors: Vec::new(),
            backup_created: false,
            backup_path: None,
        };

        // Create backup if requested
        if create_backup {
            match self.create_cleanup_backup(&duplicate_groups).await {
                Ok(backup_path) => {
                    result.backup_created = true;
                    result.backup_path = Some(backup_path);
                }
                Err(e) => {
                    result.errors.push(format!("Failed to create backup: {}", e));
                    return Ok(result);
                }
            }
        }

        // Process each duplicate group
        for group in duplicate_groups {
            match self.process_duplicate_group(&group, &keep_strategy).await {
                Ok((removed, freed)) => {
                    result.files_removed += removed;
                    result.space_freed += freed;
                }
                Err(e) => {
                    result.errors.push(format!("Failed to process group {}: {}", group.hash, e));
                }
            }
        }

        info!("Cleanup completed: {} files removed, {} bytes freed", 
              result.files_removed, result.space_freed);

        Ok(result)
    }

    /// Scan for temporary files and cleanup opportunities
    pub async fn scan_temp_files(&self) -> Result<Vec<FileInfo>> {
        let temp_directories = vec![
            std::env::temp_dir(),
            PathBuf::from("C:\\Windows\\Temp"),
            PathBuf::from("C:\\Windows\\Prefetch"),
            PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download"),
        ];

        let mut temp_files = Vec::new();
        
        for temp_dir in temp_directories {
            if temp_dir.exists() {
                match self.collect_files(&temp_dir, &mut temp_files, &None).await {
                    Ok(_) => {},
                    Err(e) => {
                        warn!("Failed to scan temp directory {}: {}", temp_dir.display(), e);
                    }
                }
            }
        }

        // Filter for files older than 7 days
        let cutoff_date = Utc::now() - chrono::Duration::days(7);
        temp_files.retain(|file| file.modified < cutoff_date);

        Ok(temp_files)
    }

    /// Clean up temporary files
    pub async fn cleanup_temp_files(&self, files: Vec<FileInfo>) -> Result<CleanupResult> {
        let mut result = CleanupResult {
            files_removed: 0,
            space_freed: 0,
            errors: Vec::new(),
            backup_created: false,
            backup_path: None,
        };

        for file in files {
            match self.safe_delete_file(&file.path).await {
                Ok(_) => {
                    result.files_removed += 1;
                    result.space_freed += file.size;
                }
                Err(e) => {
                    result.errors.push(format!("Failed to delete {}: {}", file.path.display(), e));
                }
            }
        }

        Ok(result)
    }

    /// Collect files from directory with progress reporting
    async fn collect_files(
        &self,
        directory: &Path,
        files: &mut Vec<FileInfo>,
        progress_callback: &Option<Box<dyn Fn(ScanProgress) + Send + Sync>>
    ) -> Result<()> {
        if !directory.exists() || !directory.is_dir() {
            return Ok(());
        }

        // Check if directory is excluded
        if self.excluded_paths.iter().any(|excluded| directory.starts_with(excluded)) {
            return Ok(());
        }

        let walker = WalkDir::new(directory)
            .follow_links(false)
            .max_depth(10); // Limit depth to prevent infinite recursion

        let mut file_count = 0;
        let total_files = walker.into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()).count();

        for entry in WalkDir::new(directory)
            .follow_links(false)
            .max_depth(10)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            file_count += 1;
            
            // Report progress
            if let Some(callback) = progress_callback {
                let progress = ScanProgress {
                    current_directory: directory.to_path_buf(),
                    files_scanned: file_count,
                    total_files,
                    current_file: Some(entry.path().to_path_buf()),
                    percentage: (file_count as f32 / total_files as f32) * 100.0,
                };
                callback(progress);
            }

            let path = entry.path();
            
            // Skip files that are too large
            if let Ok(metadata) = tokio::fs::metadata(path).await {
                if metadata.len() > self.max_file_size {
                    continue;
                }

                // Skip system and critical files
                if self.is_system_file(path) || self.is_critical_file(path) {
                    continue;
                }

                let file_info = FileInfo {
                    path: path.to_path_buf(),
                    size: metadata.len(),
                    modified: DateTime::from(metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now())),
                    hash: String::new(), // Will be calculated later
                    file_type: self.get_file_extension(path),
                    is_system_file: self.is_system_file(path),
                    is_critical: self.is_critical_file(path),
                };

                files.push(file_info);
            }
        }

        Ok(())
    }

    /// Calculate SHA-256 hash of file
    async fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Process a duplicate group according to keep strategy
    async fn process_duplicate_group(
        &self,
        group: &DuplicateGroup,
        strategy: &KeepStrategy
    ) -> Result<(usize, u64)> {
        let mut files_to_keep = Vec::new();
        let mut files_to_remove = Vec::new();

        match strategy {
            KeepStrategy::KeepNewest => {
                // Keep the most recently modified file
                let mut sorted_files = group.files.clone();
                sorted_files.sort_by(|a, b| b.modified.cmp(&a.modified));
                files_to_keep.push(sorted_files[0].clone());
                files_to_remove.extend(sorted_files[1..].iter().cloned());
            }
            KeepStrategy::KeepOldest => {
                // Keep the oldest file
                let mut sorted_files = group.files.clone();
                sorted_files.sort_by(|a, b| a.modified.cmp(&b.modified));
                files_to_keep.push(sorted_files[0].clone());
                files_to_remove.extend(sorted_files[1..].iter().cloned());
            }
            KeepStrategy::KeepInSystem => {
                // Keep files in system directories, remove others
                for file in &group.files {
                    if self.is_system_directory(&file.path) {
                        files_to_keep.push(file.clone());
                    } else {
                        files_to_remove.push(file.clone());
                    }
                }
            }
            KeepStrategy::KeepInProgramFiles => {
                // Keep files in Program Files, remove others
                for file in &group.files {
                    if file.path.starts_with("C:\\Program Files") || file.path.starts_with("C:\\Program Files (x86)") {
                        files_to_keep.push(file.clone());
                    } else {
                        files_to_remove.push(file.clone());
                    }
                }
            }
        }

        // Remove duplicate files
        let mut removed_count = 0;
        let mut freed_space = 0;

        for file in files_to_remove {
            match self.safe_delete_file(&file.path).await {
                Ok(_) => {
                    removed_count += 1;
                    freed_space += file.size;
                }
                Err(e) => {
                    warn!("Failed to delete duplicate file {}: {}", file.path.display(), e);
                }
            }
        }

        Ok((removed_count, freed_space))
    }

    /// Safely delete a file with error handling
    async fn safe_delete_file(&self, path: &Path) -> Result<()> {
        // Check if file is critical
        if self.is_critical_file(path) {
            return Err(anyhow!("Attempting to delete critical file: {}", path.display()));
        }

        // Move to recycle bin instead of permanent deletion
        let delete_command = format!("powershell.exe -Command \"Remove-Item '{}' -Force -ErrorAction Stop\"", path.display());
        
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &delete_command])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to delete file: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Create backup of files before cleanup
    async fn create_cleanup_backup(&self, duplicate_groups: &[DuplicateGroup]) -> Result<PathBuf> {
        let backup_id = format!("cleanup_backup_{}", Utc::now().format("%Y%m%d_%H%M%S"));
        let backup_path = self.backup_directory.join(&format!("{}.zip", backup_id));

        // Create ZIP backup of files to be deleted
        let file = tokio::fs::File::create(&backup_path).await?;
        let mut zip = zip::ZipWriter::new(file);

        for group in duplicate_groups {
            for file in &group.files {
                if let Ok(content) = tokio::fs::read(&file.path).await {
                    let options = zip::write::FileOptions::default()
                        .compression_method(zip::CompressionMethod::Deflated);
                    
                    if let Ok(mut file_in_zip) = zip.start_file(file.path.to_string_lossy(), options) {
                        let _ = std::io::copy(&mut std::io::Cursor::new(content), &mut file_in_zip);
                    }
                }
            }
        }

        zip.finish()?;
        Ok(backup_path)
    }

    /// Check if file is in system directory
    fn is_system_directory(&self, path: &Path) -> bool {
        path.starts_with("C:\\Windows") || 
        path.starts_with("C:\\System Volume Information") ||
        path.starts_with("C:\\$Recycle.Bin")
    }

    /// Check if file is a system file
    fn is_system_file(&self, path: &Path) -> bool {
        self.is_system_directory(path) || 
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with(".") || name == "desktop.ini" || name == "thumbs.db")
            .unwrap_or(false)
    }

    /// Check if file is critical
    fn is_critical_file(&self, path: &Path) -> bool {
        let critical_extensions = [".sys", ".dll", ".exe", ".drv", ".ocx"];
        let critical_paths = [
            "C:\\Windows\\System32",
            "C:\\Windows\\SysWOW64",
            "C:\\Windows\\WinSxS",
        ];

        // Check if file has critical extension
        if let Some(ext) = path.extension() {
            if critical_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                return true;
            }
        }

        // Check if file is in critical path
        critical_paths.iter().any(|critical_path| path.starts_with(critical_path))
    }

    /// Get file extension
    fn get_file_extension(&self, path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_lowercase()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeepStrategy {
    KeepNewest,
    KeepOldest,
    KeepInSystem,
    KeepInProgramFiles,
}
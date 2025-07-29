use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;
use winreg::enums::*;
use winreg::RegKey;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryBackup {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub backup_path: PathBuf,
    pub registry_keys: Vec<RegistryKeyInfo>,
    pub file_size: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryKeyInfo {
    pub path: String,
    pub key_type: String,
    pub value_count: usize,
    pub subkey_count: usize,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryScanResult {
    pub orphaned_keys: Vec<RegistryKeyInfo>,
    pub bloatware_keys: Vec<RegistryKeyInfo>,
    pub dangerous_keys: Vec<RegistryKeyInfo>,
    pub total_keys_scanned: usize,
    pub scan_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryOperation {
    pub operation_type: String,
    pub key_path: String,
    pub value_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
}

pub struct RegistryManager {
    backups: Arc<RwLock<HashMap<String, RegistryBackup>>>,
    operations_log: Arc<RwLock<Vec<RegistryOperation>>>,
    backup_directory: PathBuf,
}

impl RegistryManager {
    pub fn new(backup_dir: PathBuf) -> Self {
        Self {
            backups: Arc::new(RwLock::new(HashMap::new())),
            operations_log: Arc::new(RwLock::new(Vec::new())),
            backup_directory: backup_dir,
        }
    }

    /// Create a comprehensive registry backup with user prompt
    pub async fn create_backup(&self, description: String) -> Result<RegistryBackup> {
        let backup_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        // Create backup filename with timestamp
        let filename = format!(
            "registry_backup_{}_{}.reg",
            timestamp.format("%Y%m%d_%H%M%S"),
            backup_id[..8].to_string()
        );
        
        let backup_path = self.backup_directory.join(&filename);
        
        info!("Creating registry backup: {}", backup_path.display());
        
        // Export full registry
        let export_command = format!(
            "reg export HKLM \"{}\" /y",
            backup_path.display()
        );
        
        // Execute registry export
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &export_command])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Registry export failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        // Get file size and calculate checksum
        let metadata = tokio::fs::metadata(&backup_path).await?;
        let file_size = metadata.len();
        
        // Calculate MD5 checksum
        let file_content = tokio::fs::read(&backup_path).await?;
        let checksum = format!("{:x}", md5::compute(&file_content));
        
        // Scan registry keys for backup info
        let registry_keys = self.scan_registry_keys_for_backup().await?;
        
        let backup = RegistryBackup {
            id: backup_id,
            timestamp,
            description,
            backup_path,
            registry_keys,
            file_size,
            checksum,
        };
        
        // Store backup info
        {
            let mut backups = self.backups.write().await;
            backups.insert(backup_id.clone(), backup.clone());
        }
        
        info!("Registry backup created successfully: {}", backup_id);
        Ok(backup)
    }

    /// Scan for orphaned registry entries
    pub async fn scan_orphaned_entries(&self) -> Result<RegistryScanResult> {
        let start_time = std::time::Instant::now();
        let mut result = RegistryScanResult {
            orphaned_keys: Vec::new(),
            bloatware_keys: Vec::new(),
            dangerous_keys: Vec::new(),
            total_keys_scanned: 0,
            scan_duration_ms: 0,
        };
        
        info!("Starting orphaned registry entry scan");
        
        // Scan common uninstall keys
        let uninstall_keys = vec![
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ];
        
        for key_path in uninstall_keys {
            if let Ok(orphaned) = self.scan_uninstall_key(key_path).await {
                result.orphaned_keys.extend(orphaned);
            }
        }
        
        // Scan for bloatware patterns
        let bloatware_patterns = self.get_bloatware_registry_patterns();
        for pattern in bloatware_patterns {
            if let Ok(bloatware) = self.scan_for_pattern(&pattern).await {
                result.bloatware_keys.extend(bloatware);
            }
        }
        
        // Scan for dangerous keys
        let dangerous_patterns = self.get_dangerous_registry_patterns();
        for pattern in dangerous_patterns {
            if let Ok(dangerous) = self.scan_for_pattern(&pattern).await {
                result.dangerous_keys.extend(dangerous);
            }
        }
        
        result.scan_duration_ms = start_time.elapsed().as_millis() as u64;
        result.total_keys_scanned = result.orphaned_keys.len() + 
                                   result.bloatware_keys.len() + 
                                   result.dangerous_keys.len();
        
        info!("Registry scan completed: {} keys found in {}ms", 
              result.total_keys_scanned, result.scan_duration_ms);
        
        Ok(result)
    }

    /// Restore registry from backup
    pub async fn restore_backup(&self, backup_id: &str) -> Result<()> {
        let backup = {
            let backups = self.backups.read().await;
            backups.get(backup_id)
                .ok_or_else(|| anyhow!("Backup not found: {}", backup_id))?
                .clone()
        };
        
        info!("Restoring registry from backup: {}", backup_id);
        
        // Verify backup file integrity
        self.verify_backup_integrity(&backup).await?;
        
        // Create restore point before restoration
        self.create_system_restore_point("Before registry restoration").await?;
        
        // Import registry backup
        let import_command = format!(
            "reg import \"{}\"",
            backup.backup_path.display()
        );
        
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &import_command])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Registry import failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        info!("Registry backup restored successfully: {}", backup_id);
        Ok(())
    }

    /// Delete registry key with safety checks
    pub async fn delete_registry_key(&self, key_path: &str, force: bool) -> Result<()> {
        // Log the operation
        let operation = RegistryOperation {
            operation_type: "DELETE_KEY".to_string(),
            key_path: key_path.to_string(),
            value_name: None,
            old_value: None,
            new_value: None,
            timestamp: Utc::now(),
            success: false,
            error_message: None,
        };
        
        // Check if key is dangerous
        if !force && self.is_dangerous_key(key_path).await {
            return Err(anyhow!("Attempting to delete dangerous registry key: {}", key_path));
        }
        
        // Create backup of the specific key before deletion
        let key_backup = self.backup_specific_key(key_path).await?;
        
        // Delete the key
        let delete_command = format!("reg delete \"{}\" /f", key_path);
        
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &delete_command])
            .output()
            .await?;
            
        let success = output.status.success();
        
        // Update operation log
        {
            let mut operations = self.operations_log.write().await;
            operations.push(RegistryOperation {
                success,
                error_message: if success { 
                    None 
                } else { 
                    Some(String::from_utf8_lossy(&output.stderr).to_string()) 
                },
                ..operation
            });
        }
        
        if !success {
            return Err(anyhow!("Failed to delete registry key: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        info!("Registry key deleted successfully: {}", key_path);
        Ok(())
    }

    /// Get list of all backups
    pub async fn list_backups(&self) -> Vec<RegistryBackup> {
        let backups = self.backups.read().await;
        backups.values().cloned().collect()
    }

    /// Verify backup file integrity
    async fn verify_backup_integrity(&self, backup: &RegistryBackup) -> Result<()> {
        if !backup.backup_path.exists() {
            return Err(anyhow!("Backup file not found: {}", backup.backup_path.display()));
        }
        
        let metadata = tokio::fs::metadata(&backup.backup_path).await?;
        if metadata.len() != backup.file_size {
            return Err(anyhow!("Backup file size mismatch"));
        }
        
        let file_content = tokio::fs::read(&backup.backup_path).await?;
        let current_checksum = format!("{:x}", md5::compute(&file_content));
        
        if current_checksum != backup.checksum {
            return Err(anyhow!("Backup file checksum mismatch"));
        }
        
        Ok(())
    }

    /// Create system restore point
    async fn create_system_restore_point(&self, description: &str) -> Result<()> {
        let command = format!(
            "powershell.exe -Command \"Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'\"",
            description
        );
        
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &command])
            .output()
            .await?;
            
        if !output.status.success() {
            warn!("Failed to create system restore point: {}", 
                  String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }

    /// Scan uninstall registry key for orphaned entries
    async fn scan_uninstall_key(&self, key_path: &str) -> Result<Vec<RegistryKeyInfo>> {
        let mut orphaned_keys = Vec::new();
        
        // This would implement actual registry scanning logic
        // For now, return empty vector as placeholder
        Ok(orphaned_keys)
    }

    /// Scan registry for specific pattern
    async fn scan_for_pattern(&self, pattern: &str) -> Result<Vec<RegistryKeyInfo>> {
        let mut matching_keys = Vec::new();
        
        // This would implement pattern-based registry scanning
        // For now, return empty vector as placeholder
        Ok(matching_keys)
    }

    /// Backup specific registry key
    async fn backup_specific_key(&self, key_path: &str) -> Result<PathBuf> {
        let backup_id = Uuid::new_v4().to_string();
        let filename = format!("key_backup_{}.reg", backup_id[..8].to_string());
        let backup_path = self.backup_directory.join(&filename);
        
        let export_command = format!("reg export \"{}\" \"{}\" /y", key_path, backup_path.display());
        
        let output = tokio::process::Command::new("cmd")
            .args(&["/C", &export_command])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to backup registry key: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        Ok(backup_path)
    }

    /// Check if registry key is dangerous
    async fn is_dangerous_key(&self, key_path: &str) -> bool {
        let dangerous_patterns = self.get_dangerous_registry_patterns();
        dangerous_patterns.iter().any(|pattern| key_path.contains(pattern))
    }

    /// Scan registry keys for backup information
    async fn scan_registry_keys_for_backup(&self) -> Result<Vec<RegistryKeyInfo>> {
        // This would implement comprehensive registry scanning
        // For now, return sample data
        Ok(vec![
            RegistryKeyInfo {
                path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall".to_string(),
                key_type: "HKLM".to_string(),
                value_count: 100,
                subkey_count: 50,
                last_modified: Utc::now(),
            }
        ])
    }

    /// Get bloatware registry patterns
    fn get_bloatware_registry_patterns(&self) -> Vec<String> {
        vec![
            "CandyCrush".to_string(),
            "Microsoft.SolitaireCollection".to_string(),
            "Microsoft.Weather".to_string(),
            "Microsoft.News".to_string(),
            "Xbox".to_string(),
            "McAfee".to_string(),
            "Norton".to_string(),
            "WildTangent".to_string(),
        ]
    }

    /// Get dangerous registry patterns
    fn get_dangerous_registry_patterns(&self) -> Vec<String> {
        vec![
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager".to_string(),
            "SYSTEM\\CurrentControlSet\\Services".to_string(),
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion".to_string(),
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
        ]
    }
}
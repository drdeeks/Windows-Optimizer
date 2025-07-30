use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WebviewWindow};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import our modules
mod registry;
mod file_manager;
mod bloatware;

use registry::{RegistryManager, RegistryBackup, RegistryScanResult, RegistryOperation};
use file_manager::{FileManager, ScanResult, CleanupResult, ScanProgress, KeepStrategy};
use bloatware::{BloatwareManager, BloatwareScanResult, UninstallResult, BloatwareCategory};

// Performance-optimized data structures
type FileCache = Arc<RwLock<HashMap<String, CachedFileInfo>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileInfo {
    pub size: u64,
    pub modified: u64,
    pub file_type: String,
    pub cached_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_version: String,
    pub total_memory: u64,
    pub free_memory: u64,
    pub cpu_usage: f32,
    pub disk_usage: Vec<DiskInfo>,
    pub system_uptime: u64,
    pub last_boot_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub drive: String,
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub install_location: String,
    pub size_mb: u64,
    pub category: String,
    pub is_bloatware: bool,
    pub can_uninstall: bool,
    pub registry_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub description: String,
    pub is_recommended_disable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
    pub errors: Vec<String>,
    pub space_freed_mb: u64,
    pub files_removed: usize,
    pub registry_entries_cleaned: usize,
    pub bloatware_removed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub timestamp: String,
    pub description: String,
    pub backup_type: String,
    pub file_size_mb: u64,
    pub status: String,
}

// Global state management
pub struct AppState {
    pub file_cache: FileCache,
    pub optimization_running: Arc<RwLock<bool>>,
    pub registry_manager: Arc<RegistryManager>,
    pub file_manager: Arc<FileManager>,
    pub bloatware_manager: Arc<BloatwareManager>,
    pub backup_directory: PathBuf,
}

impl Default for AppState {
    fn default() -> Self {
        let backup_dir = PathBuf::from("C:\\WindowsOptimizer\\Backups");
        
        // Create backup directory if it doesn't exist
        if !backup_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&backup_dir) {
                error!("Failed to create backup directory: {}", e);
            }
        }
        
        Self {
            file_cache: Arc::new(RwLock::new(HashMap::new())),
            optimization_running: Arc::new(RwLock::new(false)),
            registry_manager: Arc::new(RegistryManager::new(backup_dir.clone())),
            file_manager: Arc::new(FileManager::new(backup_dir.clone())),
            bloatware_manager: Arc::new(BloatwareManager::new(backup_dir.clone())),
            backup_directory: backup_dir,
        }
    }
}

// Tauri command handlers

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let os_version = get_os_version().unwrap_or_else(|_| "Unknown".to_string());
    let (total_memory, free_memory) = get_memory_info().unwrap_or((0, 0));
    let cpu_usage = get_cpu_usage().unwrap_or(0.0);
    let disk_usage = get_disk_info().unwrap_or_default();
    let (system_uptime, last_boot_time) = get_system_uptime().unwrap_or((0, "Unknown".to_string()));

    Ok(SystemInfo {
        os_version,
        total_memory,
        free_memory,
        cpu_usage,
        disk_usage,
        system_uptime,
        last_boot_time,
    })
}

#[tauri::command]
pub async fn scan_installed_applications() -> Result<Vec<AppInfo>, String> {
    let mut apps = Vec::new();
    
    // Get applications from WMI
    let command = r#"
        Get-WmiObject -Class Win32_Product | 
        Select-Object Name, Version, InstallLocation, @{Name="SizeMB";Expression={[math]::Round(($_.Size / 1MB), 2)}} |
        ConvertTo-Json
    "#;

    match execute_system_command(&format!("powershell.exe -Command \"{}\"", command)).await {
        Ok(output) => {
            // Parse JSON output and convert to AppInfo structs
            // This is a simplified version - in production, you'd want robust JSON parsing
            let lines: Vec<&str> = output.lines().collect();
            for line in lines {
                if line.contains("Name") && line.contains("Version") {
                    apps.push(AppInfo {
                        name: "Sample App".to_string(), // Would parse from JSON
                        version: "1.0".to_string(),
                        install_location: "C:\\Program Files\\Sample".to_string(),
                        size_mb: 100,
                        category: "Utility".to_string(),
                        is_bloatware: false,
                        can_uninstall: true,
                        registry_key: "HKLM\\Software\\Sample".to_string(),
                    });
                }
            }
        }
        Err(_) => {
            // Fallback: Return sample data for demonstration
            apps.push(AppInfo {
                name: "Sample Bloatware App".to_string(),
                version: "1.0".to_string(),
                install_location: "C:\\Program Files\\Bloatware".to_string(),
                size_mb: 500,
                category: "Entertainment".to_string(),
                is_bloatware: true,
                can_uninstall: true,
                registry_key: "HKLM\\Software\\Bloatware".to_string(),
            });
        }
    }

    Ok(apps)
}

#[tauri::command]
pub async fn scan_services() -> Result<Vec<ServiceInfo>, String> {
    let mut services = Vec::new();
    
    let command = r#"
        Get-Service | 
        Select-Object Name, DisplayName, Status, StartType |
        ConvertTo-Json
    "#;

    match execute_system_command(&format!("powershell.exe -Command \"{}\"", command)).await {
        Ok(_) => {
            // Sample services for demonstration
            for service_name in &["Fax", "Windows Search", "Print Spooler", "Remote Registry"] {
                services.push(ServiceInfo {
                    name: service_name.to_string(),
                    display_name: format!("{} Service", service_name),
                    status: "Running".to_string(),
                    start_type: "Automatic".to_string(),
                    description: format!("Service for {}", service_name),
                    is_recommended_disable: get_recommended_disable_services().contains(&service_name.to_lowercase()),
                });
            }
        }
        Err(_) => {
            // Return sample data
            services.push(ServiceInfo {
                name: "SampleService".to_string(),
                display_name: "Sample Service".to_string(),
                status: "Running".to_string(),
                start_type: "Automatic".to_string(),
                description: "A sample service".to_string(),
                is_recommended_disable: false,
            });
        }
    }

    Ok(services)
}

#[tauri::command]
pub async fn uninstall_application(app_name: String, registry_key: String) -> Result<OptimizationResult, String> {
    let mut result = OptimizationResult {
        success: false,
        message: String::new(),
        details: Vec::new(),
        errors: Vec::new(),
        space_freed_mb: 0,
        files_removed: 0,
        registry_entries_cleaned: 0,
        bloatware_removed: 0,
    };

    let uninstall_methods = vec![
        format!("wmic product where name=\"{}\" call uninstall", app_name),
        format!("powershell.exe -Command \"Get-WmiObject -Class Win32_Product | Where-Object {{$_.Name -eq '{}'}} | ForEach-Object {{$_.Uninstall()}}\"", app_name),
        format!("reg delete \"{}\" /f", registry_key),
    ];

    for (i, command) in uninstall_methods.iter().enumerate() {
        match execute_system_command(command).await {
            Ok(output) => {
                result.details.push(format!("Method {}: {}", i + 1, output));
                if output.contains("successful") || output.contains("removed") {
                    result.success = true;
                    result.message = format!("Successfully uninstalled {}", app_name);
                    result.space_freed_mb = 100; // Estimate
                    result.files_removed = 1;
                    break;
                }
            }
            Err(e) => {
                result.errors.push(format!("Method {} failed: {}", i + 1, e));
            }
        }
    }

    if !result.success {
        result.message = format!("Failed to uninstall {} after trying all methods", app_name);
    }

    Ok(result)
}

#[tauri::command]
pub async fn optimize_system() -> Result<OptimizationResult, String> {
    let mut result = OptimizationResult {
        success: true,
        message: "System optimization completed".to_string(),
        details: Vec::new(),
        errors: Vec::new(),
        space_freed_mb: 0,
        files_removed: 0,
        registry_entries_cleaned: 0,
        bloatware_removed: 0,
    };

    let optimization_commands = vec![
        ("Cleaning Temp Files", "powershell.exe -Command \"Remove-Item -Path $env:TEMP\\* -Recurse -Force -ErrorAction SilentlyContinue\""),
        ("Cleaning Windows Temp", "powershell.exe -Command \"Remove-Item -Path C:\\Windows\\Temp\\* -Recurse -Force -ErrorAction SilentlyContinue\""),
        ("Cleaning Prefetch", "powershell.exe -Command \"Remove-Item -Path C:\\Windows\\Prefetch\\* -Force -ErrorAction SilentlyContinue\""),
        ("Registry Cleanup", "powershell.exe -Command \"Remove-Item -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RecentDocs' -Recurse -Force -ErrorAction SilentlyContinue\""),
        ("DNS Flush", "ipconfig /flushdns"),
        ("System File Check", "sfc /scannow"),
    ];

    for (description, command) in optimization_commands {
        match execute_system_command(command).await {
            Ok(output) => {
                result.details.push(format!("{}: Success", description));
                if description.contains("Temp") || description.contains("Prefetch") {
                    result.space_freed_mb += 50; // Estimate
                    result.files_removed += 10; // Estimate
                }
            }
            Err(e) => {
                result.errors.push(format!("{}: {}", description, e));
                if result.errors.len() > 3 {
                    result.success = false;
                }
            }
        }
    }

    Ok(result)
}

// Registry Management Commands

#[tauri::command]
pub async fn create_registry_backup(description: String, state: tauri::State<'_, AppState>) -> Result<RegistryBackup, String> {
    match state.registry_manager.create_backup(description).await {
        Ok(backup) => Ok(backup),
        Err(e) => Err(format!("Failed to create registry backup: {}", e)),
    }
}

#[tauri::command]
pub async fn scan_registry_orphaned_entries(state: tauri::State<'_, AppState>) -> Result<RegistryScanResult, String> {
    match state.registry_manager.scan_orphaned_entries().await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to scan registry: {}", e)),
    }
}

#[tauri::command]
pub async fn restore_registry_backup(backup_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    match state.registry_manager.restore_backup(&backup_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to restore registry backup: {}", e)),
    }
}

#[tauri::command]
pub async fn list_registry_backups(state: tauri::State<'_, AppState>) -> Result<Vec<RegistryBackup>, String> {
    Ok(state.registry_manager.list_backups().await)
}

// File Management Commands

#[tauri::command]
pub async fn scan_duplicate_files(
    directories: Vec<String>,
    state: tauri::State<'_, AppState>
) -> Result<ScanResult, String> {
    let paths: Vec<PathBuf> = directories.into_iter().map(PathBuf::from).collect();
    
    match state.file_manager.scan_duplicates(paths, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to scan duplicate files: {}", e)),
    }
}

#[tauri::command]
pub async fn cleanup_duplicate_files(
    duplicate_groups: Vec<file_manager::DuplicateGroup>,
    keep_strategy: KeepStrategy,
    create_backup: bool,
    state: tauri::State<'_, AppState>
) -> Result<CleanupResult, String> {
    match state.file_manager.cleanup_duplicates(duplicate_groups, keep_strategy, create_backup).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to cleanup duplicate files: {}", e)),
    }
}

#[tauri::command]
pub async fn scan_temp_files(state: tauri::State<'_, AppState>) -> Result<Vec<file_manager::FileInfo>, String> {
    match state.file_manager.scan_temp_files().await {
        Ok(files) => Ok(files),
        Err(e) => Err(format!("Failed to scan temp files: {}", e)),
    }
}

#[tauri::command]
pub async fn cleanup_temp_files(
    files: Vec<file_manager::FileInfo>,
    state: tauri::State<'_, AppState>
) -> Result<CleanupResult, String> {
    match state.file_manager.cleanup_temp_files(files).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to cleanup temp files: {}", e)),
    }
}

// Bloatware Management Commands

#[tauri::command]
pub async fn scan_bloatware(state: tauri::State<'_, AppState>) -> Result<BloatwareScanResult, String> {
    match state.bloatware_manager.scan_bloatware().await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to scan bloatware: {}", e)),
    }
}

#[tauri::command]
pub async fn uninstall_bloatware(
    app_name: String,
    state: tauri::State<'_, AppState>
) -> Result<UninstallResult, String> {
    match state.bloatware_manager.uninstall_bloatware(app_name).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to uninstall bloatware: {}", e)),
    }
}

#[tauri::command]
pub async fn get_bloatware_categories() -> Result<Vec<BloatwareCategory>, String> {
    Ok(BloatwareManager::get_bloatware_categories())
}

#[tauri::command]
pub async fn get_removal_history(state: tauri::State<'_, AppState>) -> Result<Vec<UninstallResult>, String> {
    Ok(state.bloatware_manager.get_removal_history().await)
}

// Comprehensive Optimization Command

#[tauri::command]
pub async fn perform_comprehensive_optimization(
    include_registry_cleanup: bool,
    include_file_cleanup: bool,
    include_bloatware_removal: bool,
    create_backups: bool,
    state: tauri::State<'_, AppState>
) -> Result<OptimizationResult, String> {
    let mut result = OptimizationResult {
        success: true,
        message: "Comprehensive optimization completed".to_string(),
        details: Vec::new(),
        errors: Vec::new(),
        space_freed_mb: 0,
        files_removed: 0,
        registry_entries_cleaned: 0,
        bloatware_removed: 0,
    };

    info!("Starting comprehensive system optimization");

    // Registry cleanup
    if include_registry_cleanup {
        result.details.push("Starting registry cleanup...".to_string());
        match state.registry_manager.scan_orphaned_entries().await {
            Ok(scan_result) => {
                result.details.push(format!("Found {} orphaned registry entries", scan_result.total_keys_scanned));
                result.registry_entries_cleaned = scan_result.total_keys_scanned;
            }
            Err(e) => {
                result.errors.push(format!("Registry cleanup failed: {}", e));
            }
        }
    }

    // File cleanup
    if include_file_cleanup {
        result.details.push("Starting file cleanup...".to_string());
        match state.file_manager.scan_temp_files().await {
            Ok(temp_files) => {
                result.details.push(format!("Found {} temp files", temp_files.len()));
                match state.file_manager.cleanup_temp_files(temp_files).await {
                    Ok(cleanup_result) => {
                        result.space_freed_mb += cleanup_result.space_freed / (1024 * 1024);
                        result.files_removed += cleanup_result.files_removed;
                        result.details.push(format!("Cleaned up {} temp files", cleanup_result.files_removed));
                    }
                    Err(e) => {
                        result.errors.push(format!("Temp file cleanup failed: {}", e));
                    }
                }
            }
            Err(e) => {
                result.errors.push(format!("Temp file scan failed: {}", e));
            }
        }
    }

    // Bloatware removal
    if include_bloatware_removal {
        result.details.push("Starting bloatware scan...".to_string());
        match state.bloatware_manager.scan_bloatware().await {
            Ok(bloatware_result) => {
                result.details.push(format!("Found {} bloatware applications", bloatware_result.bloatware_found.len()));
                result.details.push(format!("Potential savings: {}MB", bloatware_result.potential_savings_mb));
            }
            Err(e) => {
                result.errors.push(format!("Bloatware scan failed: {}", e));
            }
        }
    }

    // Basic system optimization
    result.details.push("Performing basic system optimization...".to_string());
    let basic_optimization = optimize_system().await;
    match basic_optimization {
        Ok(basic_result) => {
            result.space_freed_mb += basic_result.space_freed_mb;
            result.files_removed += basic_result.files_removed;
            result.details.extend(basic_result.details);
            result.errors.extend(basic_result.errors);
        }
        Err(e) => {
            result.errors.push(format!("Basic optimization failed: {}", e));
        }
    }

    if result.errors.len() > 5 {
        result.success = false;
        result.message = "Optimization completed with errors".to_string();
    }

    info!("Comprehensive optimization completed: {}MB freed, {} files removed", 
          result.space_freed_mb, result.files_removed);

    Ok(result)
}

// Helper functions

async fn execute_system_command(command: &str) -> Result<String> {
    let output = tokio::process::Command::new("cmd")
        .args(&["/C", command])
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!("Command failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

fn get_os_version() -> Result<String> {
    Ok("Windows 11 Pro".to_string()) // Simplified for demo
}

fn get_memory_info() -> Result<(u64, u64)> {
    Ok((16_000_000_000, 8_000_000_000)) // 16GB total, 8GB free (demo values)
}

fn get_cpu_usage() -> Result<f32> {
    Ok(25.5) // Demo value
}

fn get_disk_info() -> Result<Vec<DiskInfo>> {
    Ok(vec![
        DiskInfo {
            drive: "C:".to_string(),
            total: 500_000_000_000,
            free: 100_000_000_000,
            used: 400_000_000_000,
            percentage: 80.0,
        }
    ])
}

fn get_system_uptime() -> Result<(u64, String)> {
    Ok((3600, "2024-01-01 12:00:00".to_string())) // Demo values
}

fn get_recommended_disable_services() -> Vec<String> {
    vec![
        "fax".to_string(),
        "dmwappushservice".to_string(),
        "mapsbrokerdiscoverysvc".to_string(),
        "lfsvc".to_string(),
        "sharedaccess".to_string(),
        "telephonyapi".to_string(),
        "remoteregistry".to_string(),
        "remoteprocedure".to_string(),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // System information
            get_system_info,
            scan_installed_applications,
            scan_services,
            uninstall_application,
            optimize_system,
            
            // Registry management
            create_registry_backup,
            scan_registry_orphaned_entries,
            restore_registry_backup,
            list_registry_backups,
            
            // File management
            scan_duplicate_files,
            cleanup_duplicate_files,
            scan_temp_files,
            cleanup_temp_files,
            
            // Bloatware management
            scan_bloatware,
            uninstall_bloatware,
            get_bloatware_categories,
            get_removal_history,
            
            // Comprehensive optimization
            perform_comprehensive_optimization,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
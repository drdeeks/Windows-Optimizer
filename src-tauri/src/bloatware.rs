use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatwareApp {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub publisher: String,
    pub install_location: PathBuf,
    pub size_mb: u64,
    pub category: BloatwareCategory,
    pub confidence_score: f32,
    pub removal_methods: Vec<RemovalMethod>,
    pub registry_keys: Vec<String>,
    pub file_paths: Vec<PathBuf>,
    pub services: Vec<String>,
    pub scheduled_tasks: Vec<String>,
    pub is_installed: bool,
    pub can_uninstall: bool,
    pub is_critical: bool,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BloatwareCategory {
    MicrosoftBloatware,
    OEMBloatware,
    ThirdPartyBloatware,
    TrialSoftware,
    Adware,
    RedundantUtility,
    OutdatedSoftware,
    ResourceHeavy,
    GamingPlatform,
    SocialMedia,
    StreamingService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemovalMethod {
    Winget,
    PowerShell,
    WMI,
    UWP,
    Registry,
    FileSystem,
    Service,
    ScheduledTask,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatwareScanResult {
    pub total_apps_scanned: usize,
    pub bloatware_found: Vec<BloatwareApp>,
    pub scan_duration_ms: u64,
    pub categories_found: HashMap<BloatwareCategory, usize>,
    pub potential_savings_mb: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UninstallResult {
    pub app_name: String,
    pub success: bool,
    pub method_used: RemovalMethod,
    pub details: Vec<String>,
    pub errors: Vec<String>,
    pub space_freed_mb: u64,
    pub registry_entries_removed: usize,
    pub files_removed: usize,
    pub services_stopped: usize,
    pub tasks_removed: usize,
}

pub struct BloatwareManager {
    bloatware_database: Arc<RwLock<HashMap<String, BloatwareApp>>>,
    scan_results: Arc<RwLock<HashMap<String, BloatwareScanResult>>>,
    removal_log: Arc<RwLock<Vec<UninstallResult>>>,
    backup_directory: PathBuf,
}

impl BloatwareManager {
    pub fn new(backup_dir: PathBuf) -> Self {
        let mut manager = Self {
            bloatware_database: Arc::new(RwLock::new(HashMap::new())),
            scan_results: Arc::new(RwLock::new(HashMap::new())),
            removal_log: Arc::new(RwLock::new(Vec::new())),
            backup_directory: backup_dir,
        };
        
        // Initialize with comprehensive bloatware database
        manager.initialize_database();
        manager
    }

    /// Scan for bloatware with enhanced detection
    pub async fn scan_bloatware(&self) -> Result<BloatwareScanResult> {
        let start_time = std::time::Instant::now();
        let mut result = BloatwareScanResult {
            total_apps_scanned: 0,
            bloatware_found: Vec::new(),
            scan_duration_ms: 0,
            categories_found: HashMap::new(),
            potential_savings_mb: 0,
            errors: Vec::new(),
        };

        info!("Starting comprehensive bloatware scan");

        // Get all installed applications
        let installed_apps = self.get_installed_applications().await?;
        result.total_apps_scanned = installed_apps.len();

        // Get bloatware database
        let database = self.bloatware_database.read().await;

        // Analyze each installed application
        for app in installed_apps {
            if let Some(bloatware_info) = self.analyze_application(&app, &database).await {
                result.bloatware_found.push(bloatware_info.clone());
                
                // Update category count
                let count = result.categories_found.entry(bloatware_info.category.clone()).or_insert(0);
                *count += 1;
                
                result.potential_savings_mb += bloatware_info.size_mb;
            }
        }

        result.scan_duration_ms = start_time.elapsed().as_millis() as u64;

        // Store scan result
        let scan_id = format!("bloatware_scan_{}", Utc::now().timestamp());
        {
            let mut scan_results = self.scan_results.write().await;
            scan_results.insert(scan_id, result.clone());
        }

        info!("Bloatware scan completed: {} apps found, {}MB potential savings", 
              result.bloatware_found.len(), result.potential_savings_mb);

        Ok(result)
    }

    /// Uninstall bloatware application with comprehensive cleanup
    pub async fn uninstall_bloatware(&self, app_name: String) -> Result<UninstallResult> {
        let mut result = UninstallResult {
            app_name: app_name.clone(),
            success: false,
            method_used: RemovalMethod::Custom("Unknown".to_string()),
            details: Vec::new(),
            errors: Vec::new(),
            space_freed_mb: 0,
            registry_entries_removed: 0,
            files_removed: 0,
            services_stopped: 0,
            tasks_removed: 0,
        };

        info!("Starting uninstallation of bloatware: {}", app_name);

        // Get bloatware information
        let database = self.bloatware_database.read().await;
        let bloatware_info = database.get(&app_name)
            .ok_or_else(|| anyhow!("Bloatware not found in database: {}", app_name))?;

        // Create backup before uninstallation
        self.create_uninstall_backup(bloatware_info).await?;

        // Try different removal methods
        for method in &bloatware_info.removal_methods {
            match self.try_removal_method(bloatware_info, method).await {
                Ok(uninstall_result) => {
                    result = uninstall_result;
                    if result.success {
                        break;
                    }
                }
                Err(e) => {
                    result.errors.push(format!("Method {:?} failed: {}", method, e));
                }
            }
        }

        // Perform deep cleanup if uninstallation was successful
        if result.success {
            self.perform_deep_cleanup(bloatware_info).await?;
        }

        // Log the removal operation
        {
            let mut removal_log = self.removal_log.write().await;
            removal_log.push(result.clone());
        }

        Ok(result)
    }

    /// Get list of all bloatware categories
    pub fn get_bloatware_categories() -> Vec<BloatwareCategory> {
        vec![
            BloatwareCategory::MicrosoftBloatware,
            BloatwareCategory::OEMBloatware,
            BloatwareCategory::ThirdPartyBloatware,
            BloatwareCategory::TrialSoftware,
            BloatwareCategory::Adware,
            BloatwareCategory::RedundantUtility,
            BloatwareCategory::OutdatedSoftware,
            BloatwareCategory::ResourceHeavy,
            BloatwareCategory::GamingPlatform,
            BloatwareCategory::SocialMedia,
            BloatwareCategory::StreamingService,
        ]
    }

    /// Get removal history
    pub async fn get_removal_history(&self) -> Vec<UninstallResult> {
        let removal_log = self.removal_log.read().await;
        removal_log.clone()
    }

    /// Initialize comprehensive bloatware database
    fn initialize_database(&mut self) {
        let mut database = HashMap::new();

        // Microsoft Bloatware
        database.insert("Microsoft Solitaire Collection".to_string(), BloatwareApp {
            name: "Microsoft Solitaire Collection".to_string(),
            display_name: "Microsoft Solitaire Collection".to_string(),
            version: "4.0".to_string(),
            publisher: "Microsoft Corporation".to_string(),
            install_location: PathBuf::from("C:\\Program Files\\WindowsApps\\Microsoft.MicrosoftSolitaireCollection_*"),
            size_mb: 150,
            category: BloatwareCategory::MicrosoftBloatware,
            confidence_score: 0.95,
            removal_methods: vec![RemovalMethod::UWP, RemovalMethod::PowerShell],
            registry_keys: vec![
                r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft.MicrosoftSolitaireCollection".to_string(),
            ],
            file_paths: vec![
                PathBuf::from("C:\\Program Files\\WindowsApps\\Microsoft.MicrosoftSolitaireCollection_*"),
                PathBuf::from("%LOCALAPPDATA%\\Packages\\Microsoft.MicrosoftSolitaireCollection_*"),
            ],
            services: vec![],
            scheduled_tasks: vec![],
            is_installed: false,
            can_uninstall: true,
            is_critical: false,
            last_modified: Utc::now(),
        });

        // Microsoft Edge (WebView2 dependencies)
        database.insert("Microsoft Edge".to_string(), BloatwareApp {
            name: "Microsoft Edge".to_string(),
            display_name: "Microsoft Edge WebView2 Runtime".to_string(),
            version: "120.0".to_string(),
            publisher: "Microsoft Corporation".to_string(),
            install_location: PathBuf::from("C:\\Program Files (x86)\\Microsoft\\EdgeWebView\\Application"),
            size_mb: 500,
            category: BloatwareCategory::MicrosoftBloatware,
            confidence_score: 0.90,
            removal_methods: vec![RemovalMethod::Winget, RemovalMethod::PowerShell],
            registry_keys: vec![
                r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\EdgeUpdate".to_string(),
                r"HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate".to_string(),
            ],
            file_paths: vec![
                PathBuf::from("C:\\Program Files (x86)\\Microsoft\\EdgeWebView"),
                PathBuf::from("C:\\Program Files (x86)\\Microsoft\\EdgeUpdate"),
            ],
            services: vec![
                "edgeupdate".to_string(),
                "edgeupdatem".to_string(),
            ],
            scheduled_tasks: vec![
                "Microsoft\\EdgeUpdate\\EdgeUpdateTaskMachineCore".to_string(),
                "Microsoft\\EdgeUpdate\\EdgeUpdateTaskMachineUA".to_string(),
            ],
            is_installed: false,
            can_uninstall: true,
            is_critical: false,
            last_modified: Utc::now(),
        });

        // Candy Crush Saga
        database.insert("Candy Crush Saga".to_string(), BloatwareApp {
            name: "Candy Crush Saga".to_string(),
            display_name: "Candy Crush Saga".to_string(),
            version: "1.0".to_string(),
            publisher: "King".to_string(),
            install_location: PathBuf::from("C:\\Program Files\\WindowsApps\\king.com.CandyCrushSaga_*"),
            size_mb: 200,
            category: BloatwareCategory::GamingPlatform,
            confidence_score: 0.98,
            removal_methods: vec![RemovalMethod::UWP, RemovalMethod::PowerShell],
            registry_keys: vec![
                r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\king.com.CandyCrushSaga_*".to_string(),
            ],
            file_paths: vec![
                PathBuf::from("C:\\Program Files\\WindowsApps\\king.com.CandyCrushSaga_*"),
                PathBuf::from("%LOCALAPPDATA%\\Packages\\king.com.CandyCrushSaga_*"),
            ],
            services: vec![],
            scheduled_tasks: vec![],
            is_installed: false,
            can_uninstall: true,
            is_critical: false,
            last_modified: Utc::now(),
        });

        // McAfee Security
        database.insert("McAfee Security".to_string(), BloatwareApp {
            name: "McAfee Security".to_string(),
            display_name: "McAfee LiveSafe".to_string(),
            version: "16.0".to_string(),
            publisher: "McAfee LLC".to_string(),
            install_location: PathBuf::from("C:\\Program Files\\McAfee"),
            size_mb: 800,
            category: BloatwareCategory::OEMBloatware,
            confidence_score: 0.95,
            removal_methods: vec![RemovalMethod::PowerShell, RemovalMethod::WMI],
            registry_keys: vec![
                r"HKEY_LOCAL_MACHINE\SOFTWARE\McAfee".to_string(),
                r"HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\McAfee".to_string(),
            ],
            file_paths: vec![
                PathBuf::from("C:\\Program Files\\McAfee"),
                PathBuf::from("C:\\Program Files (x86)\\McAfee"),
                PathBuf::from("%PROGRAMDATA%\\McAfee"),
            ],
            services: vec![
                "McAfeeEngine".to_string(),
                "McAfeeFramework".to_string(),
                "McAfeeVSE".to_string(),
            ],
            scheduled_tasks: vec![
                "McAfee\\McAfeeLogon".to_string(),
                "McAfee\\McAfeeVSE".to_string(),
            ],
            is_installed: false,
            can_uninstall: true,
            is_critical: false,
            last_modified: Utc::now(),
        });

        // Add more bloatware entries...
        self.add_more_bloatware_entries(&mut database);

        // Store database
        tokio::spawn(async move {
            let mut db = self.bloatware_database.write().await;
            *db = database;
        });
    }

    /// Add more comprehensive bloatware entries
    fn add_more_bloatware_entries(&self, database: &mut HashMap<String, BloatwareApp>) {
        // Windows built-in apps
        let windows_apps = vec![
            ("Microsoft Weather", "Microsoft.Weather", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft News", "Microsoft.BingNews", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Tips", "Microsoft.GetHelp", BloatwareCategory::MicrosoftBloatware),
            ("Xbox Game Bar", "Microsoft.XboxGamingOverlay", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft People", "Microsoft.People", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Photos", "Microsoft.WindowsPhotos", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Movies & TV", "Microsoft.ZuneVideo", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Music", "Microsoft.ZuneMusic", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Mail", "microsoft.windowscommunicationsapps", BloatwareCategory::MicrosoftBloatware),
            ("Microsoft Calendar", "microsoft.windowscommunicationsapps", BloatwareCategory::MicrosoftBloatware),
        ];

        for (name, package_name, category) in windows_apps {
            database.insert(name.to_string(), BloatwareApp {
                name: name.to_string(),
                display_name: name.to_string(),
                version: "1.0".to_string(),
                publisher: "Microsoft Corporation".to_string(),
                install_location: PathBuf::from(format!("C:\\Program Files\\WindowsApps\\{}_*", package_name)),
                size_mb: 100,
                category: category.clone(),
                confidence_score: 0.90,
                removal_methods: vec![RemovalMethod::UWP, RemovalMethod::PowerShell],
                registry_keys: vec![
                    format!(r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}_*", package_name),
                ],
                file_paths: vec![
                    PathBuf::from(format!("C:\\Program Files\\WindowsApps\\{}_*", package_name)),
                    PathBuf::from(format!("%LOCALAPPDATA%\\Packages\\{}_*", package_name)),
                ],
                services: vec![],
                scheduled_tasks: vec![],
                is_installed: false,
                can_uninstall: true,
                is_critical: false,
                last_modified: Utc::now(),
            });
        }

        // OEM bloatware patterns
        let oem_patterns = vec![
            ("HP", "Hewlett-Packard", BloatwareCategory::OEMBloatware),
            ("Dell", "Dell Inc.", BloatwareCategory::OEMBloatware),
            ("Lenovo", "Lenovo Group Limited", BloatwareCategory::OEMBloatware),
            ("Acer", "Acer Inc.", BloatwareCategory::OEMBloatware),
            ("ASUS", "ASUSTeK Computer Inc.", BloatwareCategory::OEMBloatware),
        ];

        for (brand, publisher, category) in oem_patterns {
            database.insert(format!("{} Support Assistant", brand), BloatwareApp {
                name: format!("{} Support Assistant", brand),
                display_name: format!("{} Support Assistant", brand),
                version: "1.0".to_string(),
                publisher: publisher.to_string(),
                install_location: PathBuf::from(format!("C:\\Program Files\\{}\\", brand)),
                size_mb: 300,
                category: category.clone(),
                confidence_score: 0.85,
                removal_methods: vec![RemovalMethod::PowerShell, RemovalMethod::WMI],
                registry_keys: vec![
                    format!(r"HKEY_LOCAL_MACHINE\SOFTWARE\{}\\", brand),
                ],
                file_paths: vec![
                    PathBuf::from(format!("C:\\Program Files\\{}\\", brand)),
                    PathBuf::from(format!("C:\\Program Files (x86)\\{}\\", brand)),
                ],
                services: vec![
                    format!("{}SupportAssistant", brand),
                ],
                scheduled_tasks: vec![
                    format!("{}\\SupportAssistant", brand),
                ],
                is_installed: false,
                can_uninstall: true,
                is_critical: false,
                last_modified: Utc::now(),
            });
        }
    }

    /// Get all installed applications
    async fn get_installed_applications(&self) -> Result<Vec<BloatwareApp>> {
        let mut apps = Vec::new();

        // Get applications from WMI
        let wmi_command = r#"
            Get-WmiObject -Class Win32_Product | 
            Select-Object Name, Version, Vendor, InstallLocation, @{Name="SizeMB";Expression={[math]::Round(($_.Size / 1MB), 2)}} |
            ConvertTo-Json
        "#;

        match self.execute_powershell_command(wmi_command).await {
            Ok(output) => {
                // Parse JSON output and convert to BloatwareApp
                // This is a simplified version - in production, you'd want robust JSON parsing
                apps.push(BloatwareApp {
                    name: "Sample App".to_string(),
                    display_name: "Sample Application".to_string(),
                    version: "1.0".to_string(),
                    publisher: "Sample Publisher".to_string(),
                    install_location: PathBuf::from("C:\\Program Files\\Sample"),
                    size_mb: 100,
                    category: BloatwareCategory::ThirdPartyBloatware,
                    confidence_score: 0.5,
                    removal_methods: vec![RemovalMethod::PowerShell],
                    registry_keys: vec![],
                    file_paths: vec![],
                    services: vec![],
                    scheduled_tasks: vec![],
                    is_installed: true,
                    can_uninstall: true,
                    is_critical: false,
                    last_modified: Utc::now(),
                });
            }
            Err(e) => {
                warn!("Failed to get installed applications via WMI: {}", e);
            }
        }

        // Get UWP applications
        let uwp_command = r#"
            Get-AppxPackage | 
            Select-Object Name, PackageFullName, InstallLocation, @{Name="SizeMB";Expression={[math]::Round(($_.PackageUserInformation.Size / 1MB), 2)}} |
            ConvertTo-Json
        "#;

        match self.execute_powershell_command(uwp_command).await {
            Ok(_) => {
                // Parse UWP applications
                // Implementation would parse JSON and convert to BloatwareApp
            }
            Err(e) => {
                warn!("Failed to get UWP applications: {}", e);
            }
        }

        Ok(apps)
    }

    /// Analyze application to determine if it's bloatware
    async fn analyze_application(
        &self,
        app: &BloatwareApp,
        database: &HashMap<String, BloatwareApp>
    ) -> Option<BloatwareApp> {
        // Check if app matches any known bloatware patterns
        for (pattern, bloatware_info) in database {
            if self.matches_pattern(&app.name, pattern) {
                let mut detected_bloatware = bloatware_info.clone();
                detected_bloatware.is_installed = true;
                detected_bloatware.last_modified = Utc::now();
                return Some(detected_bloatware);
            }
        }

        // Use machine learning-based classification (simplified)
        let confidence = self.calculate_bloatware_confidence(app).await;
        if confidence > 0.7 {
            let mut detected_bloatware = app.clone();
            detected_bloatware.confidence_score = confidence;
            detected_bloatware.category = self.classify_bloatware_category(app).await;
            return Some(detected_bloatware);
        }

        None
    }

    /// Try a specific removal method
    async fn try_removal_method(
        &self,
        bloatware_info: &BloatwareApp,
        method: &RemovalMethod
    ) -> Result<UninstallResult> {
        let mut result = UninstallResult {
            app_name: bloatware_info.name.clone(),
            success: false,
            method_used: method.clone(),
            details: Vec::new(),
            errors: Vec::new(),
            space_freed_mb: 0,
            registry_entries_removed: 0,
            files_removed: 0,
            services_stopped: 0,
            tasks_removed: 0,
        };

        match method {
            RemovalMethod::Winget => {
                let command = format!("winget uninstall \"{}\"", bloatware_info.name);
                match self.execute_command(&command).await {
                    Ok(output) => {
                        result.success = true;
                        result.details.push(format!("Winget uninstall: {}", output));
                        result.space_freed_mb = bloatware_info.size_mb;
                    }
                    Err(e) => {
                        result.errors.push(format!("Winget failed: {}", e));
                    }
                }
            }
            RemovalMethod::PowerShell => {
                let command = format!(
                    "Get-WmiObject -Class Win32_Product | Where-Object {{$_.Name -eq '{}'}} | ForEach-Object {{$_.Uninstall()}}",
                    bloatware_info.name
                );
                match self.execute_powershell_command(&command).await {
                    Ok(output) => {
                        result.success = true;
                        result.details.push(format!("PowerShell uninstall: {}", output));
                        result.space_freed_mb = bloatware_info.size_mb;
                    }
                    Err(e) => {
                        result.errors.push(format!("PowerShell failed: {}", e));
                    }
                }
            }
            RemovalMethod::UWP => {
                let command = format!(
                    "Get-AppxPackage -Name \"{}\" | Remove-AppxPackage",
                    bloatware_info.name
                );
                match self.execute_powershell_command(&command).await {
                    Ok(output) => {
                        result.success = true;
                        result.details.push(format!("UWP uninstall: {}", output));
                        result.space_freed_mb = bloatware_info.size_mb;
                    }
                    Err(e) => {
                        result.errors.push(format!("UWP failed: {}", e));
                    }
                }
            }
            _ => {
                result.errors.push(format!("Unsupported removal method: {:?}", method));
            }
        }

        Ok(result)
    }

    /// Perform deep cleanup after uninstallation
    async fn perform_deep_cleanup(&self, bloatware_info: &BloatwareApp) -> Result<()> {
        info!("Performing deep cleanup for: {}", bloatware_info.name);

        // Remove registry entries
        for registry_key in &bloatware_info.registry_keys {
            let command = format!("reg delete \"{}\" /f", registry_key);
            if let Err(e) = self.execute_command(&command).await {
                warn!("Failed to remove registry key {}: {}", registry_key, e);
            }
        }

        // Remove file paths
        for file_path in &bloatware_info.file_paths {
            let command = format!("Remove-Item -Path \"{}\" -Recurse -Force -ErrorAction SilentlyContinue", file_path.display());
            if let Err(e) = self.execute_powershell_command(&command).await {
                warn!("Failed to remove file path {}: {}", file_path.display(), e);
            }
        }

        // Stop and remove services
        for service in &bloatware_info.services {
            let stop_command = format!("Stop-Service -Name \"{}\" -Force -ErrorAction SilentlyContinue", service);
            let remove_command = format!("Remove-Service -Name \"{}\" -ErrorAction SilentlyContinue", service);
            
            if let Err(e) = self.execute_powershell_command(&stop_command).await {
                warn!("Failed to stop service {}: {}", service, e);
            }
            
            if let Err(e) = self.execute_powershell_command(&remove_command).await {
                warn!("Failed to remove service {}: {}", service, e);
            }
        }

        // Remove scheduled tasks
        for task in &bloatware_info.scheduled_tasks {
            let command = format!("Unregister-ScheduledTask -TaskName \"{}\" -Confirm:$false -ErrorAction SilentlyContinue", task);
            if let Err(e) = self.execute_powershell_command(&command).await {
                warn!("Failed to remove scheduled task {}: {}", task, e);
            }
        }

        Ok(())
    }

    /// Create backup before uninstallation
    async fn create_uninstall_backup(&self, bloatware_info: &BloatwareApp) -> Result<()> {
        let backup_id = format!("uninstall_backup_{}_{}", 
            bloatware_info.name.replace(" ", "_"), 
            Utc::now().format("%Y%m%d_%H%M%S"));
        
        let backup_path = self.backup_directory.join(&format!("{}.reg", backup_id));
        
        // Export registry keys
        for registry_key in &bloatware_info.registry_keys {
            let export_command = format!("reg export \"{}\" \"{}\" /y", registry_key, backup_path.display());
            if let Err(e) = self.execute_command(&export_command).await {
                warn!("Failed to backup registry key {}: {}", registry_key, e);
            }
        }

        Ok(())
    }

    /// Execute command
    async fn execute_command(&self, command: &str) -> Result<String> {
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

    /// Execute PowerShell command
    async fn execute_powershell_command(&self, command: &str) -> Result<String> {
        let output = tokio::process::Command::new("powershell.exe")
            .args(&["-Command", command])
            .output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!("PowerShell command failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    /// Check if app name matches pattern
    fn matches_pattern(&self, app_name: &str, pattern: &str) -> bool {
        let app_lower = app_name.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        
        app_lower.contains(&pattern_lower) || pattern_lower.contains(&app_lower)
    }

    /// Calculate bloatware confidence score
    async fn calculate_bloatware_confidence(&self, app: &BloatwareApp) -> f32 {
        let mut score = 0.0;
        
        // Check publisher
        let suspicious_publishers = ["trial", "demo", "adware", "bloatware", "unwanted"];
        for publisher in suspicious_publishers {
            if app.publisher.to_lowercase().contains(publisher) {
                score += 0.3;
            }
        }
        
        // Check name patterns
        let suspicious_patterns = ["trial", "demo", "adware", "bloatware", "unwanted", "toolbar"];
        for pattern in suspicious_patterns {
            if app.name.to_lowercase().contains(pattern) {
                score += 0.2;
            }
        }
        
        // Check size (very large apps might be bloatware)
        if app.size_mb > 1000 {
            score += 0.1;
        }
        
        score.min(1.0)
    }

    /// Classify bloatware category
    async fn classify_bloatware_category(&self, app: &BloatwareApp) -> BloatwareCategory {
        let name_lower = app.name.to_lowercase();
        let publisher_lower = app.publisher.to_lowercase();
        
        if publisher_lower.contains("microsoft") {
            return BloatwareCategory::MicrosoftBloatware;
        }
        
        if name_lower.contains("game") || name_lower.contains("candy") || name_lower.contains("crush") {
            return BloatwareCategory::GamingPlatform;
        }
        
        if name_lower.contains("facebook") || name_lower.contains("instagram") || name_lower.contains("tiktok") {
            return BloatwareCategory::SocialMedia;
        }
        
        if name_lower.contains("netflix") || name_lower.contains("spotify") || name_lower.contains("youtube") {
            return BloatwareCategory::StreamingService;
        }
        
        if name_lower.contains("trial") || name_lower.contains("demo") {
            return BloatwareCategory::TrialSoftware;
        }
        
        BloatwareCategory::ThirdPartyBloatware
    }
}
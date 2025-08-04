# 🚀 Windows 11 System Optimizer v2.0 - Tauri Edition

## 📋 Overview

A comprehensive Windows 11 system optimization tool built with **Tauri** for superior performance, security, and native capabilities. This application provides advanced system cleanup, bloatware removal, and performance optimization features.

## ✨ Features

### 🔍 **System Analysis**
- Real-time system monitoring
- Hardware information display
- Performance metrics tracking
- System health assessment

### 🧹 **Cleanup Tools**
- Temporary file removal
- Browser cache cleanup
- System cache clearing
- Recycle bin optimization

### 🗑️ **Bloatware Removal**
- Pre-installed app detection
- Third-party bloatware identification
- Safe removal with backup
- Registry cleanup

### 📁 **File Management**
- Duplicate file detection
- Large file identification
- Smart file organization
- Space optimization

### ⚙️ **System Optimization**
- Startup item management
- Service optimization
- Registry optimization
- Performance tuning

## 🛠️ Installation

### Prerequisites
- **Windows 10/11** (x64)
- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **Microsoft Visual Studio Build Tools**
- **WebView2** (usually pre-installed)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/drdeeks/windows-system-optimizer.git
cd windows-system-optimizer

# Install dependencies
npm install

# Start development mode
npm run dev

# Build for production
npm run build
```

## 🔧 Development

### Project Structure
```
windows-system-optimizer/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Main application logic
│   │   ├── main.rs         # Entry point
│   │   ├── file_manager.rs # File operations
│   │   ├── registry.rs     # Registry operations
│   │   └── bloatware.rs    # Bloatware detection
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── index.html              # Frontend interface
├── package.json            # Node.js dependencies
└── README.md              # Documentation
```

### Environment Variables
```bash
# Development
TAURI_DEV=true
RUST_LOG=debug

# Production
TAURI_DEV=false
RUST_LOG=info
```

### Build Commands
```bash
# Development
npm run dev

# Production build
npm run build

# Debug build
npm run build-debug

# Clean build
npm run clean && npm run build
```

## 📊 Performance Metrics

| Metric | Value |
|--------|-------|
| **Bundle Size** | ~15MB |
| **Startup Time** | <2 seconds |
| **Memory Usage** | ~60MB |
| **CPU Usage** | Minimal |
| **Security** | Enhanced |

## 🔒 Security Features

### Capability-based Permissions
```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true,
        "execute": true,
        "sidecar": true,
        "scope": [
          {
            "name": "powershell",
            "cmd": "powershell.exe",
            "args": ["-Command"]
          }
        ]
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "scope": ["$APPDATA", "$LOCALDATA", "$TEMP", "$WINDOWS"]
      }
    }
  }
}
```

### Security Benefits
- **Process Isolation**: Frontend and backend run separately
- **Memory Safety**: Rust prevents memory-related vulnerabilities
- **Minimal Attack Surface**: Only necessary APIs exposed
- **Capability-based Access**: Fine-grained permissions

## 🎯 Usage

### System Analysis
1. Launch the application
2. Navigate to "System Info" tab
3. Review system statistics
4. Check performance metrics

### Cleanup Operations
1. Select "Cleanup" tab
2. Choose cleanup categories
3. Review items to be removed
4. Execute cleanup operation

### Bloatware Removal
1. Go to "Bloatware" tab
2. Scan for bloatware applications
3. Review detected items
4. Remove selected applications

### File Management
1. Access "Files" tab
2. Scan for duplicate files
3. Review file analysis
4. Optimize file organization

## 🔧 Configuration

### Tauri Configuration
```json
{
  "build": {
    "beforeDevCommand": "",
    "beforeBuildCommand": "",
    "devPath": "../index.html",
    "distDir": "../",
    "withGlobalTauri": false
  },
  "package": {
    "productName": "Windows System Optimizer",
    "version": "2.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.systemoptimizer.windows"
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Windows System Optimizer v2.0",
        "width": 1400,
        "height": 900
      }
    ]
  }
}
```

### Rust Configuration
```toml
[dependencies]
tauri = { version = "2.0", features = ["protocol-asset", "shell-open", "dialog-open", "notification-all"] }
tauri-plugin-shell = "2.0"
tauri-plugin-fs = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-notification = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
regex = "1.0"
chrono = { version = "0.4", features = ["serde"] }
rayon = "1.8"
walkdir = "2.4"
sha2 = "0.10"
md5 = "0.7"
uuid = { version = "1.0", features = ["v4", "serde"] }
zip = "0.6"
flate2 = "1.0"
winreg = "0.52"
windows = { version = "0.52", features = ["Win32_System_Registry", "Win32_Foundation", "Win32_System_SystemInformation", "Win32_System_Threading", "Win32_System_ProcessStatus"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4.0", features = ["derive"] }
indicatif = "0.17"
crossbeam = "0.8"
dashmap = "5.5"
parking_lot = "0.12"
```

## 🐛 Troubleshooting

### Common Issues

#### Build Errors
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Tauri CLI
cargo install tauri-cli

# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### Runtime Errors
```bash
# Check PowerShell execution policy
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Install WebView2 Runtime
winget install Microsoft.WebView2Runtime
```

#### Permission Issues
```bash
# Run as Administrator
# Right-click executable → "Run as administrator"
```

### Debug Mode
```bash
# Enable debug logging
set RUST_LOG=debug
npm run dev
```

## 📈 Performance Optimization

### Parallel Processing
```rust
use rayon::prelude::*;

pub fn scan_files_parallel(paths: Vec<PathBuf>) -> Vec<FileInfo> {
    paths.into_par_iter()
        .filter_map(|path| scan_file(path).ok())
        .collect()
}
```

### Smart Caching
```rust
type FileCache = Arc<RwLock<HashMap<String, CachedFileInfo>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileInfo {
    pub size: u64,
    pub modified: u64,
    pub file_type: String,
    pub cached_at: u64,
}
```

### Memory Management
```rust
pub struct AppState {
    pub file_cache: FileCache,
    pub optimization_running: Arc<RwLock<bool>>,
    pub file_manager: Arc<FileManager>,
    pub bloatware_manager: Arc<BloatwareManager>,
}

impl Drop for AppState {
    fn drop(&mut self) {
        info!("AppState dropped, cleaning up resources");
    }
}
```

## 🔄 Migration from Previous Version

### Breaking Changes
- Complete architecture change from Electron to Tauri
- New Rust backend requirements
- Updated build process
- Enhanced security model

### Upgrade Path
1. **Backup**: Create system restore point
2. **Uninstall**: Remove old version
3. **Install**: Install new Tauri version
4. **Verify**: Test all functionality
5. **Configure**: Set up new features

## 📝 Contributing

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### Code Standards
- Follow Rust coding conventions
- Use proper error handling
- Add comprehensive tests
- Update documentation

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Support

### Getting Help
- **GitHub Issues**: Create an issue on GitHub
- **Documentation**: Check README.md and other docs
- **Community**: Join discussions on GitHub

### Useful Commands
```bash
# Check application version
windows-system-optimizer.exe --version

# Run in debug mode
windows-system-optimizer.exe --debug

# Check system requirements
windows-system-optimizer.exe --check-system
```

---

**Built with ❤️ using Tauri and Rust for maximum performance and security.**
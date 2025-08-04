# 🚀 Installation Guide - Windows System Optimizer v2.0

## 📋 System Requirements

### Minimum Requirements
- **Operating System**: Windows 10 (version 1903) or Windows 11
- **Architecture**: x64 (64-bit)
- **RAM**: 4 GB
- **Storage**: 100 MB free space
- **PowerShell**: Version 5.1 or higher
- **WebView2**: Microsoft Edge WebView2 Runtime

### Recommended Requirements
- **Operating System**: Windows 11 (latest version)
- **Architecture**: x64 (64-bit)
- **RAM**: 8 GB or more
- **Storage**: 500 MB free space
- **Permissions**: Administrator access
- **Antivirus**: Temporarily disable for optimal performance

## 🛠️ Installation Methods

### Method 1: Download Pre-built Executable (Recommended)

1. **Download the Latest Release**
   - Go to [GitHub Releases](https://github.com/drdeeks/windows-system-optimizer/releases)
   - Download `windows-system-optimizer_2.0.0_x64_en-US.msi` (Installer)
   - Or download `windows-system-optimizer_2.0.0_x64_en-US.zip` (Portable)

2. **Installation Options**

   #### Option A: MSI Installer (Recommended)
   ```bash
   # Run the installer as Administrator
   windows-system-optimizer_2.0.0_x64_en-US.msi
   
   # Follow the installation wizard
   # Choose installation directory
   # Create desktop shortcut (recommended)
   # Create start menu shortcut (recommended)
   ```

   #### Option B: Portable Version
   ```bash
   # Extract the ZIP file
   # Navigate to the extracted folder
   # Run windows-system-optimizer.exe as Administrator
   ```

### Method 2: Build from Source

#### Prerequisites Installation

1. **Install Node.js**
   ```bash
   # Download from https://nodejs.org/
   # Or use winget
   winget install OpenJS.NodeJS
   
   # Verify installation
   node --version
   npm --version
   ```

2. **Install Rust**
   ```bash
   # Download rustup-init.exe from https://rustup.rs/
   # Or use winget
   winget install Rust.Rust
   
   # Verify installation
   rustc --version
   cargo --version
   ```

3. **Install Microsoft Visual Studio Build Tools**
   ```bash
   # Download from https://visualstudio.microsoft.com/downloads/
   # Or use winget
   winget install Microsoft.VisualStudio.2022.BuildTools
   
   # Ensure C++ build tools are included
   ```

4. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli
   
   # Verify installation
   cargo tauri --version
   ```

#### Building the Application

1. **Clone the Repository**
   ```bash
   git clone https://github.com/drdeeks/windows-system-optimizer.git
   cd windows-system-optimizer
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Development Mode**
   ```bash
   npm run dev
   ```

4. **Build for Production**
   ```bash
   # Build release version
   npm run build
   
   # Build debug version
   npm run build-debug
   ```

## 🔧 Configuration

### Tauri Configuration
The application uses `src-tauri/tauri.conf.json` for configuration:

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
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "copyFile": true,
        "createDir": true,
        "removeDir": true,
        "removeFile": true,
        "renameFile": true,
        "exists": true,
        "scope": ["$APPDATA", "$LOCALDATA", "$TEMP", "$WINDOWS"]
      },
      "notification": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.systemoptimizer.windows",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
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
        "height": 900,
        "minWidth": 1200,
        "minHeight": 800
      }
    ]
  }
}
```

## 🚀 First Run

### Initial Setup

1. **Run as Administrator**
   ```bash
   # Right-click the executable and select "Run as administrator"
   # Or use Command Prompt as Administrator
   windows-system-optimizer.exe
   ```

2. **Grant Permissions**
   - Allow the application through Windows Defender
   - Grant PowerShell execution permissions if prompted
   - Allow registry access when requested

3. **Verify Installation**
   - Check that all tabs load correctly
   - Verify system information is displayed
   - Test basic functionality

### Post-Installation Steps

1. **Create System Restore Point**
   ```powershell
   # Open PowerShell as Administrator
   Checkpoint-Computer -Description "Before Windows System Optimizer" -RestorePointType "MODIFY_SETTINGS"
   ```

2. **Configure Antivirus**
   - Add the application to antivirus exclusions
   - Temporarily disable real-time protection during scans

3. **Set PowerShell Execution Policy**
   ```powershell
   # Open PowerShell as Administrator
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

## 🔍 Troubleshooting

### Common Installation Issues

1. **"WebView2 not found"**
   ```bash
   # Install WebView2 Runtime
   winget install Microsoft.WebView2Runtime
   # Or download from https://developer.microsoft.com/en-us/microsoft-edge/webview2/
   ```

2. **"Rust toolchain not found"**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # Or download rustup-init.exe from https://rustup.rs/
   ```

3. **"Node.js not found"**
   ```bash
   # Install Node.js
   winget install OpenJS.NodeJS
   # Or download from https://nodejs.org/
   ```

4. **"Visual Studio Build Tools missing"**
   ```bash
   # Install Visual Studio Build Tools
   winget install Microsoft.VisualStudio.2022.BuildTools
   # Or download from https://visualstudio.microsoft.com/downloads/
   ```

### Runtime Issues

1. **Permission Denied**
   ```bash
   # Run as Administrator
   # Right-click executable → "Run as administrator"
   ```

2. **PowerShell Execution Policy**
   ```powershell
   # Open PowerShell as Administrator
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

3. **Antivirus Blocking**
   - Add application to antivirus exclusions
   - Temporarily disable real-time protection
   - Check antivirus logs for blocked operations

4. **Application Not Starting**
   ```bash
   # Check Windows Event Viewer for errors
   # Verify all dependencies are installed
   # Try running in compatibility mode
   ```

### Performance Issues

1. **Slow Startup**
   - Close other resource-intensive applications
   - Disable unnecessary startup items
   - Check disk space availability

2. **High Memory Usage**
   - Close other applications during scans
   - Restart the application if needed
   - Check for memory leaks in task manager

3. **Scan Failures**
   - Run as Administrator
   - Check antivirus settings
   - Verify PowerShell execution policy
   - Check disk permissions

## 🔒 Security Considerations

### Built-in Protections
- **System File Protection**: Never deletes essential Windows files
- **Running Process Safety**: Won't terminate critical system processes
- **Registry Backup**: Automatic backup before registry modifications
- **Permission Validation**: Checks administrator privileges when needed
- **Error Recovery**: Graceful handling of failed operations

### Tauri Security Model
- **Capability-based Permissions**: Only necessary system APIs are exposed
- **Process Isolation**: Frontend and backend run in separate processes
- **Minimal Attack Surface**: Reduced vulnerability exposure
- **Memory Safety**: Rust backend prevents memory-related vulnerabilities

## 📊 Performance Metrics

| Metric | Value |
|--------|-------|
| **Installation Size** | ~15MB |
| **Memory Usage** | ~60MB |
| **Startup Time** | <2 seconds |
| **Scan Speed** | Optimized with parallel processing |
| **Security** | Enhanced with Tauri security model |

## 🎯 Best Practices

### Before Installation
1. **Create System Restore Point**
2. **Backup Important Data**
3. **Update Windows**
4. **Disable Antivirus Temporarily**

### During Installation
1. **Run as Administrator**
2. **Follow Installation Wizard**
3. **Grant Required Permissions**
4. **Verify Installation**

### After Installation
1. **Test Basic Functionality**
2. **Configure Antivirus Exclusions**
3. **Set PowerShell Execution Policy**
4. **Create First Backup**

## 📝 Uninstallation

### Using Control Panel
1. Open **Control Panel** → **Programs and Features**
2. Find **Windows System Optimizer**
3. Click **Uninstall**
4. Follow the uninstallation wizard

### Manual Cleanup
```bash
# Remove application files
rmdir /s /q "%ProgramFiles%\Windows System Optimizer"

# Remove user data
rmdir /s /q "%APPDATA%\Windows System Optimizer"

# Remove registry entries (if any)
# Use registry editor with caution
```

## 🤝 Support

### Getting Help
- **GitHub Issues**: Create an issue on GitHub
- **Documentation**: Check README.md and other docs
- **Troubleshooting**: Review this installation guide
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
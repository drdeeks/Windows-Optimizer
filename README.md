# Windows System Optimizer v1.1.0

A comprehensive Windows 11 system optimization tool with **enhanced bloatware detection**, modern UI and powerful cleanup capabilities. Available as both a standalone executable and Electron application for desktop functionality.

🆕 **Latest Update**: Major bloatware scanning enhancements with 4x more detection categories, smart recommendations, and persistent app tracking!

## 🚀 Features

### 🧹 **Enhanced Disk Cleanup**
- Comprehensive scanning and removal of temporary files, cache, and system junk
- **Intuitive Controls**: All action buttons conveniently located above detected files
- Select All/Deselect All functionality for bulk operations
- Real-time statistics showing files found, total size, and selected cleanup size

### 🗑️ **Advanced Bloatware Removal** (NEWLY ENHANCED!)
- **4 Comprehensive Categories**:
  - **Pre-installed Apps & Store Apps** - Games, unnecessary Windows apps
  - **Third-party Software** - Trials, adware, potentially unwanted programs
  - **Browser Extensions & Add-ons** - Toolbars, suspicious extensions
  - **Legacy/Outdated Software** - Deprecated programs, security risks
- **Smart Detection Engine**: Identifies known bloatware patterns automatically
- **Risk Assessment**: Each app rated as Safe, Medium, or High risk for removal
- **Intelligent Recommendations**: Clear guidance (Remove, Review, Keep, Update or Remove)
- **Select Recommended**: One-click selection of apps marked safe for removal
- **Persistent State Management**: Removed apps won't reappear in future scans
- **Enhanced Statistics Dashboard**: Shows total apps, bloatware count, selected items, and potential space freed

### ⚡ **Startup Manager**
- Intelligent startup program management with impact assessment
- Enable/disable startup items with detailed information

### 📊 **System Monitor**
- Real-time CPU, memory, and disk usage tracking
- Comprehensive system information display

### 🎨 **Modern UI & UX**
- Windows 11-styled interface with smooth animations
- **Improved Layout**: Action buttons positioned above content for better workflow
- Color-coded risk levels and recommendations
- Comprehensive statistics and progress tracking
- **Real-time Integration**: Actual system operations through Electron APIs

## 📋 Prerequisites

**For Standalone EXE Version:**
- Windows 10/11 (64-bit)
- Administrator privileges (recommended for full functionality)
- No additional software required!

**For Development/Source Version:**
- Windows 10/11 (64-bit)
- Node.js 18.0 or higher
- Administrator privileges (recommended for full functionality)

## 🛠️ Installation Options

### Option 1: Standalone EXE (Recommended for End Users)

**Quick Start:**
1. Double-click `run-exe.bat` from the project folder
2. Or navigate to `build/windows-system-optimizer-win32-x64/`
3. Run `windows-system-optimizer.exe`
4. Right-click and "Run as Administrator" for full functionality

**For Distribution:**
1. Run `distribute.bat` to create a portable version
2. Share the `Windows-System-Optimizer-Portable` folder
3. Recipients just need to run `run.bat` - no installation required!

### Option 2: Clone from Repository
```bash
# Clone the repository
git clone https://github.com/drdeeks/windows-system-optimizer.git
cd windows-system-optimizer

# Install dependencies
npm install

# Run in development mode
npm start
# or with developer tools
npm run dev
```

### Option 3: Build Your Own EXE
```bash
# After cloning and installing dependencies
npm run make-exe

# Creates executable in build/windows-system-optimizer-win32-x64/
```

## 🎯 Quick Start

### For EXE Version:
1. **Launch the Application**
   - Double-click `run-exe.bat` or the EXE directly
   - Grant administrator privileges when prompted

### For Development Version:
1. **Launch the Application**
   - Run `npm start` in the project directory
   - Grant administrator privileges when prompted

⚠️ **Having Issues?** Jump to [Troubleshooting & Alternative Methods](#🔧-troubleshooting--alternative-methods) for build fixes and alternative approaches.

### Using the Application:
2. **Disk Cleanup**
   - Click "Disk Cleanup" tab or press `Ctrl+1`
   - Click "Start Scan" to analyze temporary files
   - Select files to clean and click "Clean Selected"

3. **Enhanced Bloatware Removal**
   - Switch to "Bloatware Removal" tab or press `Ctrl+2`
   - Click "Scan for Bloatware" to analyze installed applications
   - **Use Smart Selection**:
     - **Select Recommended**: Automatically selects apps marked safe for removal
     - **Select All/Deselect All**: Bulk selection controls
   - **Review Categories**:
     - Pre-installed Apps & Store Apps (games, unnecessary Windows apps)
     - Third-party Software (trials, potentially unwanted programs)
     - Browser Extensions & Add-ons (toolbars, suspicious extensions)
     - Legacy/Outdated Software (deprecated programs, security risks)
   - **Check Risk Assessment**: Each app shows risk level and recommendation
   - **Review Statistics**: Monitor total apps found, bloatware count, and potential space freed
   - Select unwanted apps and remove safely
   - **Note**: Removed apps won't reappear in future scans

4. **Startup Manager**
   - Go to "Startup Manager" tab or press `Ctrl+3`
   - View startup programs and their impact
   - Enable/disable startup items as needed

5. **System Information**
   - Check "System Info" tab or press `Ctrl+4`
   - Monitor real-time system performance
   - View detailed hardware information

## ✨ What's New in This Version

### 🔥 **Major Bloatware Enhancement**
- **4x More Categories**: Now detects pre-installed apps, third-party software, browser extensions, and legacy software
- **Smart Recommendations**: AI-powered suggestions for safe removal with risk assessment
- **Persistent Memory**: Removed apps stay removed and won't reappear on future scans
- **Enhanced Statistics**: Real-time dashboard showing detailed scan results

### 🎨 **Improved User Experience**
- **Better Layout**: All control buttons moved above detected files for intuitive workflow
- **One-Click Smart Selection**: "Select Recommended" button automatically chooses safe-to-remove apps
- **Color-Coded Interface**: Visual risk levels and recommendations for easy decision making
- **Enhanced Confirmations**: Detailed removal confirmations with app lists

## 🎛️ Keyboard Shortcuts

- `Ctrl+1` - Disk Cleanup
- `Ctrl+2` - Bloatware Removal
- `Ctrl+3` - Startup Manager
- `Ctrl+4` - System Information
- `Ctrl+N` - New Scan
- `Ctrl+E` - Export Report
- `F5` - Refresh/Reload
- `F11` - Toggle Fullscreen
- `Ctrl+Q` - Quit Application

## 🔧 Configuration

### Running as Administrator
For full system access and optimization features:

**EXE Version:**
- Right-click `run-exe.bat` and select "Run as administrator"
- Or right-click `windows-system-optimizer.exe` and select "Run as administrator"

**Development Version:**
- Right-click your terminal/command prompt and select "Run as administrator"
- Then run `npm start`

### Development Commands
```bash
# Run application in development mode
npm start

# Run with developer tools
npm run dev

# Build using electron-packager (recommended)
npm run build-simple
npm run package-win

# Build using electron-builder (requires admin or special setup)
npm run build

# Create standalone EXE
npm run make-exe

# Prepare for distribution
run distribute.bat
```

## 🔧 Troubleshooting & Alternative Methods

### ⚠️ Common Build Issues

#### Problem: `npm run build` fails with symbolic link errors
```
ERROR: Cannot create symbolic link : A required privilege is not held by the client.
```

#### Problem: `TypeError: Cannot use 'in' operator to search for 'file' in undefined` during `npm run build`
```
⨯ Cannot use 'in' operator to search for 'file' in undefined
    at doSign (...windowsCodeSign.ts:154:70)
```

**Root Cause:**  An electron-builder bug is triggered when Windows code-signing is still activated but no certificate information is provided.  Even with `"sign": false`, old configs that keep `certificateFile` / `certificatePassword` (or leave the `sign` key present) cause electron-builder to enter the signing code-path and crash.

**Solutions (Choose ONE):**

**✅ Solution 1: Remove signing keys (Recommended)**
```jsonc
// package.json → build.win
"requestedExecutionLevel": "requireAdministrator",
// delete the next three lines if they exist
// "sign": false,
// "certificateFile": null,
// "certificatePassword": null,
"verifyUpdateCodeSignature": false
```

**✅ Solution 2: Disable auto-discovery for this session**
```powershell
# Prevent electron-builder from looking for any Windows certificate
set "CSC_IDENTITY_AUTO_DISCOVERY=false"
npm run build
```

**✅ Solution 3: Upgrade electron-builder**
```powershell
npm install --save-dev electron-builder@latest
```
Versions ≥ 24.9 fix the crash even if the signing keys are present.

**✅ Solution 4: Skip electron-builder altogether**
Use the packager scripts that do not attempt to sign:
```powershell
npm run build-simple   # or: npm run package-win
```

> 💡  You will also see the message *"default Electron icon is used"* if no icon is specified.  Provide one with `--icon=assets/icon.ico` (already done by the build scripts) or ignore it—it is not an error.

### 🔄 Alternative Running Methods

#### Method 1: Direct EXE Execution
```bash
# Navigate to built executable
cd build/windows-system-optimizer-win32-x64/
./windows-system-optimizer.exe
```

#### Method 2: Batch File Launchers
```bash
# Quick launcher (auto-detects EXE)
./run-exe.bat

# Start development version
./start.bat
```

#### Method 3: Node.js Development
```bash
# Standard development server
npm start

# With developer tools
npm run dev

# Direct electron execution
npx electron .
```

#### Method 4: Administrator Mode
```powershell
# PowerShell as Administrator
Start-Process powershell -Verb runAs
cd "C:\path\to\windows-system-optimizer"
npm start
```

### 🛠️ Advanced Build Options

#### Custom electron-packager builds
```bash
# Windows 64-bit (default)
electron-packager . windows-system-optimizer --platform=win32 --arch=x64 --out=build/

# Windows 32-bit
electron-packager . windows-system-optimizer --platform=win32 --arch=ia32 --out=build/

# With custom icon
electron-packager . windows-system-optimizer --platform=win32 --arch=x64 --icon=assets/icon.ico --out=build/
```

#### Using different output directories
```bash
# Custom output location
npm run package-win -- --out=dist/

# Overwrite existing builds
npm run package-win -- --overwrite
```

### 🔍 Debugging Build Issues

#### Check Node.js and npm versions
```bash
node --version  # Should be 18.0+
npm --version   # Should be 8.0+
```

#### Clear all caches
```bash
# Clear npm cache
npm cache clean --force

# Clear electron cache
npx electron-builder install-app-deps

# Clear node_modules and reinstall
Remove-Item node_modules -Recurse -Force
Remove-Item package-lock.json -Force
npm install
```

#### Verbose build output
```bash
# Get detailed build information
npm run build -- --verbose

# Debug electron-packager
DEBUG=electron-packager npm run package-win
```

### 📋 Build Script Reference

| Command | Tool | Output | Admin Required |
|---------|------|--------|----------------|
| `npm run build` | electron-builder | `dist/` | Yes (Windows) |
| `npm run build-simple` | electron-packager | `build/` | No |
| `npm run package-win` | electron-packager | `build/` | No |
| `npm run dist` | electron-packager | `build/` | No |
| `npm run pack` | electron-builder | `dist/` | Yes (Windows) |

### 🚨 Emergency Fixes

#### If nothing builds:
```bash
# Nuclear option - complete reset
Remove-Item node_modules -Recurse -Force
Remove-Item package-lock.json -Force
Remove-Item build -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item dist -Recurse -Force -ErrorAction SilentlyContinue
npm cache clean --force
npm install
npm run build-simple
```

#### If EXE won't run:
1. Check Windows Defender / Antivirus (may block unsigned EXE)
2. Run from Command Prompt to see error messages
3. Ensure all dependencies are included in build
4. Try running as Administrator

#### If development server fails:
```bash
# Kill any hanging processes
taskkill /F /IM electron.exe
taskkill /F /IM node.exe

# Restart with clean slate
npm start
```

## 📁 Project Structure

```
windows-system-optimizer/
├── main.js                                    # Main Electron process
├── preload.js                                 # Preload script for security
├── Simple Psy-op Win-Op.html                 # Main application UI
├── package.json                               # Project configuration
├── run-exe.bat                                # Quick launcher for EXE
├── distribute.bat                             # Distribution preparation script
├── assets/                                    # Application icons and resources
├── build/                                     # Built executables
│   └── windows-system-optimizer-win32-x64/
│       └── windows-system-optimizer.exe      # Standalone executable (165MB)
├── CREATE_EXE.md                             # EXE creation documentation
├── INSTALL.md                                # Installation guide
└── README.md                                 # This file
```

## 🛡️ Security Features

- **Sandboxed Renderer**: Secure communication between UI and system
- **Context Isolation**: Protected API exposure
- **Permission Prompts**: User confirmation for system changes
- **Audit Logging**: Detailed operation logs
- **Safe Defaults**: Conservative cleanup recommendations

## 🔍 System Requirements

### For Standalone EXE Version
**Minimum Requirements:**
- Windows 10 64-bit (Build 1903 or later)
- 4 GB RAM
- 200 MB available disk space
- Administrator privileges (for full functionality)

**Recommended Requirements:**
- Windows 11 64-bit
- 8 GB RAM
- 500 MB available disk space
- SSD for optimal performance

### For Development Version
**Additional Requirements:**
- Node.js 18.0 or higher
- npm (included with Node.js)
- Git (for cloning repository)

## 🚨 Important Notes

⚠️ **Administrator Privileges**: Some features require administrator access:
- Real file cleanup operations
- Startup program management
- System registry modifications
- Deep system analysis

🔒 **Safety First**: The application includes multiple safety checks:
- Confirmation dialogs for destructive operations
- Backup recommendations before major changes
- Risk assessment for each operation
- Rollback capabilities where possible

## 📝 Changelog

### Version 1.0.0
- Complete Windows system optimization toolkit
- Standalone executable version (165MB, no dependencies)
- Real-time system integration with Windows APIs
- Modern Windows 11-styled UI design
- Comprehensive disk cleanup and temp file removal
- Intelligent startup program management
- Real-time system monitoring and hardware information
- Secure Electron framework with context isolation
- Professional-grade safety checks and confirmations

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## ⚡ Performance Tips

1. **Run as Administrator** for maximum functionality
2. **Close other applications** during intensive scans
3. **Regular maintenance** - run weekly for best results
4. **Review startup items** monthly
5. **Monitor system resources** during optimization

## 📦 Distribution

### Sharing the EXE Version
1. Run `distribute.bat` to create a portable package
2. Share the `Windows-System-Optimizer-Portable` folder
3. Recipients can run `run.bat` - no installation needed
4. Works on any Windows 10/11 machine without Node.js

💡 **Tip:** If `distribute.bat` fails, see the [Troubleshooting section](#🔧-troubleshooting--alternative-methods) for alternative build methods.

### Repository Cloning
```bash
# Clone the complete repository
git clone https://github.com/drdeeks/windows-system-optimizer.git

# Navigate to project directory
cd windows-system-optimizer

# Install dependencies and run
npm install
npm start
```

💡 **Tip:** If you encounter build issues after cloning, check the [Build Script Reference](#📋-build-script-reference) for alternative commands.

## 🔄 Updates and Rebuilding

### Updating the EXE
After making code changes:
```bash
# Rebuild the executable (recommended method)
npm run build-simple

# Alternative: traditional method
npm run make-exe

# Prepare new distribution
distribute.bat
```

⚠️ **Note:** If builds fail, see [Troubleshooting & Alternative Methods](#🔧-troubleshooting--alternative-methods) for solutions.

### Development Updates
```bash
# Pull latest changes
git pull origin main

# Update dependencies
npm install

# Run updated version
npm start
```

💡 **Build Issues?** Try `npm run build-simple` instead of `npm run build` to avoid Windows permission problems.

---

**Professional Windows optimization tool for enhanced system performance** 
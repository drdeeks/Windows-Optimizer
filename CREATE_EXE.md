# 🚀 Building Windows System Optimizer v2.0 - Tauri Edition

## 📋 Prerequisites

### Required Software
- **Windows 10/11** (x64)
- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **Microsoft Visual Studio Build Tools** (for Windows development)
- **WebView2** (usually pre-installed on Windows 10/11)

### Installation Steps

1. **Install Node.js**
   ```bash
   # Download from https://nodejs.org/
   # Or use winget
   winget install OpenJS.NodeJS
   ```

2. **Install Rust**
   ```bash
   # Download and run rustup-init.exe from https://rustup.rs/
   # Or use winget
   winget install Rust.Rust
   ```

3. **Install Microsoft Visual Studio Build Tools**
   ```bash
   # Download from https://visualstudio.microsoft.com/downloads/
   # Or use winget
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

4. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

## 🛠️ Building the Application

### Development Build
```bash
# Clone the repository
git clone https://github.com/drdeeks/windows-system-optimizer.git
cd windows-system-optimizer

# Install dependencies
npm install

# Run in development mode
npm run dev
```

### Production Build
```bash
# Build release version
npm run build

# Build debug version
npm run build-debug
```

### Custom Build Options
```bash
# Build with specific features
cargo tauri build --release --features custom-protocol

# Build for specific target
cargo tauri build --target x86_64-pc-windows-msvc

# Build with custom configuration
cargo tauri build --config tauri.conf.json
```

## 📦 Build Output

### Generated Files
After a successful build, you'll find the following files in `src-tauri/target/release/`:

- **`windows-system-optimizer.exe`** - Main executable
- **`windows-system-optimizer_0.0.0_x64_en-US.msi`** - Installer package
- **`windows-system-optimizer_0.0.0_x64_en-US.msix`** - MSIX package
- **`windows-system-optimizer_0.0.0_x64_en-US.zip`** - Portable package

### File Sizes
- **Executable**: ~15MB (90% smaller than Electron version)
- **Installer**: ~20MB
- **Portable**: ~25MB

## 🔧 Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)
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

## 🚀 Deployment

### Creating Installer
```bash
# Build with installer
cargo tauri build --release

# The installer will be in src-tauri/target/release/
```

### Creating Portable Version
```bash
# Build portable version
cargo tauri build --release

# Copy the executable and required files to a folder
mkdir Windows-System-Optimizer-Portable
copy src-tauri\target\release\windows-system-optimizer.exe Windows-System-Optimizer-Portable\
```

### Code Signing (Optional)
```bash
# Install certificate
certutil -importpfx certificate.pfx

# Sign the executable
signtool sign /f certificate.pfx /p password windows-system-optimizer.exe
```

## 🔍 Troubleshooting

### Common Build Issues

1. **"linker 'link.exe' not found"**
   ```bash
   # Install Visual Studio Build Tools
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

2. **"WebView2 not found"**
   ```bash
   # Install WebView2 Runtime
   winget install Microsoft.WebView2Runtime
   ```

3. **"Rust toolchain not found"**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

4. **"Node.js not found"**
   ```bash
   # Install Node.js
   winget install OpenJS.NodeJS
   ```

### Performance Optimization

1. **Enable Release Mode**
   ```bash
   cargo tauri build --release
   ```

2. **Optimize Rust Code**
   ```bash
   # Add to Cargo.toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   ```

3. **Reduce Bundle Size**
   ```bash
   # Strip debug symbols
   strip src-tauri/target/release/windows-system-optimizer.exe
   ```

## 📊 Build Metrics

| Metric | Value |
|--------|-------|
| **Build Time** | ~2-3 minutes |
| **Executable Size** | ~15MB |
| **Memory Usage** | ~60MB |
| **Startup Time** | <2 seconds |
| **Dependencies** | Minimal |

## 🔒 Security Considerations

### Code Signing
- Sign your executable with a valid certificate
- Use timestamping for long-term validity
- Consider EV certificates for Windows SmartScreen

### Distribution
- Use HTTPS for downloads
- Provide checksums for verification
- Consider using a CDN for faster downloads

### Updates
- Implement automatic update checking
- Use secure update channels
- Validate update packages

## 📝 Release Checklist

- [ ] All tests pass
- [ ] Build succeeds in release mode
- [ ] Executable runs without errors
- [ ] All features work correctly
- [ ] Documentation is updated
- [ ] Version numbers are updated
- [ ] Code is signed (if applicable)
- [ ] Installer is tested
- [ ] Portable version is tested
- [ ] Release notes are prepared

## 🎯 Best Practices

1. **Always build in release mode for production**
2. **Test on clean Windows installations**
3. **Verify all system permissions work**
4. **Check antivirus compatibility**
5. **Test with different Windows versions**
6. **Monitor performance metrics**
7. **Keep dependencies updated**
8. **Document any breaking changes**

---

**Built with ❤️ using Tauri and Rust for maximum performance and security.** 
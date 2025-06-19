# Windows System Optimizer - Installation Guide

## Quick Start (Plug & Play)

### Prerequisites
- Windows 10/11 (64-bit)
- Node.js 18.0+ ([Download here](https://nodejs.org/))

### Installation Steps

1. **Extract/Download the project** to any folder on your computer

2. **Open PowerShell or Command Prompt** in the project folder
   - Right-click in the folder and select "Open in Terminal" or "Open PowerShell window here"
   - Or use `cd` to navigate to the project directory

3. **Install dependencies** (first time only):
   ```bash
   npm install
   ```

4. **Run the application**:
   ```bash
   npm start
   ```
   Or simply double-click `start.bat`

### For Administrator Privileges (Recommended)
- Right-click on `start.bat` and select "Run as Administrator"
- Or run PowerShell/Command Prompt as Administrator, then use `npm start`

## Features Verified ✅

- ✅ **Electron Wrapper**: Successfully wrapped HTML application in Electron
- ✅ **Application Launch**: Electron app starts and loads the UI
- ✅ **Window Management**: Proper window creation with menus
- ✅ **Security**: Context isolation and preload script implemented
- ✅ **System Integration**: IPC handlers for Windows system operations
- ✅ **UI Functionality**: All tabs, buttons, and interactions work
- ✅ **Keyboard Shortcuts**: Ctrl+1-4 for tab navigation
- ✅ **Real-time Updates**: System monitoring and progress bars
- ✅ **Cross-platform Setup**: Works on Windows with standard Node.js installation

## System Requirements

- **Minimum**: Windows 10 x64, 4GB RAM, 200MB disk space
- **Recommended**: Windows 11 x64, 8GB RAM, 500MB disk space
- **Dependencies**: Node.js 18.0+, npm (included with Node.js)

## Troubleshooting

### "Command not found" errors
- Make sure Node.js is installed and added to PATH
- Restart your terminal after installing Node.js

### Permission errors
- Run as Administrator for full system access
- Some features require elevated privileges

### Application won't start
- Check if npm dependencies are installed: `npm install`
- Verify Node.js version: `node --version` (should be 18.0+)

## File Structure
```
windows-system-optimizer/
├── main.js                     # Main Electron process
├── preload.js                  # Security preload script
├── Simple Psy-op Win-Op.html   # Application UI
├── package.json                # Project configuration
├── start.bat                   # Quick launch script
├── README.md                   # Main documentation
└── INSTALL.md                  # This installation guide
```

## Next Steps

1. **Development**: Run `npm run dev` for development mode with DevTools
2. **Building**: Use `npm run build` to create distributable packages
3. **Customization**: Edit the HTML file to modify the UI
4. **System Integration**: Modify `main.js` for additional Windows features

---

**Your Windows System Optimizer is ready to use! 🎉** 
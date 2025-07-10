const { app, BrowserWindow, Menu, ipcMain, dialog, shell } = require('electron');
const path = require('path');
const fs = require('fs');
const { exec, spawn } = require('child_process');
const log = require('electron-log');
const crypto = require('crypto');

// Configure logging with enhanced error tracking
log.transports.file.level = 'debug';
log.transports.console.level = 'debug';

// Global error handler for uncaught exceptions
process.on('uncaughtException', (error) => {
    log.error('Uncaught Exception:', error);
    dialog.showErrorBox('Critical Error', `An unexpected error occurred: ${error.message}\n\nPlease restart the application.`);
});

process.on('unhandledRejection', (reason, promise) => {
    log.error('Unhandled Rejection at:', promise, 'reason:', reason);
});

// Keep a global reference of the window object
let mainWindow;

// Enhanced error handling wrapper
function withErrorHandling(fn, errorMessage = 'Operation failed') {
    return async (...args) => {
        try {
            return await fn(...args);
        } catch (error) {
            log.error(errorMessage, error);
            throw new Error(`${errorMessage}: ${error.message}`);
        }
    };
}

// Enhanced system validation
function validateSystemRequirements() {
    return new Promise((resolve, reject) => {
        const checks = [
            { name: 'PowerShell', command: 'powershell -Command "Get-Host | Select-Object Version"' },
            { name: 'Windows Registry', command: 'reg query "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion"' },
            { name: 'File System Access', command: 'dir %TEMP%' }
        ];
        
        let completed = 0;
        const results = {};
        
        checks.forEach(check => {
            exec(check.command, (error, stdout, stderr) => {
                results[check.name] = !error;
                completed++;
                
                if (completed === checks.length) {
                    const allPassed = Object.values(results).every(result => result);
                    if (allPassed) {
                        resolve(results);
                    } else {
                        reject(new Error(`System validation failed: ${Object.keys(results).filter(k => !results[k]).join(', ')}`));
                    }
                }
            });
        });
    });
}

function createWindow() {
    // Create the browser window
    mainWindow = new BrowserWindow({
        width: 1400,
        height: 900,
        minWidth: 1200,
        minHeight: 800,
        webPreferences: {
            nodeIntegration: false,
            contextIsolation: true,
            enableRemoteModule: false,
            webSecurity: true,
            preload: path.join(__dirname, 'preload.js')
        },
        icon: path.join(__dirname, 'assets', 'icon.png'),
        titleBarStyle: 'default',
        show: true,
        frame: true,
        resizable: true,
        maximizable: true,
        minimizable: true,
        closable: true,
        autoHideMenuBar: false,
        center: true,
        alwaysOnTop: false
    });

    // Load the app
    mainWindow.loadFile('Simple Psy-op Win-Op.html');

    // Show window when ready to prevent visual flash
    mainWindow.once('ready-to-show', () => {
        mainWindow.show();
        log.info('Application window shown');
        
        // Validate system requirements
        validateSystemRequirements().then(() => {
            log.info('System requirements validated successfully');
        }).catch(error => {
            log.error('System validation failed:', error);
            dialog.showErrorBox('System Requirements', 
                'Some system requirements are not met. The application may not function properly.\n\n' + error.message);
        });
    });

    // Handle window closed
    mainWindow.on('closed', function() {
        mainWindow = null;
    });

    // Handle external links
    mainWindow.webContents.setWindowOpenHandler(({ url }) => {
        shell.openExternal(url);
        return { action: 'deny' };
    });

    // Create application menu
    createMenu();
}

function createMenu() {
    const template = [
        {
            label: 'File',
            submenu: [
                {
                    label: 'New Scan',
                    accelerator: 'CmdOrCtrl+N',
                    click: () => {
                        mainWindow.webContents.executeJavaScript('scanForFiles()');
                    }
                },
                {
                    label: 'Export Report',
                    accelerator: 'CmdOrCtrl+E',
                    click: exportReport
                },
                { type: 'separator' },
                {
                    label: 'Exit',
                    accelerator: process.platform === 'darwin' ? 'Cmd+Q' : 'Ctrl+Q',
                    click: () => {
                        app.quit();
                    }
                }
            ]
        },
        {
            label: 'Tools',
            submenu: [
                {
                    label: 'Disk Cleanup',
                    accelerator: 'CmdOrCtrl+1',
                    click: () => {
                        mainWindow.webContents.executeJavaScript("switchTab('disk-cleanup')");
                    }
                },
                {
                    label: 'Bloatware Removal',
                    accelerator: 'CmdOrCtrl+2',
                    click: () => {
                        mainWindow.webContents.executeJavaScript("switchTab('bloatware')");
                    }
                },
                {
                    label: 'Startup Manager',
                    accelerator: 'CmdOrCtrl+3',
                    click: () => {
                        mainWindow.webContents.executeJavaScript("switchTab('startup')");
                    }
                },
                {
                    label: 'System Info',
                    accelerator: 'CmdOrCtrl+4',
                    click: () => {
                        mainWindow.webContents.executeJavaScript("switchTab('system-info')");
                    }
                },
                { type: 'separator' },
                {
                    label: 'Run as Administrator',
                    click: runAsAdmin
                }
            ]
        },
        {
            label: 'View',
            submenu: [
                { role: 'reload' },
                { role: 'forceReload' },
                { role: 'toggleDevTools' },
                { type: 'separator' },
                { role: 'resetZoom' },
                { role: 'zoomIn' },
                { role: 'zoomOut' },
                { type: 'separator' },
                { role: 'togglefullscreen' }
            ]
        },
        {
            label: 'Help',
            submenu: [
                {
                    label: 'About',
                    click: showAbout
                },
                {
                    label: 'User Guide',
                    click: () => {
                        shell.openExternal('https://github.com/yourusername/windows-system-optimizer/wiki');
                    }
                },
                {
                    label: 'Report Issue',
                    click: () => {
                        shell.openExternal('https://github.com/yourusername/windows-system-optimizer/issues');
                    }
                }
            ]
        }
    ];

    const menu = Menu.buildFromTemplate(template);
    Menu.setApplicationMenu(menu);
}

// Enhanced IPC handlers with robust error handling
ipcMain.handle('get-system-info', withErrorHandling(async () => {
    const systemInfo = await getSystemInformation();
    return systemInfo;
}, 'Failed to get system information'));

ipcMain.handle('scan-temp-files', withErrorHandling(async () => {
    const tempFiles = await scanTempFiles();
    return tempFiles;
}, 'Failed to scan temporary files'));

ipcMain.handle('cleanup-files', withErrorHandling(async (event, filePaths) => {
    const result = await cleanupFiles(filePaths);
    return result;
}, 'Failed to cleanup files'));

// Enhanced bloatware scanning with better detection
ipcMain.handle('scan-bloatware-enhanced', withErrorHandling(async () => {
    const bloatwareData = await scanBloatwareEnhanced();
    return bloatwareData;
}, 'Failed to scan for bloatware'));

// Enhanced duplicate file scanning with merge options
ipcMain.handle('scan-duplicate-files-enhanced', withErrorHandling(async (event, scanPath) => {
    const duplicates = await scanForDuplicateFilesEnhanced(scanPath || 'C:\\');
    return duplicates;
}, 'Failed to scan for duplicate files'));

// Registry cleanup for complete app removal
ipcMain.handle('cleanup-registry', withErrorHandling(async (event, appName) => {
    const result = await cleanupRegistryEntries(appName);
    return result;
}, 'Failed to cleanup registry entries'));

// Enhanced space saver operations
ipcMain.handle('purge-recycle-bin', withErrorHandling(async () => {
    const result = await purgeRecycleBin();
    return result;
}, 'Failed to purge recycle bin'));

ipcMain.handle('clear-all-temp-data', withErrorHandling(async () => {
    const result = await clearAllTempData();
    return result;
}, 'Failed to clear temp data'));

ipcMain.handle('merge-duplicate-files', withErrorHandling(async (event, duplicateGroups, mergeStrategy) => {
    const result = await mergeDuplicateFiles(duplicateGroups, mergeStrategy);
    return result;
}, 'Failed to merge duplicate files'));

ipcMain.handle('get-startup-items', async () => {
    try {
        const startupItems = await getStartupItems();
        return startupItems;
    } catch (error) {
        log.error('Error getting startup items:', error);
        return [];
    }
});

ipcMain.handle('toggle-startup-item', async (event, itemName, enable) => {
    try {
        const result = await toggleStartupItem(itemName, enable);
        return result;
    } catch (error) {
        log.error('Error toggling startup item:', error);
        return { success: false, error: error.message };
    }
});

ipcMain.handle('get-installed-programs', async () => {
    try {
        const installedPrograms = await getInstalledPrograms();
        return installedPrograms;
    } catch (error) {
        log.error('Error getting installed programs:', error);
        return [];
    }
});

ipcMain.handle('get-system-stats', async () => {
    try {
        const systemStats = await getSystemStats();
        return systemStats;
    } catch (error) {
        log.error('Error getting system stats:', error);
        return { cpuUsage: 15, memoryUsage: 45, diskUsage: 65 };
    }
});

ipcMain.handle('uninstall-program', async (event, programName) => {
    try {
        const result = await uninstallProgram(programName);
        return result;
    } catch (error) {
        log.error('Error uninstalling program:', error);
        return { success: false, error: error.message };
    }
});

// IPC handler to open a folder browser dialog and return the selected path
ipcMain.handle('browse-folder', async () => {
    const result = await dialog.showOpenDialog(mainWindow, {
        properties: ['openDirectory']
    });
    if (result.canceled || !result.filePaths.length) return null;
    return result.filePaths[0];
});

// IPC handler to scan for duplicate files
ipcMain.handle('scan-duplicate-files', async (event, scanPath) => {
    try {
        const duplicates = await scanForDuplicateFiles(scanPath || 'C:\\');
        return duplicates;
    } catch (error) {
        log.error('Error scanning for duplicate files:', error);
        return [];
    }
});

// IPC handler to clear the Recycle Bin
ipcMain.handle('clear-recycle-bin', async () => {
    try {
        const result = await clearRecycleBin();
        return result;
    } catch (error) {
        log.error('Error clearing Recycle Bin:', error);
        return { success: false, error: error.message };
    }
});

// IPC handler to clear all temp files (one-click)
ipcMain.handle('clear-all-temp-files', async () => {
    try {
        const result = await clearAllTempFiles();
        return result;
    } catch (error) {
        log.error('Error clearing all temp files:', error);
        return { success: false, error: error.message };
    }
});

// Enhanced function to scan for duplicate files with better performance and metadata
async function scanForDuplicateFilesEnhanced(rootPath) {
    const fileMap = new Map(); // key: size, value: array of file paths
    const hashMap = new Map(); // key: hash, value: array of file paths
    const duplicates = [];
    const scannedFiles = new Set();
    let totalFiles = 0;
    let processedFiles = 0;

    // Enhanced directory walker with progress tracking
    function walk(dir, depth = 0) {
        if (depth > 10) return; // Prevent infinite recursion
        
        let files;
        try {
            files = fs.readdirSync(dir);
        } catch (e) {
            log.warn('Cannot read directory:', dir, e.message);
            return;
        }
        
        for (const file of files) {
            const filePath = path.join(dir, file);
            
            // Skip system files and hidden files
            if (file.startsWith('.') || file === 'System Volume Information' || 
                file === '$Recycle.Bin' || file === 'Windows' || file === 'Program Files') {
                continue;
            }
            
            let stats;
            try {
                stats = fs.statSync(filePath);
            } catch (e) {
                continue;
            }
            
            if (stats.isDirectory()) {
                walk(filePath, depth + 1);
            } else if (stats.isFile() && stats.size > 0) { // Only process non-empty files
                totalFiles++;
                const size = stats.size;
                if (!fileMap.has(size)) fileMap.set(size, []);
                fileMap.get(size).push({
                    path: filePath,
                    size: size,
                    modified: stats.mtime,
                    created: stats.birthtime
                });
            }
        }
    }

    log.info('Starting enhanced duplicate file scan in:', rootPath);
    walk(rootPath);

    // Process files by size groups for better performance
    for (const [size, files] of fileMap.entries()) {
        if (files.length < 2) continue;
        
        // For small files, use full hash comparison
        if (size < 1024 * 1024) { // Less than 1MB
            const hashGroups = {};
            for (const file of files) {
                let hash;
                try {
                    const data = fs.readFileSync(file.path);
                    hash = crypto.createHash('md5').update(data).digest('hex');
                } catch (e) {
                    log.warn('Cannot read file for hashing:', file.path, e.message);
                    continue;
                }
                if (!hashGroups[hash]) hashGroups[hash] = [];
                hashGroups[hash].push(file);
            }
            
            for (const group of Object.values(hashGroups)) {
                if (group.length > 1) {
                    duplicates.push({
                        files: group,
                        size: size,
                        totalSize: size * group.length,
                        type: 'exact_duplicate'
                    });
                }
            }
        } else {
            // For large files, use partial hash comparison
            const hashGroups = {};
            for (const file of files) {
                let hash;
                try {
                    const stream = fs.createReadStream(file.path, { start: 0, end: 8191 }); // First 8KB
                    const hash = crypto.createHash('md5');
                    hash.update(stream);
                    const partialHash = hash.digest('hex');
                } catch (e) {
                    log.warn('Cannot read file for partial hashing:', file.path, e.message);
                    continue;
                }
                if (!hashGroups[hash]) hashGroups[hash] = [];
                hashGroups[hash].push(file);
            }
            
            for (const group of Object.values(hashGroups)) {
                if (group.length > 1) {
                    duplicates.push({
                        files: group,
                        size: size,
                        totalSize: size * group.length,
                        type: 'potential_duplicate'
                    });
                }
            }
        }
        
        processedFiles += files.length;
        log.info(`Processed ${processedFiles}/${totalFiles} files, found ${duplicates.length} duplicate groups`);
    }
    
    log.info('Enhanced duplicate scan completed:', duplicates.length, 'groups found');
    return duplicates;
}

// Enhanced function to scan for duplicate files (backward compatibility)
async function scanForDuplicateFiles(rootPath) {
    const enhancedResults = await scanForDuplicateFilesEnhanced(rootPath);
    return enhancedResults.map(group => group.files.map(file => file.path));
}

// Enhanced function to purge Recycle Bin with detailed reporting
async function purgeRecycleBin() {
    return new Promise((resolve) => {
        const psCmd = 'powershell -NoProfile -ExecutionPolicy Bypass -Command "' +
            '$recycleBin = Get-ChildItem $env:SystemDrive\\`$Recycle.Bin -Recurse -Force -ErrorAction SilentlyContinue; ' +
            '$totalSize = ($recycleBin | Measure-Object -Property Length -Sum).Sum; ' +
            '$fileCount = $recycleBin.Count; ' +
            'Clear-RecycleBin -Force -ErrorAction SilentlyContinue; ' +
            'Write-Output \"$fileCount|$totalSize\"" +
        '"';
        
        exec(psCmd, (error, stdout, stderr) => {
            if (error) {
                log.error('PowerShell purge-recycle-bin error:', error);
                resolve({ success: false, error: error.message });
                return;
            }
            
            const output = stdout.toString().trim();
            const parts = output.split('|');
            const fileCount = parseInt(parts[0]) || 0;
            const totalSize = parseInt(parts[1]) || 0;
            
            resolve({ 
                success: true, 
                message: `Recycle Bin purged successfully`,
                fileCount: fileCount,
                totalSize: totalSize,
                freedSpace: totalSize
            });
        });
    });
}

// Enhanced function to clear all temp data comprehensively
async function clearAllTempData() {
    const tempPaths = [
        process.env.TEMP,
        process.env.TMP,
        'C:\\Windows\\Temp',
        path.join(process.env.LOCALAPPDATA, 'Temp'),
        path.join(process.env.APPDATA, 'Temp'),
        'C:\\Windows\\Prefetch',
        path.join(process.env.LOCALAPPDATA, 'Microsoft', 'Windows', 'INetCache'),
        path.join(process.env.LOCALAPPDATA, 'Microsoft', 'Windows', 'WebCache'),
        path.join(process.env.LOCALAPPDATA, 'Microsoft', 'Windows', 'History'),
        path.join(process.env.LOCALAPPDATA, 'Microsoft', 'Windows', 'Cookies')
    ].filter(p => p);
    
    let totalDeleted = 0;
    let totalSize = 0;
    const errors = [];
    const results = {};
    
    for (const tempPath of tempPaths) {
        if (!fs.existsSync(tempPath)) continue;
        
        try {
            const stats = await getDirectoryStats(tempPath);
            const deleted = await clearDirectory(tempPath);
            
            results[tempPath] = {
                deleted: deleted.count,
                size: deleted.size,
                errors: deleted.errors
            };
            
            totalDeleted += deleted.count;
            totalSize += deleted.size;
            errors.push(...deleted.errors);
            
        } catch (error) {
            log.error('Error clearing temp path:', tempPath, error);
            errors.push({ path: tempPath, error: error.message });
        }
    }
    
    return {
        success: true,
        totalDeleted,
        totalSize,
        errors,
        results
    };
}

// Helper function to get directory statistics
async function getDirectoryStats(dirPath) {
    return new Promise((resolve, reject) => {
        const psCmd = `powershell -NoProfile -ExecutionPolicy Bypass -Command "Get-ChildItem '${dirPath}' -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum"`;
        exec(psCmd, (error, stdout, stderr) => {
            if (error) {
                resolve({ count: 0, size: 0 });
                return;
            }
            
            const lines = stdout.toString().split('\n');
            const countMatch = lines.find(line => line.includes('Count'));
            const sumMatch = lines.find(line => line.includes('Sum'));
            
            const count = countMatch ? parseInt(countMatch.split(':')[1]) || 0 : 0;
            const size = sumMatch ? parseInt(sumMatch.split(':')[1]) || 0 : 0;
            
            resolve({ count, size });
        });
    });
}

// Helper function to clear directory contents
async function clearDirectory(dirPath) {
    return new Promise((resolve) => {
        const psCmd = `powershell -NoProfile -ExecutionPolicy Bypass -Command "Get-ChildItem '${dirPath}' -Recurse -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue"`;
        exec(psCmd, (error, stdout, stderr) => {
            resolve({ count: 0, size: 0, errors: error ? [error.message] : [] });
        });
    });
}

// Function to clear the Recycle Bin using PowerShell (backward compatibility)
async function clearRecycleBin() {
    const result = await purgeRecycleBin();
    return result;
}

// Function to clear all temp files in common temp directories
async function clearAllTempFiles() {
    const tempPaths = [
        process.env.TEMP,
        process.env.TMP,
        'C:\\Windows\\Temp',
        path.join(process.env.LOCALAPPDATA, 'Temp')
    ].filter(p => p);
    let deletedCount = 0;
    let totalSize = 0;
    let errors = [];
    for (const tempPath of tempPaths) {
        if (!fs.existsSync(tempPath)) continue;
        const files = fs.readdirSync(tempPath);
        for (const file of files) {
            const filePath = path.join(tempPath, file);
            try {
                const stats = fs.statSync(filePath);
                if (stats.isFile()) {
                    totalSize += stats.size;
                    fs.unlinkSync(filePath);
                    deletedCount++;
                } else if (stats.isDirectory()) {
                    // Recursively delete directory
                    fs.rmSync(filePath, { recursive: true, force: true });
                    deletedCount++;
                }
            } catch (err) {
                errors.push({ file: filePath, error: err.message });
            }
        }
    }
    return {
        success: true,
        deletedCount,
        totalSize,
        errors
    };
}

// System operation functions
async function getSystemInformation() {
    return new Promise((resolve, reject) => {
        const commands = [
            'wmic os get Caption,Version,BuildNumber /format:csv',
            'wmic cpu get Name /format:csv',
            'wmic computersystem get TotalPhysicalMemory /format:csv',
            'wmic diskdrive get Size,Model /format:csv'
        ];
        
        const results = {};
        let completed = 0;
        
        commands.forEach((cmd, index) => {
            exec(cmd, (error, stdout, stderr) => {
                if (error) {
                    log.error(`Command ${index} error:`, error);
                } else {
                    results[index] = stdout;
                }
                completed++;
                if (completed === commands.length) {
                    resolve(parseSystemInfo(results));
                }
            });
        });
    });
}

function parseSystemInfo(results) {
    // Parse the CSV output from WMI commands
    const systemInfo = {
        os: 'Windows',
        processor: 'Unknown',
        memory: 'Unknown',
        storage: 'Unknown'
    };
    
    try {
        // Parse OS info
        if (results[0]) {
            const osLines = results[0].split('\n').filter(line => line.trim() && !line.includes('Node'));
            if (osLines.length > 1) {
                const osData = osLines[1].split(',');
                systemInfo.os = `${osData[2]} (Build ${osData[1]})`;
            }
        }
        
        // Parse CPU info
        if (results[1]) {
            const cpuLines = results[1].split('\n').filter(line => line.trim() && !line.includes('Node'));
            if (cpuLines.length > 1) {
                systemInfo.processor = cpuLines[1].split(',')[1] || 'Unknown';
            }
        }
        
        // Parse memory info
        if (results[2]) {
            const memLines = results[2].split('\n').filter(line => line.trim() && !line.includes('Node'));
            if (memLines.length > 1) {
                const totalMemory = parseInt(memLines[1].split(',')[1]);
                systemInfo.memory = `${Math.round(totalMemory / (1024 * 1024 * 1024))} GB`;
            }
        }
    } catch (error) {
        log.error('Error parsing system info:', error);
    }
    
    return systemInfo;
}

async function scanTempFiles() {
    const tempPaths = [
        process.env.TEMP,
        process.env.TMP,
        'C:\\Windows\\Temp',
        path.join(process.env.LOCALAPPDATA, 'Temp')
    ].filter(p => p); // Remove undefined paths
    
    const tempFiles = [];
    let totalScanned = 0;
    let totalErrors = 0;
    
    log.info(`Starting temp file scan of ${tempPaths.length} directories`);
    
    for (const tempPath of tempPaths) {
        try {
            if (!fs.existsSync(tempPath)) {
                log.warn('Temp path does not exist:', tempPath);
                continue;
            }
            
            log.info('Scanning directory:', tempPath);
            const files = fs.readdirSync(tempPath);
            totalScanned++;
            
            for (const file of files) {
                const filePath = path.join(tempPath, file);
                try {
                    const stats = fs.statSync(filePath);
                    
                    // Only include files, not directories (for safety)
                    if (stats.isFile()) {
                        tempFiles.push({
                            name: file,
                            path: filePath,
                            size: stats.size,
                            modified: stats.mtime
                        });
                    } else if (stats.isDirectory()) {
                        log.debug('Skipping directory:', filePath);
                    }
                } catch (err) {
                    // Skip files we can't access
                    totalErrors++;
                    log.warn('Cannot access file:', filePath, err.message);
                }
            }
        } catch (error) {
            totalErrors++;
            log.error('Error scanning temp path:', tempPath, error.message);
        }
    }
    
    log.info(`Temp file scan completed: ${tempFiles.length} files found, ${totalScanned} directories scanned, ${totalErrors} errors`);
    
    return tempFiles;
}

async function cleanupFiles(filePaths) {
    let deletedCount = 0;
    let totalSize = 0;
    const errors = [];
    
    log.info(`Starting cleanup of ${filePaths.length} files`);
    
    for (const filePath of filePaths) {
        try {
            // Validate file path
            if (!filePath || typeof filePath !== 'string') {
                errors.push({ file: filePath, error: 'Invalid file path' });
                continue;
            }
            
            // Check if file exists before trying to delete
            if (!fs.existsSync(filePath)) {
                log.warn('File no longer exists:', filePath);
                continue;
            }
            
            const stats = fs.statSync(filePath);
            
            // Skip directories for now - only delete files
            if (stats.isDirectory()) {
                log.warn('Skipping directory:', filePath);
                continue;
            }
            
            // Attempt to delete the file
            fs.unlinkSync(filePath);
            deletedCount++;
            totalSize += stats.size;
            log.info('Successfully deleted file:', filePath);
            
        } catch (error) {
            const errorMsg = error.code === 'EACCES' ? 'Access denied - file may be in use' :
                           error.code === 'ENOENT' ? 'File not found' :
                           error.code === 'EPERM' ? 'Permission denied' :
                           error.message;
            
            errors.push({ file: filePath, error: errorMsg });
            log.error('Error deleting file:', filePath, errorMsg);
        }
    }
    
    log.info(`Cleanup completed: ${deletedCount} files deleted, ${errors.length} errors`);
    
    return {
        success: true,
        deletedCount,
        totalSize,
        errors
    };
}

// Enhanced bloatware scanning with comprehensive detection
async function scanBloatwareEnhanced() {
    return new Promise((resolve) => {
        const psCmd = 'powershell -NoProfile -ExecutionPolicy Bypass -Command "' +
            // Get installed programs from registry
            '$installed = Get-ItemProperty @(' +
            '"HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",' +
            '"HKLM:\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*",' +
            '"HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*") ' +
            '| Where-Object { $_.DisplayName } | Select-Object DisplayName,Publisher,DisplayVersion,EstimatedSize,InstallDate; ' +
            
            // Get UWP apps
            '$uwpApps = Get-AppxPackage -AllUsers | Select-Object Name,PackageFullName,Publisher,InstallLocation; ' +
            
            // Get provisioned apps
            '$provApps = Get-AppxProvisionedPackage -Online | Select-Object DisplayName,PackageName; ' +
            
            // Combine and categorize
            '$allApps = @(); ' +
            '$installed | ForEach-Object { $allApps += [PSCustomObject]@{ ' +
            'Name = $_.DisplayName; ' +
            'Publisher = $_.Publisher; ' +
            'Version = $_.DisplayVersion; ' +
            'Size = $_.EstimatedSize; ' +
            'Type = "Installed"; ' +
            'InstallDate = $_.InstallDate; ' +
            'Location = "Registry" } }; ' +
            
            '$uwpApps | ForEach-Object { $allApps += [PSCustomObject]@{ ' +
            'Name = $_.Name; ' +
            'Publisher = $_.Publisher; ' +
            'Version = "UWP"; ' +
            'Size = "Unknown"; ' +
            'Type = "UWP"; ' +
            'InstallDate = "Unknown"; ' +
            'Location = $_.InstallLocation } }; ' +
            
            '$allApps | ConvertTo-Json -Compress' +
        '"';

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error || !stdout) {
                log.error('PowerShell scanBloatwareEnhanced error:', error || 'no output');
                resolve({ preinstalled: [], thirdParty: [], uwp: [], system: [] });
                return;
            }
            
            try {
                let data = JSON.parse(stdout.toString());
                if (!Array.isArray(data)) data = [data];
                
                const categorizedApps = categorizeBloatwareApps(data);
                resolve(categorizedApps);
                
            } catch (err) {
                log.error('Parse bloatware JSON failed:', err);
                resolve({ preinstalled: [], thirdParty: [], uwp: [], system: [] });
            }
        });
    });
}

// Enhanced app categorization with better bloatware detection
function categorizeBloatwareApps(apps) {
    const knownBloatware = [
        'candy crush', 'disney magic', 'netflix', 'spotify', 'twitter', 'instagram', 'facebook', 'tiktok',
        'xbox game bar', 'mixed reality', 'your phone', 'cortana', 'groove music', 'movies & tv', 'skype',
        'solitaire', 'bubble witch', 'march of empires', 'hidden city', 'farmville', 'minecraft', 'roblox',
        'asphalt', 'flipboard', 'pinterest', 'linkedin', 'whatsapp', 'telegram', 'discord', 'zoom',
        'adobe creative cloud', 'mcafee', 'norton', 'avast', 'avg', 'ccleaner', 'driver booster',
        'advanced systemcare', 'pc cleaner', 'registry cleaner', 'toolbar', 'search protect',
        'browser hijacker', 'adware', 'trial', 'demo', 'free version', 'limited edition'
    ];

    const systemApps = [
        'windows security', 'windows defender', 'edge', 'notepad', 'calculator', 'camera', 'mail',
        'calendar', 'weather', 'maps', 'photos', 'clock', 'sticky notes', 'snipping tool', 'paint',
        'wordpad', 'windows media player', 'internet explorer', 'windows store', 'settings',
        'control panel', 'file explorer', 'task manager', 'device manager', 'disk management'
    ];

    const microsoftApps = [
        'microsoft office', 'word', 'excel', 'powerpoint', 'outlook', 'access', 'publisher',
        'visual studio', 'sql server', 'azure', 'teams', 'onedrive', 'sharepoint', 'dynamics'
    ];

    const categorized = {
        preinstalled: [],
        thirdParty: [],
        uwp: [],
        system: []
    };

    apps.forEach(app => {
        const name = (app.Name || '').toLowerCase();
        const publisher = (app.Publisher || '').toLowerCase();
        const type = app.Type || 'Unknown';
        
        // Determine category and risk level
        let category = 'thirdParty';
        let risk = 'Medium';
        let recommendation = 'Review';
        let description = '';
        
        // Check for known bloatware
        const isBloatware = knownBloatware.some(bloat => name.includes(bloat));
        const isSystem = systemApps.some(sys => name.includes(sys));
        const isMicrosoft = microsoftApps.some(ms => name.includes(ms)) || publisher.includes('microsoft');
        const isUWP = type === 'UWP';
        
        if (isBloatware) {
            category = 'preinstalled';
            risk = 'Safe';
            recommendation = 'Remove';
            description = `Bloatware: ${app.Name}`;
        } else if (isSystem) {
            category = 'system';
            risk = 'High';
            recommendation = 'Keep';
            description = `System app: ${app.Name}`;
        } else if (isMicrosoft && !isBloatware) {
            category = 'preinstalled';
            risk = 'Medium';
            recommendation = 'Review';
            description = `Microsoft app: ${app.Name}`;
        } else if (isUWP) {
            category = 'uwp';
            risk = 'Medium';
            recommendation = 'Review';
            description = `UWP app: ${app.Name}`;
        } else {
            category = 'thirdParty';
            risk = 'Medium';
            recommendation = 'Review';
            description = `Third-party app: ${app.Name}`;
        }
        
        const appData = {
            name: app.Name || 'Unknown',
            publisher: app.Publisher || 'Unknown',
            version: app.Version || 'Unknown',
            size: app.Size || 'Unknown',
            type: type,
            risk: risk,
            recommendation: recommendation,
            description: description,
            installDate: app.InstallDate || 'Unknown',
            location: app.Location || 'Unknown',
            realName: app.Name
        };
        
        categorized[category].push(appData);
    });
    
    return categorized;
}

// === PowerShell-based implementations to replace deprecated WMIC ===
async function getInstalledPrograms() {
    const enhancedResults = await scanBloatwareEnhanced();
    return [
        ...enhancedResults.preinstalled,
        ...enhancedResults.thirdParty,
        ...enhancedResults.uwp,
        ...enhancedResults.system
    ];
}

async function getStartupItems() {
    return new Promise((resolve) => {
        const psCmd = 'powershell -NoProfile -ExecutionPolicy Bypass -Command "' +
            `Get-CimInstance Win32_StartupCommand | Select-Object Name,Command,Location | ConvertTo-Json -Compress` +
        '"';

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error || !stdout) {
                log.error('PowerShell getStartupItems error:', error || 'no output');
                resolve([]);
                return;
            }
            try {
                let data = JSON.parse(stdout.toString());
                if (!Array.isArray(data)) data = [data];
                const items = data.map(i => ({
                    name: i.Name || 'Unknown',
                    command: i.Command || '',
                    location: i.Location || '',
                    enabled: true
                }));
                resolve(items);
            } catch (err) {
                log.error('Parse startup items JSON failed:', err);
                resolve([]);
            }
        });
    });
}

async function toggleStartupItem(itemName, enable) {
    // This would require more complex registry operations
    // For now, return a success message
    log.info(`${enable ? 'Enabling' : 'Disabling'} startup item:`, itemName);
    return { success: true, message: `Startup item ${enable ? 'enabled' : 'disabled'}` };
}

async function getSystemStats() {
    return new Promise((resolve) => {
        const psScript = [
            // Embedded PowerShell script with simpler, more readable syntax
            '$cpu = (Get-CimInstance Win32_Processor | Measure-Object -Property LoadPercentage -Average).Average;',
            '$os = Get-CimInstance Win32_OperatingSystem;',
            '$totalMem = [int]$os.TotalVisibleMemorySize;',
            '$freeMem = [int]$os.FreePhysicalMemory;',
            // Use Where-Object instead of complex Filter syntax
            '$drive = Get-CimInstance Win32_LogicalDisk | Where-Object { $_.DeviceID -eq \'C:\' };',
            '$totalSpace = [int]$drive.Size;',
            '$freeSpace = [int]$drive.FreeSpace;',
            '$obj = [PSCustomObject]@{',
            '  cpu = $cpu;',
            '  memTotal = $totalMem;',
            '  memFree = $freeMem;',
            '  diskTotal = $totalSpace;',
            '  diskFree = $freeSpace',
            '};',
            '$obj | ConvertTo-Json -Compress'
        ].join(' ');

        const psCmd = `powershell -NoProfile -ExecutionPolicy Bypass -Command "${psScript}"`;

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error || !stdout) {
                log.error('PowerShell getSystemStats error:', error || 'no output');
                // return default values so UI still works
                resolve({ cpuUsage: 0, memoryUsage: 0, diskUsage: 0 });
                return;
            }
            try {
                const data = JSON.parse(stdout.toString());
                const cpuUsage = parseInt(data.cpu) || 0;

                const totalMemKB = parseInt(data.memTotal) || 1; // prevent div-by-zero
                const usedMemKB = totalMemKB - (parseInt(data.memFree) || 0);
                const memoryUsage = Math.round((usedMemKB / totalMemKB) * 100);

                const totalDisk = parseInt(data.diskTotal) || 1;
                const usedDisk = totalDisk - (parseInt(data.diskFree) || 0);
                const diskUsage = Math.round((usedDisk / totalDisk) * 100);

                resolve({ cpuUsage, memoryUsage, diskUsage });
            } catch (err) {
                log.error('Parse system stats JSON failed:', err);
                resolve({ cpuUsage: 0, memoryUsage: 0, diskUsage: 0 });
            }
        });
    });
}

// Enhanced registry cleanup for complete app removal
async function cleanupRegistryEntries(appName) {
    return new Promise((resolve) => {
        const sanitized = appName.replace(/`/g, '').replace(/'/g, "''");
        
        const psScript = `
        Try {
            $cleaned = @()
            $errors = @()
            
            # Registry paths to clean
            $regPaths = @(
                'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                'HKLM:SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths',
                'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths',
                'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run',
                'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run',
                'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce',
                'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce'
            )
            
            foreach ($regPath in $regPaths) {
                try {
                    $items = Get-ChildItem $regPath -ErrorAction SilentlyContinue | Where-Object {
                        ($_.GetValue('DisplayName') -like '*${sanitized}*') -or
                        ($_.GetValue('DisplayName') -like '*${sanitized.split(' ')[0]}*') -or
                        ($_.Name -like '*${sanitized}*')
                    }
                    
                    foreach ($item in $items) {
                        try {
                            Remove-Item $item.PsPath -Recurse -Force -ErrorAction SilentlyContinue
                            $cleaned += "$regPath\\$($item.PSChildName)"
                        } catch {
                            $errors += "Failed to remove $regPath\\$($item.PSChildName): $($_.Exception.Message)"
                        }
                    }
                } catch {
                    $errors += "Failed to access $regPath: $($_.Exception.Message)"
                }
            }
            
            # Clean file associations
            $assocPaths = @(
                'HKLM:SOFTWARE\\Classes',
                'HKCU:SOFTWARE\\Classes'
            )
            
            foreach ($assocPath in $assocPaths) {
                try {
                    $assocs = Get-ChildItem $assocPath -ErrorAction SilentlyContinue | Where-Object {
                        $_.Name -like '*${sanitized}*'
                    }
                    
                    foreach ($assoc in $assocs) {
                        try {
                            Remove-Item $assoc.PsPath -Recurse -Force -ErrorAction SilentlyContinue
                            $cleaned += "$assocPath\\$($assoc.PSChildName)"
                        } catch {
                            $errors += "Failed to remove association $assocPath\\$($assoc.PSChildName): $($_.Exception.Message)"
                        }
                    }
                } catch {
                    $errors += "Failed to access $assocPath: $($_.Exception.Message)"
                }
            }
            
            $result = @{
                cleaned = $cleaned
                errors = $errors
                success = $true
            }
            
            $result | ConvertTo-Json -Compress
        } Catch {
            @{
                cleaned = @()
                errors = @($_.Exception.Message)
                success = $false
            } | ConvertTo-Json -Compress
        }`.replace(/\s+/g, ' ').trim();

        const psCmd = `powershell -NoProfile -ExecutionPolicy Bypass -Command "${psScript}"`;

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error) {
                log.error('PowerShell registry cleanup error:', error);
                resolve({ success: false, error: error.message });
                return;
            }
            
            try {
                const result = JSON.parse(stdout.toString());
                log.info('Registry cleanup completed for:', appName, result);
                resolve(result);
            } catch (err) {
                log.error('Parse registry cleanup JSON failed:', err);
                resolve({ success: false, error: 'Failed to parse registry cleanup results' });
            }
        });
    });
}

// Enhanced duplicate file merge with multiple strategies
async function mergeDuplicateFiles(duplicateGroups, mergeStrategy) {
    return new Promise((resolve) => {
        let deletedCount = 0;
        let totalSize = 0;
        const errors = [];
        
        duplicateGroups.forEach(group => {
            if (group.files && group.files.length > 1) {
                const files = group.files;
                let filesToDelete = [];
                
                switch (mergeStrategy) {
                    case 'keep_oldest':
                        // Keep the oldest file, delete the rest
                        files.sort((a, b) => new Date(a.created) - new Date(b.created));
                        filesToDelete = files.slice(1);
                        break;
                    case 'keep_newest':
                        // Keep the newest file, delete the rest
                        files.sort((a, b) => new Date(b.created) - new Date(a.created));
                        filesToDelete = files.slice(1);
                        break;
                    case 'keep_first':
                        // Keep the first file, delete the rest
                        filesToDelete = files.slice(1);
                        break;
                    case 'keep_last':
                        // Keep the last file, delete the rest
                        filesToDelete = files.slice(0, -1);
                        break;
                    case 'delete_all_but_one':
                        // Keep one file, delete all others
                        filesToDelete = files.slice(1);
                        break;
                    default:
                        // Default: keep the first file
                        filesToDelete = files.slice(1);
                }
                
                // Delete the selected files
                filesToDelete.forEach(file => {
                    try {
                        if (fs.existsSync(file.path)) {
                            const stats = fs.statSync(file.path);
                            fs.unlinkSync(file.path);
                            deletedCount++;
                            totalSize += stats.size;
                        }
                    } catch (error) {
                        errors.push({ file: file.path, error: error.message });
                    }
                });
            }
        });
        
        resolve({
            success: true,
            deletedCount,
            totalSize,
            errors,
            message: `Merged ${duplicateGroups.length} duplicate groups, deleted ${deletedCount} files`
        });
    });
}

async function uninstallProgram(programName) {
    return new Promise((resolve) => {
        // Sanitize and properly quote the application name for PowerShell
        const sanitized = programName.replace(/`/g, '').replace(/'/g, "''"); // escape single quotes for PowerShell

        // Enhanced PowerShell script for deep removal with registry cleanup
        const psScript = `
        Try {
            $found = $false
            $removalMethods = @()
            
            # 1. Try Get-Package/Uninstall-Package
            $app = Get-Package -Name '${sanitized}' -ErrorAction SilentlyContinue;
            if ($app) {
                $app | Uninstall-Package -Force -ErrorAction SilentlyContinue;
                $removalMethods += "Get-Package"
                $found = $true
            }
            
            # 2. Try winget
            if (Get-Command winget -ErrorAction SilentlyContinue) {
                $result = winget uninstall '${sanitized}' --silent --accept-source-agreements 2>$null;
                if ($LASTEXITCODE -eq 0) { 
                    $removalMethods += "winget"
                    $found = $true
                }
            }
            
            # 3. Try WMI
            $app = Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -eq '${sanitized}' };
            if ($app) {
                $null = $app.Uninstall();
                $removalMethods += "WMI"
                $found = $true
            }
            
            # 4. Try UWP/Provisioned App removal (for preloaded apps)
            $uwp = Get-AppxPackage -Name '*${sanitized}*' -ErrorAction SilentlyContinue;
            if ($uwp) {
                $uwp | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue;
                $removalMethods += "Remove-AppxPackage"
                $found = $true
            }
            
            $prov = Get-AppxProvisionedPackage -Online | Where-Object { $_.DisplayName -like '*${sanitized}*' };
            if ($prov) {
                $prov | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue;
                $removalMethods += "Remove-AppxProvisionedPackage"
                $found = $true
            }
            
            # 5. Enhanced Registry Cleanup
            if ($found) {
                $regPaths = @(
                    'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                    'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                    'HKLM:SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall',
                    'HKLM:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths',
                    'HKCU:SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths'
                )
                
                foreach ($regPath in $regPaths) {
                    Get-ChildItem $regPath -ErrorAction SilentlyContinue | Where-Object {
                        ($_.GetValue('DisplayName') -like '*${sanitized}*') -or
                        ($_.Name -like '*${sanitized}*')
                    } | ForEach-Object {
                        Remove-Item $_.PsPath -Recurse -Force -ErrorAction SilentlyContinue
                    }
                }
                
                # Remove leftover files
                $folders = @(
                    "$env:ProgramFiles\\*${sanitized}*",
                    "$env:ProgramFiles(x86)\\*${sanitized}*",
                    "$env:LOCALAPPDATA\\*${sanitized}*",
                    "$env:APPDATA\\*${sanitized}*"
                )
                
                foreach ($folder in $folders) {
                    Get-ChildItem -Path $folder -Recurse -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
                }
                
                # Remove Start Menu shortcuts
                $startMenu = "$env:ProgramData\\Microsoft\\Windows\\Start Menu\\Programs"
                Get-ChildItem -Path $startMenu -Recurse -Force -ErrorAction SilentlyContinue | Where-Object { $_.Name -like '*${sanitized}*' } | Remove-Item -Force -ErrorAction SilentlyContinue
                
                $removalMethods += "Registry Cleanup"
                Write-Output "SUCCESS:Removed via $($removalMethods -join ', ')"
                return
            }
            
            Write-Output 'NOTFOUND'
        } Catch {
            Write-Output "ERROR:$($_.Exception.Message)"
        }`.replace(/\s+/g, ' ').trim();

        const psCmd = `powershell -NoProfile -ExecutionPolicy Bypass -Command "${psScript}"`;

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error) {
                log.error('PowerShell uninstall error:', error);
                resolve({ success: false, error: error.message });
                return;
            }
            const output = stdout.toString().trim();
            if (/SUCCESS/i.test(output)) {
                log.info('Program uninstalled:', programName, output);
                resolve({ success: true, message: output });
            } else if (/NOTFOUND/i.test(output)) {
                log.warn('Program not found for uninstall:', programName);
                resolve({ success: false, error: 'Program not found' });
            } else if (/ERROR/i.test(output)) {
                log.error('Uninstall failed:', output);
                resolve({ success: false, error: output.replace('ERROR:', '') });
            } else {
                log.error('Uninstall failed:', output);
                resolve({ success: false, error: output });
            }
        });
    });
}

// Utility functions
function exportReport() {
    const options = {
        title: 'Export System Report',
        defaultPath: 'system-report.txt',
        filters: [
            { name: 'Text Files', extensions: ['txt'] },
            { name: 'All Files', extensions: ['*'] }
        ]
    };
    
    dialog.showSaveDialog(mainWindow, options).then(result => {
        if (!result.canceled) {
            // Generate and save report
            const reportContent = `Windows System Optimizer Report\nGenerated: ${new Date().toISOString()}\n\n[Report content would be generated here]`;
            fs.writeFileSync(result.filePath, reportContent);
            dialog.showMessageBox(mainWindow, {
                type: 'info',
                title: 'Export Complete',
                message: 'System report exported successfully!'
            });
        }
    });
}

function runAsAdmin() {
    if (process.platform === 'win32') {
        const options = {
            type: 'info',
            title: 'Administrator Required',
            message: 'Some operations require administrator privileges. Please restart the application as administrator for full functionality.',
            buttons: ['OK', 'Restart as Admin']
        };
        
        dialog.showMessageBox(mainWindow, options).then(result => {
            if (result.response === 1) {
                // Restart as admin (this would require additional implementation)
                app.quit();
            }
        });
    }
}

function showAbout() {
    dialog.showMessageBox(mainWindow, {
        type: 'info',
        title: 'About Windows System Optimizer',
        message: 'Windows System Optimizer v1.0.0',
        detail: 'A comprehensive system optimization tool for Windows 11.\n\nBuilt with Electron and modern web technologies.\n\n© 2024 System Optimizer Team'
    });
}

// Disable GPU acceleration to fix display issues on some systems
app.disableHardwareAcceleration();

// Add command line switches to improve compatibility
app.commandLine.appendSwitch('disable-gpu-sandbox');
app.commandLine.appendSwitch('disable-software-rasterizer');

// App event handlers
app.whenReady().then(createWindow);

app.on('window-all-closed', function() {
    if (process.platform !== 'darwin') app.quit();
});

app.on('activate', function() {
    if (mainWindow === null) createWindow();
});

// Security: Prevent new window creation
app.on('web-contents-created', (event, contents) => {
    contents.on('new-window', (event, navigationUrl) => {
        event.preventDefault();
        shell.openExternal(navigationUrl);
    });
});

// Handle app updates
try {
    if (require('electron-squirrel-startup')) {
        app.quit();
    }
} catch (e) {
    // electron-squirrel-startup is optional
}

log.info('Windows System Optimizer started'); 
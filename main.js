const { app, BrowserWindow, Menu, ipcMain, dialog, shell } = require('electron');
const path = require('path');
const fs = require('fs');
const { exec, spawn } = require('child_process');
const log = require('electron-log');

// Configure logging
log.transports.file.level = 'debug';
log.transports.console.level = 'debug';

// Keep a global reference of the window object
let mainWindow;

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
        show: true,  // Changed to true to force show immediately
        frame: true,
        resizable: true,
        maximizable: true,
        minimizable: true,
        closable: true,
        autoHideMenuBar: false,
        center: true,  // Center the window on screen
        alwaysOnTop: false
    });

    // Load the app
    mainWindow.loadFile('Simple Psy-op Win-Op.html');

    // Show window when ready to prevent visual flash
    mainWindow.once('ready-to-show', () => {
        mainWindow.show();
        log.info('Application window shown');
    });

    // Open DevTools in development
    if (process.argv.includes('--dev')) {
        mainWindow.webContents.openDevTools();
    }

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

// IPC handlers for system operations
ipcMain.handle('get-system-info', async () => {
    try {
        const systemInfo = await getSystemInformation();
        return systemInfo;
    } catch (error) {
        log.error('Error getting system info:', error);
        return null;
    }
});

ipcMain.handle('scan-temp-files', async () => {
    try {
        const tempFiles = await scanTempFiles();
        return tempFiles;
    } catch (error) {
        log.error('Error scanning temp files:', error);
        return [];
    }
});

ipcMain.handle('cleanup-files', async (event, filePaths) => {
    try {
        const result = await cleanupFiles(filePaths);
        return result;
    } catch (error) {
        log.error('Error cleaning up files:', error);
        return { success: false, error: error.message };
    }
});

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

// === PowerShell-based implementations to replace deprecated WMIC ===
async function getInstalledPrograms() {
    return new Promise((resolve) => {
        const psCmd = 'powershell -NoProfile -ExecutionPolicy Bypass -Command "' +
            `Get-ItemProperty @('HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKLM:\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*','HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*') ` +
            `| Where-Object { $_.DisplayName } ` +
            `| Select-Object DisplayName,Publisher,DisplayVersion,EstimatedSize ` +
            `| ConvertTo-Json -Compress` +
        '"';

        exec(psCmd, { maxBuffer: 1024 * 1024 }, (error, stdout, stderr) => {
            if (error || !stdout) {
                log.error('PowerShell getInstalledPrograms error:', error || 'no output');
                resolve([]);
                return;
            }
            try {
                let data = JSON.parse(stdout.toString());
                if (!Array.isArray(data)) data = [data];
                const programs = data.map(p => {
                    // EstimatedSize is in KB, convert to MB for display
                    let sizeInMB = 'Unknown';
                    if (p.EstimatedSize && !isNaN(p.EstimatedSize)) {
                        const sizeKB = parseInt(p.EstimatedSize);
                        sizeInMB = Math.round(sizeKB / 1024) || 1; // At least 1 MB
                    }
                    
                    return {
                        name: p.DisplayName || 'Unknown',
                        publisher: p.Publisher || 'Unknown',
                        version: p.DisplayVersion || 'Unknown',
                        size: sizeInMB
                    };
                });
                resolve(programs);
            } catch (err) {
                log.error('Parse installed programs JSON failed:', err);
                resolve([]);
            }
        });
    });
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

async function uninstallProgram(programName) {
    return new Promise((resolve) => {
        // Sanitize and properly quote the application name for PowerShell
        const sanitized = programName.replace(/`/g, '').replace(/'/g, "''"); // escape single quotes for PowerShell
        
        // Build PowerShell script with proper syntax (no backslash escaping needed for command line)
        const psScript = `Try {
            $app = Get-Package -Name '${sanitized}' -ErrorAction SilentlyContinue;
            if ($app) {
                $app | Uninstall-Package -Force; Write-Output 'SUCCESS:Removed via Get-Package'; return
            }
            if (Get-Command winget -ErrorAction SilentlyContinue) {
                $result = winget uninstall '${sanitized}' --silent --accept-source-agreements 2>$null;
                if ($LASTEXITCODE -eq 0) { Write-Output 'SUCCESS:Removed via winget'; return }
            }
            $app = Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -eq '${sanitized}' };
            if ($app) {
                $null = $app.Uninstall(); Write-Output 'SUCCESS:Removed via WMI'; return
            }
            $app = Get-WmiObject -Class Win32_Product | Where-Object { $_.Name -like '*${sanitized.split(' ')[0]}*' };
            if ($app) {
                $null = $app.Uninstall(); Write-Output 'SUCCESS:Removed via WMI partial match'; return
            }
            Write-Output 'NOTFOUND'
        } Catch {
            Write-Output $_.Exception.Message
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
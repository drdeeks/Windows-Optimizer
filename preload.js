const { contextBridge, ipcRenderer } = require('electron');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
    // System information
    getSystemInfo: () => ipcRenderer.invoke('get-system-info'),
    getSystemStats: () => ipcRenderer.invoke('get-system-stats'),
    
    // File operations
    scanTempFiles: () => ipcRenderer.invoke('scan-temp-files'),
    cleanupFiles: (filePaths) => ipcRenderer.invoke('cleanup-files', filePaths),
    
    // Enhanced bloatware scanning
    scanBloatwareEnhanced: () => ipcRenderer.invoke('scan-bloatware-enhanced'),
    
    // Enhanced duplicate file scanning
    scanDuplicateFilesEnhanced: (scanPath) => ipcRenderer.invoke('scan-duplicate-files-enhanced', scanPath),
    
    // Registry cleanup
    cleanupRegistry: (appName) => ipcRenderer.invoke('cleanup-registry', appName),
    
    // Enhanced space saver operations
    purgeRecycleBin: () => ipcRenderer.invoke('purge-recycle-bin'),
    clearAllTempData: () => ipcRenderer.invoke('clear-all-temp-data'),
    mergeDuplicateFiles: (duplicateGroups, mergeStrategy) => ipcRenderer.invoke('merge-duplicate-files', duplicateGroups, mergeStrategy),
    
    // Program management
    getInstalledPrograms: () => ipcRenderer.invoke('get-installed-programs'),
    uninstallProgram: (programName) => ipcRenderer.invoke('uninstall-program', programName),
    
    // Startup management
    getStartupItems: () => ipcRenderer.invoke('get-startup-items'),
    toggleStartupItem: (itemName, enable) => ipcRenderer.invoke('toggle-startup-item', itemName, enable),
    
    // Utility functions
    openExternal: (url) => ipcRenderer.invoke('open-external', url),
    showMessageBox: (options) => ipcRenderer.invoke('show-message-box', options),
    browseFolder: () => ipcRenderer.invoke('browse-folder'),
    
    // App information
    getAppVersion: () => ipcRenderer.invoke('get-app-version'),
    
    // Notifications
    showNotification: (title, body) => ipcRenderer.invoke('show-notification', title, body),
    
    // Legacy functions for backward compatibility
    scanDuplicateFiles: (scanPath) => ipcRenderer.invoke('scan-duplicate-files', scanPath),
    clearRecycleBin: () => ipcRenderer.invoke('clear-recycle-bin'),
    clearAllTempFiles: () => ipcRenderer.invoke('clear-all-temp-files')
});

// Enhanced system integration functions
window.systemOptimizer = {
    // Real system information gathering
    async getRealSystemInfo() {
        try {
            const systemInfo = await window.electronAPI.getSystemInfo();
            return systemInfo || {
                os: 'Windows 11 Pro',
                processor: 'Unknown Processor',
                memory: 'Unknown Memory',
                graphics: 'Unknown Graphics',
                storage: 'Unknown Storage',
                motherboard: 'Unknown Motherboard',
                uptime: 'Unknown Uptime'
            };
        } catch (error) {
            console.error('Error getting real system info:', error);
            return null;
        }
    },
    
    // Real temp file scanning
    async scanRealTempFiles() {
        try {
            const tempFiles = await window.electronAPI.scanTempFiles();
            return tempFiles.map(file => ({
                name: file.name,
                path: file.path,
                size: file.size,
                type: 'Temporary File',
                description: `File in temp directory: ${file.path}`,
                lastModified: file.modified
            }));
        } catch (error) {
            console.error('Error scanning real temp files:', error);
            return [];
        }
    },
    
    // Real file cleanup
    async cleanupRealFiles(filePaths) {
        try {
            const result = await window.electronAPI.cleanupFiles(filePaths);
            return result;
        } catch (error) {
            console.error('Error cleaning up real files:', error);
            return { success: false, error: error.message };
        }
    },
    
    // Real startup items
    async getRealStartupItems() {
        try {
            const startupItems = await window.electronAPI.getStartupItems();
            return startupItems.map(item => ({
                name: item.name,
                publisher: 'Unknown',
                status: item.enabled ? 'Enabled' : 'Disabled',
                impact: 'Medium',
                command: item.command,
                location: item.location
            }));
        } catch (error) {
            console.error('Error getting real startup items:', error);
            return [];
        }
    },
    
    // Toggle startup item
    async toggleRealStartupItem(itemName, enable) {
        try {
            const result = await window.electronAPI.toggleStartupItem(itemName, enable);
            return result;
        } catch (error) {
            console.error('Error toggling startup item:', error);
            return { success: false, error: error.message };
        }
    }
};

// Enhance the existing functions in the HTML with real system integration
document.addEventListener('DOMContentLoaded', function() {
    // Override the mock functions with real ones
    if (window.systemOptimizer) {
        // Replace loadSystemInfo with real system info
        window.loadSystemInfo = async function() {
            const realSystemInfo = await window.systemOptimizer.getRealSystemInfo();
            if (realSystemInfo) {
                systemInfo = realSystemInfo;
                displaySystemInfo();
            }
            updateSystemStats();
        };
        
        // Replace scanForFiles with real temp file scanning
        window.scanForFiles = async function() {
            updateProgress(0);
            const progressBar = document.getElementById('cleanup-progress');
            const statusElement = document.getElementById('cleanup-status');
            
            statusElement.textContent = 'Scanning system files...';
            
            try {
                // Simulate progress
                for (let i = 0; i <= 100; i += 10) {
                    updateProgress(i);
                    await new Promise(resolve => setTimeout(resolve, 100));
                }
                
                const realTempFiles = await window.systemOptimizer.scanRealTempFiles();
                
                // Update the cleanup items with real files
                const container = document.getElementById('cleanup-items');
                container.innerHTML = '';
                
                let totalSize = 0;
                realTempFiles.forEach(file => {
                    totalSize += file.size;
                    const item = createCleanupItem(file.name, file.description, formatFileSize(file.size));
                    container.appendChild(item);
                });
                
                statusElement.textContent = `Scan complete! Found ${realTempFiles.length} items (${formatFileSize(totalSize)})`;
                
                // Show notification
                if (window.electronAPI && window.electronAPI.showNotification) {
                    window.electronAPI.showNotification('Scan Complete', `Found ${realTempFiles.length} temporary files`);
                }
                
            } catch (error) {
                console.error('Error during real file scan:', error);
                statusElement.textContent = 'Error occurred during scan';
            }
        };
    }
});

// Utility function to format file sizes
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

console.log('Preload script loaded successfully'); 
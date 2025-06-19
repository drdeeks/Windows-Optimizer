# Changelog

All notable changes to Windows System Optimizer will be documented in this file.

## [1.1.0] - 2024-12-19

### 🔥 Major Enhancements

#### Enhanced Bloatware Detection
- **4x More Categories**: Now detects and categorizes apps across 4 comprehensive categories:
  - Pre-installed Apps & Store Apps (games, unnecessary Windows apps)
  - Third-party Software (trials, adware, potentially unwanted programs)
  - Browser Extensions & Add-ons (toolbars, suspicious extensions)
  - Legacy/Outdated Software (deprecated programs, security risks)

- **Smart Detection Engine**: Identifies known bloatware patterns automatically using comprehensive databases
- **Risk Assessment System**: Each app rated as Safe, Medium, or High risk for removal
- **Intelligent Recommendations**: Clear guidance (Remove, Review, Keep, Update or Remove)
- **Persistent State Management**: Removed apps won't reappear in future scans (fixes major user complaint)

#### Improved User Experience
- **Better Layout**: All control buttons moved above detected files for intuitive workflow
- **Smart Selection Controls**:
  - "Select Recommended" button automatically chooses apps marked safe for removal
  - Enhanced Select All/Deselect All functionality
- **Enhanced Statistics Dashboard**: Real-time stats showing total apps, bloatware count, selected items, and potential space freed
- **Color-Coded Interface**: Visual risk levels and recommendations for easy decision making
- **Enhanced Confirmations**: Detailed removal confirmations with app lists

### 🧹 Disk Cleanup Improvements
- **Consistent Layout**: Moved control buttons above detected files for consistency
- **Better Visual Hierarchy**: Improved spacing and organization

### 🛠️ Technical Improvements
- **Enhanced Data Structures**: Better app categorization and tracking
- **Improved Memory Management**: Tracks removed apps to prevent reappearing
- **Better Error Handling**: More robust uninstall processes with detailed logging

### 🐛 Bug Fixes
- **Fixed**: Apps reappearing after removal on subsequent scans
- **Fixed**: Inconsistent button placement across different tabs
- **Improved**: App detection accuracy and categorization

---

## [1.0.0] - 2024-12-18

### Initial Release
- Basic disk cleanup functionality
- Simple bloatware detection
- Startup manager
- System information display
- Modern Windows 11-styled UI 
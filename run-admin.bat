@echo off
cd /d %~dp0

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Running as Administrator - Full functionality available
) else (
    echo.
    echo ============================================
    echo   ADMINISTRATOR PRIVILEGES REQUIRED
    echo ============================================
    echo.
    echo This application requires administrator privileges
    echo for full system cleanup functionality.
    echo.
    echo Please right-click this batch file and select
    echo "Run as administrator"
    echo.
    pause
    exit /b 1
)

echo ======================================
echo   Windows System Optimizer - ADMIN
echo ======================================
echo.
echo Administrator mode detected
echo Full system access enabled
echo Current directory: %CD%
echo.

REM Check if executable exists
if not exist "build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe" (
    echo ERROR: Executable not found!
    echo.
    echo Building application...
    call npm run build-simple
    echo.
    if not exist "build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe" (
        echo Build failed! Please check the errors above.
        pause
        exit /b 1
    )
)

echo Starting Windows System Optimizer with full privileges...
echo.

REM Change to executable directory and run
cd "build\windows-system-optimizer-win32-x64"

REM Run with specific flags to prevent crashes
echo Starting application with admin privileges...
start "" "windows-system-optimizer.exe" --disable-gpu-sandbox --no-sandbox --disable-dev-shm-usage

echo.
echo Application started with administrator privileges.
echo If you experience any issues, check the application logs.
echo.
pause 
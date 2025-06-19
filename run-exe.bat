@echo off
cd /d %~dp0
echo ======================================
echo   Windows System Optimizer - EXE
echo ======================================
echo.
echo Starting the executable version...
echo For full functionality, run as Administrator.
echo Current directory: %CD%
echo.

REM Check if the executable exists
if not exist "build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe" (
    echo ERROR: Executable not found!
    echo Expected path: build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe
    echo.
    echo Attempting to build the application...
    npm run build-simple
    echo.
    if not exist "build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe" (
        echo Build failed! Please check the errors above.
        pause
        exit /b 1
    )
)

echo Executable found. Starting application...
echo.
cd "build\windows-system-optimizer-win32-x64"
start "" "windows-system-optimizer.exe"
echo Application started. If no window appears, try running as Administrator.
pause 
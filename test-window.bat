@echo off
echo ======================================
echo   Windows System Optimizer - DEBUG
echo ======================================
echo.
echo Testing the executable version...
echo Current directory: %CD%
echo.

REM Check if the executable exists
if not exist "build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe" (
    echo ERROR: Executable not found!
    echo Expected path: build\windows-system-optimizer-win32-x64\windows-system-optimizer.exe
    echo.
    echo Please run: npm run build-simple
    pause
    exit /b 1
)

echo Executable found. Starting application...
echo.

REM Change to the executable directory
cd "build\windows-system-optimizer-win32-x64"

REM Try to start the application with error capture
echo Starting windows-system-optimizer.exe...
start /wait "" "windows-system-optimizer.exe"

echo.
echo Application has closed.
echo Check if a window appeared. If not, there may be display issues.
echo.
pause 
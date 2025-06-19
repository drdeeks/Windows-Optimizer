@echo off
echo =====================================
echo   Windows System Optimizer
echo   Distribution Preparation
echo =====================================
echo.

REM Create a distribution directory
if not exist "Windows-System-Optimizer-Portable" mkdir "Windows-System-Optimizer-Portable"

echo Copying executable and dependencies...
xcopy "build\windows-system-optimizer-win32-x64\*" "Windows-System-Optimizer-Portable\" /E /Y /Q

echo.
echo Creating run script...
echo @echo off > "Windows-System-Optimizer-Portable\run.bat"
echo echo Starting Windows System Optimizer... >> "Windows-System-Optimizer-Portable\run.bat"
echo echo For full functionality, run as Administrator. >> "Windows-System-Optimizer-Portable\run.bat"
echo echo. >> "Windows-System-Optimizer-Portable\run.bat"
echo .\windows-system-optimizer.exe >> "Windows-System-Optimizer-Portable\run.bat"

echo.
echo =======================================
echo   DISTRIBUTION READY!
echo =======================================
echo.
echo Portable version created in:
echo Windows-System-Optimizer-Portable\
echo.
echo To distribute:
echo 1. Zip the Windows-System-Optimizer-Portable folder
echo 2. Send to users - no installation required!
echo 3. Users just need to extract and run run.bat
echo.
echo File size: ~165MB (includes all dependencies)
echo Requirements: Windows 10/11 only
echo.
pause 
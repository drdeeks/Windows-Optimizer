@echo off
echo Building Windows System Optimizer v2.0 - Production Release
echo.

REM Check if Node.js is installed
node --version >nul 2>&1
if errorlevel 1 (
    echo Error: Node.js is not installed or not in PATH
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo Error: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Check if Tauri CLI is installed
cargo tauri --version >nul 2>&1
if errorlevel 1 (
    echo Installing Tauri CLI...
    cargo install tauri-cli
)

REM Install npm dependencies
echo Installing dependencies...
npm install

REM Clean previous builds
echo Cleaning previous builds...
if exist "src-tauri\target" rmdir /s /q "src-tauri\target"

REM Build production version
echo Building production version...
npm run build

if errorlevel 1 (
    echo Build failed!
    pause
    exit /b 1
)

echo.
echo Build completed successfully!
echo Executable location: src-tauri\target\release\windows-system-optimizer.exe
echo.

REM Check if executable was created
if exist "src-tauri\target\release\windows-system-optimizer.exe" (
    echo Build verification: SUCCESS
    echo File size: 
    for %%A in ("src-tauri\target\release\windows-system-optimizer.exe") do echo %%~zA bytes
) else (
    echo Build verification: FAILED
    echo Executable not found!
)

pause
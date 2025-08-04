@echo off
echo Starting Windows System Optimizer v2.0 - Development Mode
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

REM Start development mode
echo Starting development server...
npm run dev

pause
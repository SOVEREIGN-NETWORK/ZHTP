@echo off
echo Building ZHTP Network Installer...
echo.

:: Check if Rust is installed
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Rust is not installed.
    echo Please install Rust from: https://rustup.rs/
    pause
    exit /b 1
)

:: Check if cargo-tauri is installed
where cargo-tauri >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Tauri CLI...
    cargo install tauri-cli
)

:: Clean previous builds
echo Cleaning previous builds...
cargo clean

:: Build the installer
echo Building ZHTP Installer...
echo This may take a few minutes for the first build...
cargo tauri build

if %errorlevel% equ 0 (
    echo.
    echo ============================================
    echo SUCCESS! ZHTP Installer built successfully
    echo ============================================
    echo.
    echo Your installer is located at:
    echo target\release\bundle\msi\ZHTP Network Installer_1.0.0_x64_en-US.msi
    echo.
    echo You can now distribute this installer to users!
    echo.
) else (
    echo.
    echo ==========================================
    echo ERROR: Build failed
    echo ==========================================
    echo.
    echo Please check the error messages above.
    echo.
)

pause

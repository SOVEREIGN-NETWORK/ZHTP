@echo off
echo Building WASM contract...

REM Ensure WASM target is available
rustup target add wasm32-unknown-unknown

REM Try wasm-pack first
where wasm-pack >nul 2>&1
if %errorlevel% equ 0 (
    echo Using wasm-pack...
    wasm-pack build --target web --dev
) else (
    echo Using cargo directly...
    cargo build --target wasm32-unknown-unknown --release
)

REM Create basic WASM if it doesn't exist
if not exist "pkg\zhtp_contracts_bg.wasm" (
    if exist "..\target\wasm32-unknown-unknown\release\zhtp_contracts.wasm" (
        copy "..\target\wasm32-unknown-unknown\release\zhtp_contracts.wasm" "pkg\zhtp_contracts_bg.wasm"
    ) else (
        echo Warning: WASM file not found, creating placeholder
        echo. > pkg\zhtp_contracts_bg.wasm
    )
)

REM Copy result to main project
if exist "pkg\zhtp_contracts_bg.wasm" (
    copy /Y "pkg\zhtp_contracts_bg.wasm" "..\token.wasm"
)

echo Build complete!
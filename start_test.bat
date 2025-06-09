@echo off
echo Building ZHTP Network...

REM Build WASM contract
cd contracts
cargo build --release
cd ..

REM Build main project
cargo build --release

REM Run single node test
echo Starting test node...
start "ZHTP Node" cargo run --example discovery_node

REM Wait a moment
timeout /t 5

REM Open browser interface
echo Opening browser interface...
start browser/index.html

echo.
echo Test network is running!
echo Press Ctrl+C to exit
pause
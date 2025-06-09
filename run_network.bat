@echo off
echo Starting ZHTP Decentralized Network...

REM Build contracts first
cd contracts
call build.bat
cd ..

REM Start discovery node in new window
start cmd /k "cargo run --example discovery_node"

REM Wait for discovery node to start
timeout /t 5

REM Start decentralized app in new window
start cmd /k "cargo run --example decentralized_app"

REM Wait for app to start
timeout /t 3

REM Run integration tests
echo Running system tests...
cargo test --test integration_test -- --nocapture

echo.
echo Network is running! Access the browser interface at browser/index.html
echo Press any key to shut down the network...
pause

REM Kill all network processes
taskkill /F /IM discovery_node.exe
taskkill /F /IM decentralized_app.exe
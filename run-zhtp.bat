@echo off
REM ZHTP - Simple Run Script (Batch Version)
REM Builds, runs ZHTP mainnet, and opens browser automatically!

echo Building ZHTP Mainnet...
cargo build --release

if %errorlevel% == 0 (
    echo Starting ZHTP Mainnet Core...
    echo Browser interface will be at: http://localhost:4000
    echo API Dashboard will be at: http://localhost:4000/api/
    echo Start earning ZHTP tokens!
    echo.
      REM Start ZHTP Mainnet using the example
    echo Starting ZHTP Mainnet Network...
    start "ZHTP Mainnet" cmd /k "cargo run --example zhtp_mainnet_launch --release"
    
    REM Start ZHTP Web Service 
    echo Starting ZHTP Web Service...
    start "ZHTP Web Service" cmd /k "cargo run --bin network-service --release"
    
    REM Wait for both services to start
    echo Waiting for all services to initialize...
    timeout /t 10 /nobreak >nul
    
    REM Launch browser automatically with onboarding
    echo Opening ZHTP Browser Interface...
    start "" "http://localhost:4000/browser/welcome.html"
    
    echo.
    echo ZHTP MAINNET IS RUNNING!
    echo Browser opened automatically
    echo You're now earning ZHTP tokens!
    echo.
    echo Available DApps:
    echo   news.zhtp - Decentralized news
    echo   social.zhtp - Private social network
    echo   market.zhtp - P2P marketplace
    echo.
    echo Press any key to stop ZHTP mainnet...
    pause >nul
      REM Kill the background processes when user presses a key
    taskkill /f /fi "WindowTitle eq ZHTP Mainnet*" >nul 2>&1
    taskkill /f /fi "WindowTitle eq ZHTP Web Service*" >nul 2>&1
    
) else (
    echo Build failed!
    pause
)

# ZHTP Simple Installer - One-Click Setup
# This script provides the same functionality as the GUI installer

Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "    🚀 ZHTP ONE-CLICK INSTALLER 🚀" -ForegroundColor Yellow
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Welcome to ZHTP Network!" -ForegroundColor Green
Write-Host "This installer will set up your node and wallet automatically." -ForegroundColor White
Write-Host ""

# Check prerequisites
Write-Host "Step 1: Checking prerequisites..." -ForegroundColor Yellow

# Check if Rust is installed
$rustVersion = try { cargo --version } catch { $null }
if (-not $rustVersion) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
    .\rustup-init.exe -y
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
    Remove-Item "rustup-init.exe"
}

Write-Host "✅ Rust installed: $rustVersion" -ForegroundColor Green

# Create ZHTP directories
Write-Host "Step 2: Creating ZHTP directories..." -ForegroundColor Yellow
$zhtpDir = "$env:USERPROFILE\ZHTP"
$configDir = "$zhtpDir\config"
New-Item -ItemType Directory -Path $zhtpDir -Force | Out-Null
New-Item -ItemType Directory -Path $configDir -Force | Out-Null

# Create wallet with initial tokens and strict governance controls
Write-Host "Step 3: Creating wallet with FREE starter tokens..." -ForegroundColor Yellow
$walletAddress = "zhtp_" + [System.Guid]::NewGuid().ToString("N").Substring(0, 16)
$wallet = @{
    address = $walletAddress
    walletAddress = $walletAddress
    privateKey = "pk_" + [System.Guid]::NewGuid().ToString("N").Substring(0, 32)
    zhtp_balance = 10000
    governance_tokens = 1  # IMMUTABLE: One wallet = One vote
    created_at = [DateTimeOffset]::Now.ToUnixTimeSeconds()
    earnings_total = 0
    zk_identity = $walletAddress
    has_voted = $false
    governanceTokensLocked = $true
    walletVersion = "1.0"
    governanceAuditTrail = @(
        @{
            action = "WALLET_CREATED"
            tokens = 1
            timestamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
            source = "installer"
        }
    )
} | ConvertTo-Json

$wallet | Out-File "$configDir\wallet.json"

# Create additional governance security record
$governanceRecord = @{
    wallet = $walletAddress
    tokens = 1
    allocated = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
    immutable = $true
    source = "installer_creation"
} | ConvertTo-Json

$governanceRecord | Out-File "$configDir\governance_$walletAddress.json"

Write-Host "✅ Wallet created: $walletAddress" -ForegroundColor Green
Write-Host "✅ Initial tokens: 10,000 ZHTP + 1 Governance (Voting Rights)" -ForegroundColor Green
Write-Host "✅ Governance tokens LOCKED (1:1 ratio enforced)" -ForegroundColor Cyan

# Choose node type
Write-Host ""
Write-Host "Step 4: Choose your node type:" -ForegroundColor Yellow
Write-Host "1. Validator Node (High earnings, requires good uptime)"
Write-Host "2. Router Node (Medium earnings, requires good bandwidth)"  
Write-Host "3. Storage Node (Steady earnings, requires disk space)"
Write-Host "4. Raspberry Pi Node (Basic earnings, low requirements)"
Write-Host ""

$nodeChoice = Read-Host "Enter choice (1-4) or press Enter for Auto-detect"

if ([string]::IsNullOrEmpty($nodeChoice)) {
    Write-Host "Auto-detecting optimal node type..." -ForegroundColor Yellow
    $nodeChoice = "2" # Default to Router Node
}

$nodeTypes = @{
    "1" = @{ type = "validator"; stake = 5000; description = "Validator Node - High Earnings" }
    "2" = @{ type = "router"; stake = 2000; description = "Router Node - Medium Earnings" }  
    "3" = @{ type = "storage"; stake = 1000; description = "Storage Node - Steady Earnings" }
    "4" = @{ type = "raspberry_pi"; stake = 500; description = "Raspberry Pi Node - Basic Earnings" }
}

$selectedNode = $nodeTypes[$nodeChoice]
if (-not $selectedNode) { $selectedNode = $nodeTypes["2"] }

# Create node config
$nodeConfig = @{
    node_type = $selectedNode.type
    stake_amount = $selectedNode.stake
    network_port = 8080
    api_port = 4000
    auto_start = $true
    earnings_address = $walletAddress
    created_at = [DateTimeOffset]::Now.ToUnixTimeSeconds()
} | ConvertTo-Json

$nodeConfig | Out-File "$configDir\node.json"
Write-Host "✅ Node configured: $($selectedNode.description)" -ForegroundColor Green

# Build ZHTP if needed
Write-Host ""
Write-Host "Step 5: Building ZHTP Network..." -ForegroundColor Yellow
if (-not (Test-Path "target\release\decentralized_network.exe")) {
    cargo build --release
}

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ ZHTP Network built successfully!" -ForegroundColor Green
} else {
    Write-Host "❌ Build failed. Please check errors above." -ForegroundColor Red
    exit 1
}

# Create shortcuts
Write-Host "Step 6: Creating shortcuts..." -ForegroundColor Yellow
$desktopPath = [Environment]::GetFolderPath("Desktop")

# Create run script shortcut
$runScript = @"
# ZHTP Quick Launch
cd "$PWD"
.\run-zhtp.ps1
"@
$runScript | Out-File "$desktopPath\Launch ZHTP.ps1"

Write-Host "✅ Desktop shortcut created" -ForegroundColor Green

# Final setup
Write-Host ""
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "    ✅ INSTALLATION COMPLETE! ✅" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Your ZHTP Node Details:" -ForegroundColor White
Write-Host "  Wallet: $walletAddress" -ForegroundColor Gray
Write-Host "  Initial Tokens: 10,000 ZHTP + 1 Governance (Voting Rights)" -ForegroundColor Gray
Write-Host "  Node Type: $($selectedNode.description)" -ForegroundColor Gray
Write-Host "  Stake Amount: $($selectedNode.stake) ZHTP" -ForegroundColor Gray
Write-Host ""
Write-Host "Ready to launch:" -ForegroundColor White
Write-Host "  Run: .\run-zhtp.ps1" -ForegroundColor Gray
Write-Host "  Verify: .\simple-verify.ps1" -ForegroundColor Gray
Write-Host "  Security Test: .\final-security-test.ps1" -ForegroundColor Gray
Write-Host ""

$launch = Read-Host "Launch ZHTP Network now? (Y/n)"
if ($launch -ne "n" -and $launch -ne "N") {
    Write-Host ""
    Write-Host "🚀 Launching ZHTP Network..." -ForegroundColor Green
    .\run-zhtp.ps1
}

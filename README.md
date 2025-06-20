# ZHTP Network - Decentralized Internet

**Join the decentralized web revolution with ZHTP Network**

ZHTP is a complete decentralized internet replacement with built-in privacy, governance, and economic incentives. No central authorities, no surveillance, no censorship.

## Quick Start (Windows)

```powershell
# Build and run ZHTP Network
.\run-zhtp.bat
```

## Quick Start (Linux/macOS)

```bash
# Setup and build
./run-zhtp-linux.sh
```

Both options will:
1. Build the ZHTP network
2. Start all services automatically  
3. Open your browser to `http://localhost:4000/browser/welcome.html`
4. Guide you through wallet creation and onboarding

## What You Get

### Decentralized Features
- **Private Browsing** - No tracking, surveillance, or data collection
- **Blockchain DNS** - Decentralized domain resolution
- **Zero-Knowledge Identity** - Anonymous but verifiable identity
- **Post-Quantum Security** - Future-proof cryptography

### Economic System
- **Earn ZHTP Tokens** - Get rewarded for running a node
- **Fair Governance** - One wallet = one governance token = one vote
- **No Registration** - No accounts, no personal information required

### Network Participation
- **Browse DApps** - Visit decentralized websites and applications
- **Vote on Proposals** - Help govern the network democratically
- **Run Applications** - Deploy your own decentralized apps

## First Time Setup

When you first launch ZHTP, you'll be guided through:

1. **Wallet Creation** - Secure local wallet with private key
2. **Identity Generation** - Zero-knowledge identity system
3. **Governance Token** - Receive exactly 1 governance token (one person, one vote)
4. **Node Configuration** - Choose your participation level

## Available Services

After setup, you can access:
- **Main Interface**: `http://localhost:4000/browser/`
- **Governance Dashboard**: Vote on network proposals
- **Wallet Management**: Check balances and transactions
- **Network Status**: Monitor your node and earnings

## Governance System

ZHTP uses democratic governance where:
- **Every wallet gets exactly 1 governance token**
- **One person = one vote** (no buying extra influence)
- **All votes are transparent** but voter identity is private
- **Network upgrades require community approval**

## Troubleshooting

### Common Issues

**"Failed to build"**
- Ensure Rust is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Run `rustup update` to get the latest version

**"Port already in use"**
- Check if ZHTP is already running
- Try `netstat -ano | findstr :4000` (Windows) or `lsof -i :4000` (Linux)

**"Browser doesn't open"**
- Manually navigate to `http://localhost:4000/browser/welcome.html`
- Check your firewall settings

### Getting Help
- **Community Forum**: Join discussions with other users
- **Discord**: Real-time chat and support
- **GitHub Issues**: Report bugs and request features

## For Developers

### Building from Source
```bash
# Clone repository
git clone <repository-url>
cd zhtp

# Install dependencies and build
cargo build --release

# Run development version
cargo run
```

### Key Components
- **Core Network**: Decentralized consensus and networking
- **Browser Interface**: Web-based user interface
- **Governance System**: Democratic decision making
- **Identity System**: Zero-knowledge identity management

## Security & Privacy

- **End-to-End Encryption** - All communications encrypted
- **Local Key Storage** - Private keys never leave your device
- **Anonymous Routing** - Your activity is private by default
- **Zero Data Collection** - No tracking, analytics, or profiling

## Network Economics

- **Node Rewards** - Earn tokens for participating in the network
- **Transaction Fees** - Minimal fees go to node operators
- **Governance Participation** - Vote on network parameters
- **Fair Distribution** - No pre-mine, everyone starts equal

## Join the Revolution

ZHTP represents a fundamental shift toward user-controlled internet infrastructure. By joining, you're helping build a more open, private, and democratic web for everyone.

**Ready to start?** Run `.\run-zhtp.bat` (Windows) or `./run-zhtp-linux.sh` (Linux) and join the decentralized future!

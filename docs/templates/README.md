# 🎨 DApp Templates

**Ready-to-use templates for building DApps on ZHTP**

This directory contains production-ready templates that you can use as starting points for your ZHTP DApps. Each template includes complete smart contracts, frontend code, deployment scripts, and documentation.

## 📋 Available Templates

### 🗳️ [Voting DApp](./voting-dao/)
A complete decentralized voting and governance application.
- **Features**: Proposal creation, voting, delegation, treasury management
- **Technologies**: Solidity, React, ZHTP SDK
- **Use Cases**: DAOs, community governance, corporate voting

### 🛒 [Marketplace DApp](./nft-marketplace/)
A full-featured NFT marketplace with zero-knowledge privacy features.
- **Features**: NFT minting, trading, auctions, royalties, private sales
- **Technologies**: Solidity, React, IPFS, ZK proofs
- **Use Cases**: Digital art, gaming assets, collectibles

### 💰 [DeFi Platform](./defi-platform/)
A comprehensive DeFi platform with staking, lending, and yield farming.
- **Features**: Token staking, liquidity pools, yield farming, governance
- **Technologies**: Solidity, Vue.js, ZHTP SDK
- **Use Cases**: Yield farming, decentralized finance, token economics

### 🎮 [Gaming Platform](./gaming-platform/)
A blockchain-based gaming platform with in-game assets and rewards.
- **Features**: Character NFTs, in-game currency, achievements, tournaments
- **Technologies**: Solidity, Unity integration, ZHTP SDK
- **Use Cases**: P2E games, virtual worlds, gaming guilds

### 📚 [Learning Management](./learning-platform/)
A decentralized learning platform with credential verification.
- **Features**: Course creation, certification, skill verification, payments
- **Technologies**: Solidity, React, ZK credentials
- **Use Cases**: Online education, professional certification, skill verification

### 🔐 [Identity Management](./identity-platform/)
Zero-knowledge identity and credential management system.
- **Features**: Self-sovereign identity, credential issuance, privacy-preserving verification
- **Technologies**: Solidity, ZK circuits, React
- **Use Cases**: Digital identity, credential verification, privacy-first apps

### 💬 [Social Platform](./social-platform/)
Decentralized social media with privacy and content ownership.
- **Features**: Private messaging, content sharing, reputation system, monetization
- **Technologies**: Solidity, React, IPFS, ZK messaging
- **Use Cases**: Social networks, content platforms, community building

### 🏥 [Healthcare Records](./healthcare-platform/)
Secure, privacy-preserving healthcare record management.
- **Features**: Medical records, access control, privacy proofs, insurance integration
- **Technologies**: Solidity, React, ZK proofs, encryption
- **Use Cases**: Medical records, telemedicine, health insurance

## 🚀 Quick Start

### Option 1: Clone Template
```bash
# Clone a specific template
git clone https://github.com/zhtp-network/dapp-templates.git
cd dapp-templates/voting-dao

# Install dependencies
npm install

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Start development server
npm run dev
```

### Option 2: Use ZHTP CLI
```bash
# Install ZHTP CLI
npm install -g @zhtp/cli

# Create new DApp from template
zhtp create my-dapp --template=voting-dao
cd my-dapp

# Start development
npm run dev
```

### Option 3: Use Create ZHTP App
```bash
# Create with interactive selection
npx create-zhtp-app my-dapp

# Follow the prompts to select a template
```

## 🛠️ Template Structure

Each template follows a consistent structure:

```
template-name/
├── contracts/              # Smart contracts
│   ├── src/               # Contract source code
│   ├── test/              # Contract tests
│   ├── scripts/           # Deployment scripts
│   └── artifacts/         # Compiled contracts
├── frontend/              # Frontend application
│   ├── src/               # Source code
│   ├── public/            # Static assets
│   ├── components/        # Reusable components
│   └── pages/             # Application pages
├── docs/                  # Documentation
│   ├── README.md          # Template overview
│   ├── SETUP.md           # Setup instructions
│   ├── DEPLOYMENT.md      # Deployment guide
│   └── API.md             # API documentation
├── scripts/               # Build and deployment scripts
├── tests/                 # Integration tests
├── .env.example           # Environment variables template
├── package.json           # Dependencies and scripts
├── zhtp.config.js         # ZHTP configuration
└── README.md              # Quick start guide
```

## 📖 Template Documentation

### Common Files

Each template includes:

- **README.md**: Overview, features, and quick start
- **SETUP.md**: Detailed setup instructions
- **DEPLOYMENT.md**: Production deployment guide
- **API.md**: Smart contract and API documentation
- **ARCHITECTURE.md**: Technical architecture overview

### Configuration Files

- **zhtp.config.js**: ZHTP network and deployment configuration
- **.env.example**: Environment variables template
- **package.json**: Dependencies and npm scripts
- **hardhat.config.js**: Smart contract compilation and testing

## 🎯 Customization Guide

### 1. Configuration
```javascript
// zhtp.config.js
module.exports = {
  dapp: {
    name: "My Custom DApp",
    version: "1.0.0",
    description: "Description of my DApp",
    author: "your-email@example.com"
  },
  networks: {
    // Network configurations
  },
  features: {
    // Enable/disable template features
    voting: true,
    staking: false,
    governance: true
  }
};
```

### 2. Smart Contract Customization
```solidity
// contracts/src/CustomContract.sol
contract CustomContract is BaseTemplate {
    // Override template behavior
    function customFunction() public override {
        // Your custom logic
    }
    
    // Add new features
    function newFeature() public {
        // New functionality
    }
}
```

### 3. Frontend Customization
```javascript
// frontend/src/config/customization.js
export const customConfig = {
  branding: {
    name: "My DApp",
    logo: "/path/to/logo.png",
    colors: {
      primary: "#007bff",
      secondary: "#6c757d"
    }
  },
  features: {
    enableFeatureX: true,
    enableFeatureY: false
  }
};
```

## 🔧 Development Tools

### Build Scripts
```bash
# Development
npm run dev          # Start development server
npm run build        # Build for production
npm run test         # Run tests

# Smart Contracts
npm run compile      # Compile contracts
npm run test:contracts # Test contracts
npm run deploy       # Deploy contracts

# Deployment
npm run deploy:testnet # Deploy to testnet
npm run deploy:mainnet # Deploy to mainnet
npm run verify       # Verify contracts
```

### Testing
```bash
# Unit tests
npm run test:unit

# Integration tests
npm run test:integration

# End-to-end tests
npm run test:e2e

# All tests
npm run test:all
```

## 🚀 Deployment Options

### Development Deployment
```bash
# Deploy to local ZHTP node
npm run deploy:local

# Deploy to ZHTP testnet
npm run deploy:testnet
```

### Production Deployment
```bash
# Build optimized version
npm run build:production

# Deploy to ZHTP mainnet
npm run deploy:mainnet

# Register domain (optional)
npm run register:domain
```

### Automated Deployment
```yaml
# .github/workflows/deploy.yml
name: Deploy DApp
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '16'
      - name: Install dependencies
        run: npm install
      - name: Build DApp
        run: npm run build
      - name: Deploy to ZHTP
        run: npm run deploy:mainnet
        env:
          PRIVATE_KEY: ${{ secrets.PRIVATE_KEY }}
          ZHTP_API_KEY: ${{ secrets.ZHTP_API_KEY }}
```

## 📚 Learning Resources

### Tutorials
- [Building Your First DApp](./tutorials/first-dapp.md)
- [Smart Contract Development](./tutorials/smart-contracts.md)
- [Frontend Integration](./tutorials/frontend-integration.md)
- [Zero-Knowledge Features](./tutorials/zk-integration.md)

### Video Guides
- **Template Overview**: Introduction to each template
- **Setup Tutorial**: Step-by-step setup process
- **Customization Guide**: How to customize templates
- **Deployment Walkthrough**: Production deployment process

### Community Resources
- **Discord**: Get help from the community
- **GitHub Discussions**: Template-specific discussions
- **Stack Overflow**: Technical Q&A with `zhtp` tag

## 🔗 External Resources

### Development Tools
- [ZHTP SDK Documentation](../api/)
- [Smart Contract Guide](../guides/smart-contracts.md)
- [DApp Development Guide](../guides/dapp-development.md)

### Design Resources
- [UI Component Library](https://github.com/zhtp-network/ui-components)
- [Design System](https://design.zhtp.network)
- [Icon Library](https://icons.zhtp.network)

### Community Templates
- [Community Template Registry](https://templates.zhtp.network)
- [Template Contributions](https://github.com/zhtp-network/community-templates)

## 🤝 Contributing

### Adding New Templates
1. Fork the repository
2. Create your template in a new directory
3. Follow the template structure guidelines
4. Add comprehensive documentation
5. Include tests and examples
6. Submit a pull request

### Template Requirements
- Complete documentation
- Working smart contracts with tests
- Functional frontend with responsive design
- Deployment scripts and configuration
- Zero-knowledge integration (where applicable)
- Security best practices

### Review Process
1. **Technical Review**: Code quality, security, best practices
2. **Documentation Review**: Completeness, clarity, examples
3. **Testing**: Automated tests, manual testing, security audit
4. **Community Feedback**: Community review and feedback
5. **Approval**: Final approval and inclusion in registry

---

## 📞 Support

Need help with templates? Reach out:

- **Documentation**: [Developer Portal](../README.md)
- **Discord**: [ZHTP Developer Community](https://discord.gg/zhtp)
- **GitHub Issues**: [Template Issues](https://github.com/zhtp-network/dapp-templates/issues)
- **Email**: developers@zhtp.network

Happy building! 🚀

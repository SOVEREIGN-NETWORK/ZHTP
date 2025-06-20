# 🚀 ZHTP SDK Documentation

**Comprehensive SDK documentation for all languages and platforms**

## 📚 Official SDKs

### 🟨 JavaScript/TypeScript SDK
The most popular SDK for web development.

- **Framework Support**: React, Vue, Angular, Node.js
- **Package**: `@zhtp/sdk`
- **TypeScript**: Full type definitions included
- **Documentation**: [📖 JavaScript SDK Guide](./javascript.md)

### 🐍 Python SDK  
Perfect for backend services, data science, and AI applications.

- **Framework Support**: Flask, Django, FastAPI, Jupyter
- **Package**: `zhtp-sdk`
- **Async Support**: Full asyncio compatibility  
- **Documentation**: [📖 Python SDK Guide](./python.md)

### 🦀 Rust SDK
High-performance SDK for systems programming and protocol development.

- **Framework Support**: Tokio, Actix, Warp
- **Package**: `zhtp-sdk`
- **Performance**: Zero-copy networking and memory-safe operations
- **Documentation**: [📖 Rust SDK Guide](./rust.md)

### 🌐 REST API
Language-agnostic HTTP API for any platform.

- **Protocol**: Standard HTTP/HTTPS
- **Authentication**: API keys, JWT tokens
- **Rate Limiting**: Built-in protection
- **Documentation**: [📖 REST API Guide](./rest.md)

---

## 🔧 SDK Feature Comparison

| Feature | JavaScript | Python | Rust | REST API |
|---------|------------|--------|------|----------|
| ✅ Wallet Connection | ✅ | ✅ | ✅ | ✅ |
| ✅ Smart Contracts | ✅ | ✅ | ✅ | ✅ |
| ✅ Zero-Knowledge Proofs | ✅ | ✅ | ✅ | ✅ |
| ✅ DNS Resolution | ✅ | ✅ | ✅ | ✅ |
| ✅ File Storage | ✅ | ✅ | ✅ | ✅ |
| ✅ Real-time Events | ✅ | ✅ | ✅ | ⚠️ Webhooks |
| ✅ UI Components | ✅ | ❌ | ❌ | ❌ |
| ✅ Mobile Support | ✅ | ❌ | ❌ | ✅ |
| ✅ Performance | Good | Good | Excellent | Good |
| ✅ Learning Curve | Easy | Easy | Medium | Easy |

---

## 🎯 Quick Start Examples

### JavaScript/TypeScript
```javascript
import { ZHTPClient, Wallet } from '@zhtp/sdk';

// Connect to ZHTP network
const client = new ZHTPClient({ network: 'mainnet' });

// Connect wallet
const wallet = new Wallet();
await wallet.connect();

// Deploy contract
const contract = await client.deployContract({
  code: contractCode,
  args: [arg1, arg2]
});

// Interact with contract
const result = await contract.call('myMethod', [param1, param2]);
```

### Python
```python
from zhtp_sdk import ZHTPClient, Wallet

# Connect to ZHTP network
client = ZHTPClient(network='mainnet')

# Connect wallet
wallet = Wallet()
await wallet.connect()

# Deploy contract
contract = await client.deploy_contract(
    code=contract_code,
    args=[arg1, arg2]
)

# Interact with contract
result = await contract.call('my_method', [param1, param2])
```

### Rust
```rust
use zhtp_sdk::{ZHTPClient, Wallet, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to ZHTP network
    let client = ZHTPClient::new(Network::Mainnet).await?;
    
    // Connect wallet
    let wallet = Wallet::new().connect().await?;
    
    // Deploy contract
    let contract = client.deploy_contract(contract_code, vec![arg1, arg2]).await?;
    
    // Interact with contract
    let result = contract.call("my_method", vec![param1, param2]).await?;
    
    Ok(())
}
```

### REST API
```bash
# Authentication
curl -X POST https://api.zhtp.network/auth \
  -H "Content-Type: application/json" \
  -d '{"wallet": "0x..."}'

# Deploy contract
curl -X POST https://api.zhtp.network/contracts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"code": "...", "args": ["arg1", "arg2"]}'

# Call contract method
curl -X POST https://api.zhtp.network/contracts/0x.../call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"method": "myMethod", "args": ["param1", "param2"]}'
```

---

## 🌟 SDK Features

### 🔐 Wallet Integration
All SDKs support multiple wallet providers:
- MetaMask
- WalletConnect  
- Coinbase Wallet
- Hardware wallets (Ledger, Trezor)
- ZHTP native wallet

### 📜 Smart Contract Support
- Deploy contracts with simple APIs
- Type-safe contract interactions
- Event listening and filtering
- Gas optimization helpers
- ABI parsing and validation

### 🔒 Zero-Knowledge Features
- ZK proof generation and verification
- Private transaction support
- Anonymous voting systems
- Confidential data sharing
- Identity verification

### 🌐 Decentralized DNS
- Register .zhtp domains
- Resolve domain names
- Update DNS records
- Certificate management
- IPFS integration

### 💾 Distributed Storage
- Upload files to IPFS
- Pin content for availability
- Encrypt sensitive data
- Share files securely
- Content addressing

### ⚡ Real-time Features
- WebSocket connections
- Event subscriptions
- Live data feeds
- Push notifications
- State synchronization

---

## 🎨 UI Framework Integration

### React Components
```jsx
import { ZHTPProvider, useZHTP, ZHTPButton } from '@zhtp/react-components';

function App() {
  return (
    <ZHTPProvider network="mainnet">
      <MyDApp />
    </ZHTPProvider>
  );
}

function MyDApp() {
  const { account, connect, isConnected } = useZHTP();
  
  return (
    <div>
      {isConnected ? (
        <p>Connected: {account}</p>
      ) : (
        <ZHTPButton onClick={connect}>Connect Wallet</ZHTPButton>
      )}
    </div>
  );
}
```

### Vue Components
```vue
<template>
  <ZHTPProvider network="mainnet">
    <div v-if="isConnected">
      <p>Connected: {{ account }}</p>
    </div>
    <ZHTPButton v-else @click="connect">
      Connect Wallet
    </ZHTPButton>
  </ZHTPProvider>
</template>

<script setup>
import { useZHTP } from '@zhtp/vue-components'
const { account, connect, isConnected } = useZHTP()
</script>
```

### Angular Components
```typescript
import { Component } from '@angular/core';
import { ZHTPService } from '@zhtp/angular';

@Component({
  selector: 'app-root',
  template: `
    <div *ngIf="zhtp.isConnected$ | async; else notConnected">
      <p>Connected: {{ zhtp.account$ | async }}</p>
    </div>
    <ng-template #notConnected>
      <zhtp-button (click)="connect()">Connect Wallet</zhtp-button>
    </ng-template>
  `
})
export class AppComponent {
  constructor(public zhtp: ZHTPService) {}
  
  connect() {
    this.zhtp.connect();
  }
}
```

---

## 📱 Mobile Development

### React Native
```typescript
import { ZHTPMobileProvider, useZHTPMobile } from '@zhtp/react-native';

function App() {
  return (
    <ZHTPMobileProvider network="mainnet">
      <WalletScreen />
    </ZHTPMobileProvider>
  );
}

function WalletScreen() {
  const { account, balance, connect, isConnected } = useZHTPMobile();
  
  return (
    <View>
      {isConnected ? (
        <Text>Balance: {balance} ZHTP</Text>
      ) : (
        <Button title="Connect" onPress={connect} />
      )}
    </View>
  );
}
```

### Flutter
```dart
import 'package:zhtp_flutter/zhtp_flutter.dart';

class WalletScreen extends StatefulWidget {
  @override
  _WalletScreenState createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  ZHTPClient? client;
  String? account;
  
  @override
  void initState() {
    super.initState();
    client = ZHTPClient(network: 'mainnet');
  }
  
  Future<void> connect() async {
    account = await client?.connectWallet();
    setState(() {});
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: account != null
          ? Text('Connected: $account')
          : ElevatedButton(
              onPressed: connect,
              child: Text('Connect Wallet'),
            ),
      ),
    );
  }
}
```

---

## 🔧 Development Tools

### CLI Tools
```bash
# Install ZHTP CLI
npm install -g @zhtp/cli

# Create new project
zhtp create my-dapp --template react

# Deploy contract
zhtp deploy --network mainnet

# Manage domains
zhtp domain register my-app.zhtp
```

### VS Code Extension
- Syntax highlighting for ZHTP contracts
- IntelliSense and autocomplete
- Integrated testing and debugging
- Deploy directly from VS Code

### Browser Extension
- Wallet management
- DApp connection
- Transaction signing
- Network switching

---

## 📊 Performance & Scaling

### Batch Operations
```javascript
// JavaScript batch processing
const batch = client.createBatch();
batch.add('getBalance', [address1]);
batch.add('getBalance', [address2]);
batch.add('getBalance', [address3]);

const results = await batch.execute();
```

### Connection Pooling
```python
# Python connection pooling
client = ZHTPClient(
    network='mainnet',
    max_connections=10,
    connection_timeout=30
)
```

### Caching
```rust
// Rust with built-in caching
let client = ZHTPClient::builder()
    .network(Network::Mainnet)
    .cache_enabled(true)
    .cache_ttl(Duration::from_secs(30))
    .build()
    .await?;
```

---

## 🔐 Security Best Practices

### Private Key Management
```javascript
// Never hardcode private keys
const wallet = new Wallet({
  privateKey: process.env.PRIVATE_KEY, // ❌ Don't do this
  provider: 'secure-storage'           // ✅ Use secure storage
});
```

### Input Validation
```python
# Always validate inputs
def transfer_tokens(to_address: str, amount: int):
    if not is_valid_address(to_address):
        raise ValueError("Invalid address")
    
    if amount <= 0:
        raise ValueError("Amount must be positive")
    
    # Proceed with transfer...
```

### Rate Limiting
```rust
// Implement rate limiting
let client = ZHTPClient::builder()
    .rate_limit(RateLimit::new(100, Duration::from_secs(60)))
    .build()
    .await?;
```

---

## 📚 Learning Resources

### Tutorials
- [🚀 Quick Start Guide](../quick-start.md)
- [📱 First DApp Tutorial](../first-dapp.md)
- [🎮 Gaming DApp Example](../examples/gaming/)
- [🛒 Marketplace Tutorial](../examples/marketplace/)

### Video Courses
- ZHTP Fundamentals (Beginner)
- Smart Contract Development (Intermediate)
- Zero-Knowledge Applications (Advanced)
- Production Deployment (Expert)

### Community
- [💬 Discord Developer Chat](https://discord.gg/zhtp-dev)
- [📝 Developer Blog](https://blog.zhtp.dev)
- [🐛 GitHub Issues](https://github.com/zhtp/sdk)
- [❓ Stack Overflow](https://stackoverflow.com/questions/tagged/zhtp)

---

## 🆕 SDK Updates

### Latest Version: v2.1.0
- ✅ Zero-knowledge proof optimization
- ✅ Improved TypeScript types
- ✅ New mobile components
- ✅ Performance improvements
- ✅ Bug fixes and stability

### Roadmap
- **v2.2.0**: GraphQL API support
- **v2.3.0**: WebAssembly integration
- **v2.4.0**: Cross-chain bridges
- **v3.0.0**: Major architecture update

---

## 🤝 Contributing

Help improve the ZHTP SDKs:

1. **Report Bugs**: [GitHub Issues](https://github.com/zhtp/sdk/issues)
2. **Feature Requests**: [Discord Feedback](https://discord.gg/zhtp-feedback)
3. **Documentation**: [Edit on GitHub](https://github.com/zhtp/docs)
4. **Code Contributions**: [Contributing Guide](https://github.com/zhtp/sdk/blob/main/CONTRIBUTING.md)

---

## 📞 Support

Need help? We're here for you:

- 📚 **Documentation**: [docs.zhtp.network](https://docs.zhtp.network)
- 💬 **Discord Support**: [discord.gg/zhtp-support](https://discord.gg/zhtp-support)
- 📧 **Email Support**: developers@zhtp.network
- 🐛 **Bug Reports**: [github.com/zhtp/sdk/issues](https://github.com/zhtp/sdk/issues)

**🚀 Start building with ZHTP SDKs today!**

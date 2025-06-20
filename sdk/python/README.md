# ZHTP Python SDK

**Complete decentralized internet replacement - Zero HTTP, DNS, or SSL dependencies**

[![PyPI version](https://badge.fury.io/py/zhtp-sdk.svg)](https://badge.fury.io/py/zhtp-sdk)
[![Python](https://img.shields.io/pypi/pyversions/zhtp-sdk.svg)](https://pypi.org/project/zhtp-sdk/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## 🌟 Features

- **No HTTP/HTTPS**: Direct peer-to-peer communication
- **No DNS**: Blockchain-based domain resolution
- **No SSL/TLS**: Quantum-resistant encryption built-in
- **Zero-Knowledge**: Privacy-preserving by default
- **Quantum-Resistant**: Post-quantum cryptography
- **Anonymous**: Built-in Tor-like routing
- **Cross-Platform**: Windows, macOS, Linux support
- **Developer-Friendly**: Simple, intuitive API

## Quick Start

```bash
pip install zhtp-sdk
```

```python
import asyncio
from zhtp import ZhtpClient

async def main():
    # Replace HTTP client entirely
    client = ZhtpClient(privacy_level="maximum")
    await client.connect()
    
    # Access decentralized web (no DNS, no HTTP)
    content = await client.fetch_content("news.zhtp", "/latest")
    print(content)

asyncio.run(main())
```

## 📚 Documentation

- **[Complete API Reference](https://docs.zhtp.network/api/python/)**
- **[Quick Start Guide](https://docs.zhtp.network/quick-start/)**
- **[Migration from HTTP](https://docs.zhtp.network/migration/)**

## Installation Options

```bash
# Basic installation
pip install zhtp-sdk

# With all features
pip install zhtp-sdk[full]

# Development version
pip install git+https://github.com/zhtp-network/zhtp-python-sdk.git
```

## Replace Traditional Internet Infrastructure

### Before (Traditional Internet)
```python
# OLD: HTTP/DNS dependent code
import requests
import ssl
import dns.resolver

response = requests.get("https://api.example.com/data")  # Centralized
data = response.json()
```

### After (ZHTP Decentralized)
```python
# NEW: Pure decentralized replacement
import asyncio
from zhtp import ZhtpClient

async def main():
    client = ZhtpClient()
    await client.connect()
    
    # No HTTP, no DNS, no SSL - pure P2P
    data = await client.fetch_json("api.zhtp", "/data")
    
asyncio.run(main())
```

## Privacy & Security

- **Anonymous by Default**: All traffic routed through privacy network
- **Quantum-Resistant**: Post-quantum cryptography algorithms
- **Zero-Knowledge Proofs**: Verify without revealing data
- **Decentralized**: No single points of failure
- **Censorship-Resistant**: Unstoppable content delivery

## 📦 Core Components

- **`ZhtpClient`**: Main client for all operations
- **`BlockchainDNS`**: Decentralized domain resolution
- **`ZkCertificateAuthority`**: Quantum-resistant certificates
- **`AnonymousRouting`**: Privacy-preserving communication
- **`DecentralizedStorage`**: Distributed content storage
- **`SmartContracts`**: Programmable network rules

## 🔄 Migration Guide

Replace traditional libraries:

| Traditional | ZHTP Replacement |
|-------------|------------------|
| `requests` | `ZhtpClient().fetch_content()` |
| `urllib` | `ZhtpClient().fetch_json()` |
| `socket` | `ZhtpClient().connect_peer()` |
| `ssl` | Built-in quantum encryption |
| `dns` | `BlockchainDNS().resolve()` |

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 🌍 Community

- **Website**: [https://zhtp.network](https://zhtp.network)
- **Documentation**: [https://docs.zhtp.network](https://docs.zhtp.network)
- **GitHub**: [https://github.com/zhtp-network](https://github.com/zhtp-network)
- **Discord**: [https://discord.gg/zhtp](https://discord.gg/zhtp)

---

**ZHTP: The Internet, Reimagined. Decentralized. Private. Quantum-Safe.**

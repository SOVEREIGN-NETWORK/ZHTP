# SOVEREIGN NETWORK

A decentralized internet platform with zero-knowledge proofs and post-quantum cryptography.

## Features

- Post-quantum cryptographic security
- Zero-knowledge routing proofs
- Decentralized storage (DHT)
- Privacy-preserving browser
- WASM smart contracts
- Consensus with proof validation

## Quick Start

### Requirements
- Rust 1.70+
- wasm-pack
- Node.js 16+

### Build & Run
```bash
# Build WASM contracts
cd contracts
./build.sh  # or build.bat on Windows

# Run network node
cargo run --release

# Run tests
cargo test --test system_test -- --nocapture
```

### Browser Interface
Open `browser/index.html` to interact with the network through the browser interface.

## Architecture

- `src/zhtp/` - Zero-knowledge Hidden Transport Protocol implementation
- `src/storage/` - Distributed Hash Table implementation
- `src/consensus/` - Network consensus and proof validation
- `contracts/` - WASM smart contracts
- `browser/` - Browser interface

## Security

- Post-quantum key encapsulation using Kyber-768
- Post-quantum signatures using Dilithium2
- Zero-knowledge routing with Plonk
- Storage proofs for content verification

## Development Status

- [x] Core networking
- [x] Post-quantum crypto
- [x] DHT storage
- [x] Browser interface
- [x] Smart contracts
- [x] System tests
- [ ] Production hardening
- [ ] Security audits

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

1. Fork the repo
2. Create feature branch (`git checkout -b my-feature`)
3. Commit changes (`git commit -am 'Add feature'`)
4. Push branch (`git push origin my-feature`)
5. Create Pull Request

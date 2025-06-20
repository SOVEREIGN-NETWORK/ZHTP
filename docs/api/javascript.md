# 🌐 JavaScript/TypeScript SDK - Complete Decentralized Internet Replacement

**Complete API documentation for the ZHTP JavaScript/TypeScript SDK that replaces all traditional internet infrastructure**

> **Important**: This SDK completely replaces HTTP/HTTPS clients (fetch, axios), DNS resolution (native DNS), SSL/TLS libraries, and traditional web APIs. No traditional internet infrastructure is used.

## Installation (Replaces HTTP Libraries)

```bash
# Replaces fetch, axios, request, superagent, etc.
npm install @zhtp/sdk

# Optional: TypeScript support
npm install --save-dev @types/zhtp

# Verify installation and connectivity to decentralized network
npx zhtp --version
npx zhtp network status
```

## Quick Import (Native ZHTP Protocol)

```typescript
// Import ZHTP SDK (replaces traditional web libraries)
import { 
  ZhtpClient,           // Replaces fetch/axios
  DAppBuilder,          // Replaces traditional web frameworks  
  SmartContract,        // Blockchain contract interface
  ZkProof,             // Zero-knowledge privacy
  BlockchainDNS,       // Replaces traditional DNS
  ZkCertificateAuth    // Replaces SSL/TLS certificates
} from '@zhtp/sdk';

// No traditional imports needed:
// ❌ import fetch from 'node-fetch';
// ❌ import axios from 'axios';  
// ❌ import https from 'https';
// ❌ import dns from 'dns';
```

## 🏗️ ZhtpClient - Complete Traditional Internet Replacement

The `ZhtpClient` replaces all traditional internet infrastructure and protocols.

### Constructor

```typescript
class ZhtpClient {
  constructor(options?: ZhtpClientOptions)
}

interface ZhtpClientOptions {
  network?: 'mainnet' | 'testnet' | 'local';
  nodeEndpoints?: string[];           // ZHTP protocol endpoints (not HTTP)
  privateKey?: string;                // Wallet for transactions
  privacyLevel?: 'standard' | 'high' | 'maximum';  // Built-in anonymity
  securityLevel?: 'standard' | 'quantum_resistant'; // Post-quantum crypto
  cacheSize?: number;                 // Local content cache
  maxHops?: number;                   // Anonymous routing hops
}
```

### Example (Replaces HTTP Clients)

```typescript
import { ZhtpClient } from '@zhtp/sdk';

// Traditional HTTP client setup - NEVER DO THIS IN ZHTP:
// ❌ const axios = require('axios');
// ❌ const client = axios.create({ baseURL: 'https://api.example.com' });

// ZHTP decentralized client setup:
const client = new ZhtpClient({
  network: 'mainnet',
  privacyLevel: 'high',        // Built-in anonymity (no VPN needed)
  securityLevel: 'quantum_resistant', // Post-quantum security
  nodeEndpoints: [             // Native ZHTP protocol (not HTTP)
    'zhtp://node1.zhtp.network:8443',
    'zhtp://node2.zhtp.network:8443'
  ]
});

// Connect to decentralized network (no DNS, no HTTP)
await client.connect();
console.log('Connected to decentralized internet!');
```

### Methods

#### `connect(): Promise<void>`
Establishes connection to decentralized ZHTP network (replaces HTTP connections).

```typescript
// Traditional HTTP connection - NEVER DO THIS:
// ❌ await fetch('https://api.example.com/health');

// ZHTP decentralized connection:
await client.connect();
console.log('Connected to decentralized network');

// Verify connection without HTTP
const status = await client.getNetworkStatus();
console.log(`Connected to ${status.nodeCount} nodes`);
```

#### `fetch_content(): Promise<Content>` 
Fetches content via decentralized storage (replaces HTTP requests).

```typescript
// Traditional HTTP request - NEVER DO THIS:
// ❌ const response = await fetch('https://api.example.com/data');
// ❌ const data = await response.json();

// ZHTP decentralized content fetching:
const content = await client.fetch_content({
  domain: 'news.zhtp',         // Resolved via blockchain DNS
  path: '/latest-articles',
  anonymous: true,             // Built-in privacy
  verify_zk_proof: true       // Cryptographic content verification
});

console.log('Decentralized content:', content);
```

#### `blockchain_dns.resolve(): Promise<DomainInfo>`
Resolves domains via blockchain (replaces traditional DNS).

```typescript
// Traditional DNS resolution - NEVER DO THIS:
// ❌ import dns from 'dns';
// ❌ const addresses = await dns.promises.resolve4('example.com');

// ZHTP blockchain DNS resolution:
const domainInfo = await client.blockchain_dns.resolve('my-app.zhtp');
console.log('Domain info:', {
  contentHash: domainInfo.contentHash,
  zkCertificate: domainInfo.zkCertificate,
  storageNodes: domainInfo.storageNodes
});
```

```typescript
const dapp = await client.getDApp('news.zhtp');
console.log(`DApp name: ${dapp.name}`);
```

#### `searchDApps(query: string): Promise<DApp[]>`
Searches for DApps across the network.

```typescript
const results = await client.searchDApps('news');
console.log(`Found ${results.length} DApps`);
```

## 📱 DAppBuilder

Fluent API for building and configuring DApps.

### Basic Usage

```typescript
import { DAppBuilder } from '@zhtp/sdk';

const dapp = new DAppBuilder()
  .setName('My Amazing DApp')
  .setDescription('A revolutionary decentralized application')
  .setDomain('my-app.zhtp')
  .addRoute('/', HomeHandler)
  .addRoute('/api/users', UserAPIHandler)
  .enableZkPrivacy()
  .addSmartContract(userContract)
  .build();
```

### Methods

#### `setName(name: string): DAppBuilder`
Sets the DApp display name.

#### `setDescription(description: string): DAppBuilder`
Sets the DApp description.

#### `setDomain(domain: string): DAppBuilder`
Sets the .zhtp domain name.

#### `addRoute(path: string, handler: RouteHandler): DAppBuilder`
Adds a route handler.

```typescript
// Simple route
.addRoute('/', (req) => ({
  html: '<h1>Welcome!</h1>'
}))

// API route
.addRoute('/api/users', async (req) => ({
  json: { users: await getUsers() }
}))

// File upload route
.addRoute('/upload', (req) => {
  const file = req.files.upload;
  return { success: true, fileId: file.id };
})
```

#### `enableZkPrivacy(options?: ZkPrivacyOptions): DAppBuilder`
Enables zero-knowledge privacy features.

```typescript
.enableZkPrivacy({
  anonymousRouting: true,
  encryptedStorage: true,
  zkTransactions: true,
  mixnetDelay: 5000 // 5 second mixing delay
})
```

#### `addSmartContract(contract: SmartContract): DAppBuilder`
Integrates a smart contract.

#### `setTheme(theme: Theme): DAppBuilder`
Applies a visual theme.

```typescript
.setTheme({
  primaryColor: '#3498db',
  secondaryColor: '#2ecc71',
  darkMode: true,
  customCSS: `
    .hero { background: linear-gradient(45deg, #667eea, #764ba2); }
  `
})
```

#### `addDatabase(config: DatabaseConfig): DAppBuilder`
Configures decentralized storage.

```typescript
.addDatabase({
  type: 'ipfs',
  encryption: true,
  replication: 3
})
```

#### `setEconomics(config: EconomicsConfig): DAppBuilder`
Configures revenue model.

```typescript
.setEconomics({
  model: 'freemium',
  premiumFeatures: ['advanced-analytics', 'custom-themes'],
  subscriptionPrice: 100, // ZHTP tokens per month
  revenueShare: 0.8 // 80% to developer
})
```

## 📜 SmartContract

Interface for creating and interacting with smart contracts.

### Creating a Contract

```typescript
import { SmartContract } from '@zhtp/sdk';

const userContract = new SmartContract({
  name: 'UserRegistry',
  source: `
    contract UserRegistry {
      mapping(address => User) public users;
      
      struct User {
        string name;
        string email;
        uint256 joinDate;
      }
      
      function registerUser(string memory name, string memory email) public {
        users[msg.sender] = User(name, email, block.timestamp);
      }
      
      function getUser(address addr) public view returns (User memory) {
        return users[addr];
      }
    }
  `,
  abi: [...], // Contract ABI
  bytecode: '0x...' // Compiled bytecode
});
```

### Contract Methods

#### `deploy(): Promise<DeployedContract>`
Deploys the contract to ZHTP network.

```typescript
const deployed = await userContract.deploy();
console.log(`Contract deployed at: ${deployed.address}`);
```

#### `call(method: string, params: any[]): Promise<any>`
Calls a contract method.

```typescript
// Read-only call
const user = await deployed.call('getUser', [userAddress]);

// Transaction call
const txHash = await deployed.call('registerUser', ['Alice', 'alice@example.com']);
```

#### `listen(event: string, callback: Function): void`
Listens for contract events.

```typescript
deployed.listen('UserRegistered', (event) => {
  console.log(`New user: ${event.name}`);
});
```

## 🔐 ZkProof

Zero-knowledge proof utilities.

### Generating Proofs

```typescript
import { ZkProof } from '@zhtp/sdk';

// Prove age without revealing exact age
const ageProof = await ZkProof.create({
  circuit: 'age-verification',
  publicInputs: [21], // Minimum age
  privateInputs: [25], // Actual age
  witness: userAgeWitness
});

// Verify proof
const isValid = await ZkProof.verify(ageProof);
console.log(`Age proof valid: ${isValid}`);
```

### Common Proof Types

```typescript
// Identity proof
const identityProof = await ZkProof.identity({
  commitment: userCommitment,
  nullifier: userNullifier
});

// Balance proof
const balanceProof = await ZkProof.balance({
  minBalance: 1000,
  actualBalance: 5000,
  salt: randomSalt
});

// Membership proof
const membershipProof = await ZkProof.membership({
  set: validMembers,
  member: userPublicKey,
  witness: membershipWitness
});
```

## 🌐 BlockchainDNS

Interact with the decentralized DNS system.

### Domain Operations

```typescript
import { BlockchainDNS } from '@zhtp/sdk';

const dns = new BlockchainDNS(client);

// Register a domain
await dns.register({
  domain: 'my-app.zhtp',
  owner: ownerAddress,
  content: ipfsHash,
  ttl: 3600
});

// Resolve a domain
const record = await dns.resolve('news.zhtp');
console.log(`Domain points to: ${record.content}`);

// Update domain record
await dns.update('my-app.zhtp', {
  content: newIpfsHash,
  ttl: 7200
});

// Transfer ownership
await dns.transfer('my-app.zhtp', newOwnerAddress);
```

### Domain Search

```typescript
// Search available domains
const available = await dns.searchAvailable('news');
console.log(`Available domains: ${available.join(', ')}`);

// Get domain history
const history = await dns.getHistory('news.zhtp');
history.forEach(change => {
  console.log(`${change.timestamp}: ${change.action}`);
});
```

## 🔒 Certificate

ZK Certificate Authority operations.

### Certificate Management

```typescript
import { Certificate } from '@zhtp/sdk';

const cert = new Certificate(client);

// Issue a certificate
const certificate = await cert.issue({
  domain: 'my-app.zhtp',
  publicKey: domainPublicKey,
  validityPeriod: 365 * 24 * 60 * 60 // 1 year
});

// Verify a certificate
const isValid = await cert.verify('my-app.zhtp');
console.log(`Certificate valid: ${isValid}`);

// Renew certificate
await cert.renew('my-app.zhtp');

// Revoke certificate
await cert.revoke('compromised-app.zhtp', 'Key compromise');
```

## 🎨 UI Components

React components for common ZHTP functionality.

### Installation

```bash
npm install @zhtp/react
```

### Usage

```tsx
import { 
  ZhtpProvider, 
  ConnectWallet, 
  DAppCard, 
  CertificateStatus,
  DomainSearch 
} from '@zhtp/react';

function App() {
  return (
    <ZhtpProvider network="mainnet">
      <ConnectWallet />
      
      <DomainSearch 
        onDomainSelect={(domain) => console.log(domain)}
        placeholder="Search .zhtp domains..."
      />
      
      <DAppCard 
        domain="news.zhtp"
        showMetrics={true}
        onOpen={(dapp) => window.open(dapp.url)}
      />
      
      <CertificateStatus 
        domain="my-app.zhtp"
        showDetails={true}
      />
    </ZhtpProvider>
  );
}
```

### Hooks

```tsx
import { useZhtp, useDApp, useCertificate } from '@zhtp/react';

function MyComponent() {
  const { client, connected, network } = useZhtp();
  const { dapp, loading, error } = useDApp('news.zhtp');
  const { certificate, isValid } = useCertificate('my-app.zhtp');
  
  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;
  
  return (
    <div>
      <h1>{dapp.name}</h1>
      <p>Network: {network}</p>
      <p>Certificate: {isValid ? '✅ Valid' : '❌ Invalid'}</p>
    </div>
  );
}
```

## 🚀 Advanced Features

### WebAssembly ZK Proofs

```typescript
import { WasmZkProof } from '@zhtp/sdk/wasm';

// High-performance ZK proofs in the browser
const proof = await WasmZkProof.generate({
  circuit: 'complex-computation',
  inputs: largeDataset,
  useWorker: true // Use web worker for non-blocking computation
});
```

### Real-time Updates

```typescript
import { RealtimeClient } from '@zhtp/sdk';

const realtime = new RealtimeClient(client);

// Subscribe to network events
realtime.subscribe('dapp-deployed', (event) => {
  console.log(`New DApp: ${event.domain}`);
});

// Subscribe to domain changes
realtime.subscribe(`domain:news.zhtp`, (event) => {
  console.log(`Domain updated: ${event.change}`);
});
```

### Batch Operations

```typescript
import { BatchClient } from '@zhtp/sdk';

const batch = new BatchClient(client);

// Batch multiple operations
const operations = batch
  .deploy(dapp1)
  .deploy(dapp2)
  .registerDomain('app1.zhtp')
  .registerDomain('app2.zhtp');

// Execute all at once
const results = await operations.execute();
```

## 🔧 Configuration

### Environment Variables

```bash
# .env
ZHTP_NETWORK=mainnet
ZHTP_PRIVATE_KEY=your_private_key
ZHTP_ENDPOINT=https://api.zhtp.network
ZHTP_GAS_LIMIT=1000000
ZHTP_GAS_PRICE=1000
```

### Configuration File

```typescript
// zhtp.config.ts
export default {
  network: 'mainnet',
  contracts: {
    userRegistry: '0x...',
    tokenContract: '0x...'
  },
  ipfs: {
    gateway: 'https://ipfs.zhtp.network'
  },
  monitoring: {
    enabled: true,
    apiKey: 'your_api_key'
  }
};
```

## 📊 Error Handling

```typescript
import { ZhtpError, NetworkError, ContractError } from '@zhtp/sdk';

try {
  await client.deploy(dapp);
} catch (error) {
  if (error instanceof NetworkError) {
    console.log('Network connection failed');
  } else if (error instanceof ContractError) {
    console.log(`Contract error: ${error.reason}`);
  } else if (error instanceof ZhtpError) {
    console.log(`ZHTP error: ${error.code} - ${error.message}`);
  }
}
```

## 🧪 Testing

```typescript
import { ZhtpTestClient } from '@zhtp/sdk/testing';

describe('My DApp', () => {
  let testClient: ZhtpTestClient;
  
  beforeEach(() => {
    testClient = new ZhtpTestClient();
  });
  
  test('should deploy successfully', async () => {
    const result = await testClient.deploy(dapp);
    expect(result.success).toBe(true);
  });
  
  test('should handle user registration', async () => {
    const response = await testClient.callRoute('/api/register', {
      method: 'POST',
      body: { name: 'Alice', email: 'alice@example.com' }
    });
    
    expect(response.status).toBe(200);
    expect(response.data.success).toBe(true);
  });
});
```

---

**Next**: [Python SDK Reference](./python.md) →

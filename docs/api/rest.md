# 🌐 ZHTP Protocol API Reference - Native Decentralized Access

**Complete native protocol documentation for accessing ZHTP network without traditional HTTP/REST infrastructure**

> **Important**: This documentation describes the native ZHTP protocol interface. Unlike traditional REST APIs that use HTTP/HTTPS, ZHTP uses its own decentralized protocol that eliminates the need for web servers, DNS, and SSL certificates.

## Native Protocol Endpoints (No HTTP)

```
Mainnet: zhtp://mainnet.zhtp.network:8443
Testnet: zhtp://testnet.zhtp.network:8443
Local:   zhtp://localhost:8443
```

## Protocol Authentication (No JWT/HTTP Headers)

ZHTP protocol uses wallet-based cryptographic authentication with zero-knowledge proofs.

### Authentication Method

```bash
# Connect with wallet authentication (no HTTP headers)
zhtp connect --network mainnet --wallet <private_key>

# Or using CLI authentication
zhtp auth login --wallet <wallet_address> --sign-message
```

### Protocol Message Authentication

```rust
// Rust example - native protocol authentication
use zhtp::{ZhtpClient, WalletAuth};

let auth = WalletAuth::new(
    wallet_address,
    private_key,
    None  // No timestamp needed - blockchain provides ordering
);

let client = ZhtpClient::new(config)
    .with_auth(auth)
    .build()
    .await?;
```

### Zero-Knowledge Authentication (Replaces JWT)

```python
# Python example - ZK proof authentication
from zhtp import ZhtpClient, ZkProofAuth

# Create ZK proof of wallet ownership without revealing private key
zk_auth = ZkProofAuth.create_proof(
    wallet_address="0x1234...",
    private_key_file="~/.zhtp/wallet.key",
    privacy_level="maximum"  # Anonymous authentication
)

client = ZhtpClient(
    network="mainnet",
    auth=zk_auth  # No JWT tokens needed
)
```

```javascript
const message = JSON.stringify({
  method: 'POST',
  path: '/dapps/deploy',
  timestamp: Date.now(),
  body: requestBody
});

const signature = await wallet.signMessage(message);
```

## 🏗️ Network Information

### Get Network Stats

```http
GET /network/info
```

**Response:**
```json
{
  "success": true,
  "data": {
    "network": "mainnet",
    "node_count": 1247,
    "block_height": 892456,
    "total_supply": "1000000000000000000000000000",
    "staked_amount": "750000000000000000000000000",
    "consensus_algorithm": "PoS",
    "block_time": 3,
    "tps": 10000,
    "validator_count": 100
  }
}
```

### Get Node Status

```http
GET /network/nodes
```

**Response:**
```json
{
  "success": true,
  "data": {
    "nodes": [
      {
        "id": "node_001",
        "address": "192.168.1.100:8080",
        "status": "active",
        "stake": "10000000000000000000000",
        "uptime": "99.9%",
        "last_seen": 1672531200
      }
    ],
    "total_nodes": 1247,
    "active_nodes": 1245
  }
}
```

## 🚀 DApp Management

### Deploy DApp

```http
POST /dapps/deploy
Content-Type: multipart/form-data
Authorization: Bearer <jwt_token>

{
  "name": "MyAwesomeApp",
  "version": "1.0.0",
  "description": "My first ZHTP DApp",
  "package": <binary_file>,
  "metadata": {
    "author": "developer@example.com",
    "license": "MIT",
    "tags": ["productivity", "tools"]
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0xabcdef1234567890...",
    "transaction_hash": "0x...",
    "gas_used": 234567,
    "deployment_cost": "0.05",
    "content_hash": "QmXoYCz...",
    "size": 1048576
  }
}
```

### Get DApp Info

```http
GET /dapps/{address}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0xabcdef1234567890...",
    "name": "MyAwesomeApp",
    "version": "1.0.0",
    "description": "My first ZHTP DApp",
    "author": "developer@example.com",
    "deploy_time": 1672531200,
    "size": 1048576,
    "content_hash": "QmXoYCz...",
    "access_count": 15647,
    "rating": 4.8,
    "status": "active",
    "metadata": {
      "license": "MIT",
      "tags": ["productivity", "tools"],
      "homepage": "https://myapp.zhtp",
      "repository": "https://github.com/user/myapp"
    }
  }
}
```

### List DApps

```http
GET /dapps?limit=20&offset=0&category=productivity&sort=popularity
```

**Query Parameters:**
- `limit`: Number of results (default: 20, max: 100)
- `offset`: Pagination offset (default: 0)
- `category`: Filter by category
- `sort`: Sort order (popularity, newest, name, rating)
- `search`: Search query

**Response:**
```json
{
  "success": true,
  "data": {
    "dapps": [
      {
        "address": "0xabcdef1234567890...",
        "name": "MyAwesomeApp",
        "description": "My first ZHTP DApp",
        "author": "developer@example.com",
        "rating": 4.8,
        "access_count": 15647,
        "tags": ["productivity", "tools"],
        "thumbnail": "QmThumbnail...",
        "deploy_time": 1672531200
      }
    ],
    "total": 1247,
    "page": 1,
    "pages": 63
  }
}
```

### Update DApp

```http
PUT /dapps/{address}
Content-Type: multipart/form-data
Authorization: Bearer <jwt_token>

{
  "version": "1.1.0",
  "package": <binary_file>,
  "changelog": "Bug fixes and performance improvements"
}
```

### Delete DApp

```http
DELETE /dapps/{address}
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "success": true,
  "message": "DApp deleted successfully",
  "transaction_hash": "0x..."
}
```

## 🔗 Smart Contracts

### Deploy Contract

```http
POST /contracts/deploy
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "name": "MyContract",
  "source_code": "contract MyContract { ... }",
  "compiler_version": "0.8.19",
  "constructor_args": [1000, "Token Name"],
  "gas_limit": 500000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x1234567890abcdef...",
    "transaction_hash": "0x...",
    "gas_used": 456789,
    "deployment_cost": "0.08",
    "abi": [...],
    "bytecode": "0x608060405234801561001057600080fd5b50..."
  }
}
```

### Call Contract Function

```http
POST /contracts/{address}/call
Content-Type: application/json

{
  "function": "balanceOf",
  "args": ["0x1234567890abcdef..."],
  "block": "latest"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "result": "1000000000000000000000",
    "gas_used": 21000,
    "block_number": 892456
  }
}
```

### Send Contract Transaction

```http
POST /contracts/{address}/send
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "function": "transfer",
  "args": ["0xrecipient...", "1000000000000000000"],
  "gas_limit": 50000,
  "gas_price": "20000000000"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "transaction_hash": "0x...",
    "gas_used": 45000,
    "status": "pending",
    "block_number": null
  }
}
```

### Get Contract Info

```http
GET /contracts/{address}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x1234567890abcdef...",
    "name": "MyContract",
    "compiler_version": "0.8.19",
    "deploy_time": 1672531200,
    "deployer": "0xdeployer...",
    "transaction_count": 15647,
    "abi": [...],
    "source_code": "contract MyContract { ... }",
    "verified": true
  }
}
```

## 🌐 Blockchain DNS

### Register Domain

```http
POST /dns/register
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "domain": "myapp.zhtp",
  "record": {
    "address": "0x1234567890abcdef...",
    "content_hash": "QmXoYCz...",
    "ttl": 3600
  },
  "duration": 31536000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "transaction_hash": "0x...",
    "registration_cost": "10.0",
    "expires_at": 1704067200
  }
}
```

### Resolve Domain

```http
GET /dns/resolve/{domain}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "domain": "myapp.zhtp",
    "address": "0x1234567890abcdef...",
    "content_hash": "QmXoYCz...",
    "ttl": 3600,
    "owner": "0xowner...",
    "expires_at": 1704067200,
    "last_updated": 1672531200
  }
}
```

### Update Domain Record

```http
PUT /dns/{domain}
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "record": {
    "address": "0xnewaddress...",
    "content_hash": "QmNewHash...",
    "ttl": 7200
  }
}
```

### List Domains

```http
GET /dns/domains?owner=0x...&limit=20&offset=0
```

**Response:**
```json
{
  "success": true,
  "data": {
    "domains": [
      {
        "domain": "myapp.zhtp",
        "owner": "0xowner...",
        "address": "0x1234567890abcdef...",
        "expires_at": 1704067200,
        "registration_date": 1672531200
      }
    ],
    "total": 47,
    "page": 1
  }
}
```

## 🔑 Certificates

### Generate Certificate

```http
POST /certificates/generate
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "domain": "myapp.zhtp",
  "subject": {
    "country": "US",
    "state": "CA",
    "city": "San Francisco",
    "organization": "My Company",
    "email": "admin@myapp.zhtp"
  },
  "validity_days": 365
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "certificate": "-----BEGIN CERTIFICATE-----\n...",
    "private_key": "-----BEGIN PRIVATE KEY-----\n...",
    "transaction_hash": "0x...",
    "valid_from": 1672531200,
    "valid_until": 1704067200
  }
}
```

### Install Certificate

```http
POST /certificates/install
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "domain": "myapp.zhtp",
  "certificate": "-----BEGIN CERTIFICATE-----\n...",
  "private_key": "-----BEGIN PRIVATE KEY-----\n..."
}
```

### Verify Certificate

```http
GET /certificates/verify/{domain}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "domain": "myapp.zhtp",
    "valid": true,
    "issuer": "ZHTP CA",
    "subject": "myapp.zhtp",
    "valid_from": 1672531200,
    "valid_until": 1704067200,
    "fingerprint": "SHA256:...",
    "algorithm": "RSA-2048"
  }
}
```

## 🔐 Zero-Knowledge Proofs

### Generate Proof

```http
POST /zk/proofs/generate
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "circuit": "age_verification",
  "inputs": {
    "age": 25,
    "min_age": 18
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "proof": "0x...",
    "public_inputs": [1],
    "verification_key": "0x...",
    "circuit_hash": "0x..."
  }
}
```

### Verify Proof

```http
POST /zk/proofs/verify
Content-Type: application/json

{
  "proof": "0x...",
  "public_inputs": [1],
  "verification_key": "0x...",
  "circuit": "age_verification"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "verification_time": 123
  }
}
```

### List Available Circuits

```http
GET /zk/circuits
```

**Response:**
```json
{
  "success": true,
  "data": {
    "circuits": [
      {
        "name": "age_verification",
        "description": "Verify age without revealing exact age",
        "inputs": ["age", "min_age"],
        "outputs": ["is_valid"],
        "complexity": "low"
      },
      {
        "name": "identity_verification",
        "description": "Verify identity attributes",
        "inputs": ["citizenship", "age", "required_citizenship", "min_age"],
        "outputs": ["citizenship_valid", "age_valid"],
        "complexity": "medium"
      }
    ]
  }
}
```

## 💰 Wallet & Transactions

### Get Balance

```http
GET /wallet/{address}/balance
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x1234567890abcdef...",
    "balance": "1000.5",
    "staked": "500.0",
    "locked": "0.0",
    "available": "500.5"
  }
}
```

### Send Transaction

```http
POST /transactions/send
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "to": "0xrecipient...",
  "amount": "10.5",
  "memo": "Payment for services",
  "gas_limit": 21000,
  "gas_price": "20000000000"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "transaction_hash": "0x...",
    "status": "pending",
    "gas_used": null,
    "block_number": null,
    "timestamp": 1672531200
  }
}
```

### Get Transaction

```http
GET /transactions/{hash}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "hash": "0x...",
    "from": "0xsender...",
    "to": "0xrecipient...",
    "amount": "10.5",
    "gas_used": 21000,
    "gas_price": "20000000000",
    "status": "confirmed",
    "block_number": 892456,
    "block_hash": "0x...",
    "timestamp": 1672531200,
    "memo": "Payment for services"
  }
}
```

### Get Transaction History

```http
GET /wallet/{address}/transactions?limit=20&offset=0&type=all
```

**Query Parameters:**
- `limit`: Number of results (default: 20, max: 100)
- `offset`: Pagination offset (default: 0)
- `type`: Transaction type (all, sent, received, contract)
- `from_date`: Start date (Unix timestamp)
- `to_date`: End date (Unix timestamp)

**Response:**
```json
{
  "success": true,
  "data": {
    "transactions": [
      {
        "hash": "0x...",
        "from": "0xsender...",
        "to": "0xrecipient...",
        "amount": "10.5",
        "type": "transfer",
        "status": "confirmed",
        "timestamp": 1672531200,
        "gas_used": 21000,
        "memo": "Payment for services"
      }
    ],
    "total": 347,
    "page": 1,
    "pages": 18
  }
}
```

## 📊 Blockchain Data

### Get Block

```http
GET /blocks/{block_number_or_hash}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "number": 892456,
    "hash": "0x...",
    "parent_hash": "0x...",
    "timestamp": 1672531200,
    "size": 12345,
    "gas_used": 8000000,
    "gas_limit": 10000000,
    "transactions": [
      {
        "hash": "0x...",
        "from": "0x...",
        "to": "0x...",
        "amount": "10.5",
        "gas_used": 21000
      }
    ],
    "transaction_count": 156,
    "validator": "0xvalidator...",
    "rewards": "2.5"
  }
}
```

### Get Latest Blocks

```http
GET /blocks?limit=10
```

**Response:**
```json
{
  "success": true,
  "data": {
    "blocks": [
      {
        "number": 892456,
        "hash": "0x...",
        "timestamp": 1672531200,
        "transaction_count": 156,
        "validator": "0xvalidator...",
        "size": 12345
      }
    ],
    "latest_block": 892456
  }
}
```

### Search

```http
GET /search?q={query}
```

**Query Types:**
- Block number: `892456`
- Block hash: `0x...`
- Transaction hash: `0x...`
- Address: `0x...`
- Domain: `myapp.zhtp`

**Response:**
```json
{
  "success": true,
  "data": {
    "type": "transaction",
    "result": {
      "hash": "0x...",
      "from": "0x...",
      "to": "0x...",
      "amount": "10.5",
      "status": "confirmed"
    }
  }
}
```

## 📈 Analytics & Statistics

### Get DApp Analytics

```http
GET /analytics/dapps/{address}?period=7d
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x...",
    "period": "7d",
    "metrics": {
      "unique_users": 1247,
      "total_sessions": 8934,
      "avg_session_duration": 456,
      "page_views": 25678,
      "transactions": 3456,
      "revenue": "125.5"
    },
    "daily_stats": [
      {
        "date": "2023-01-01",
        "users": 178,
        "sessions": 1278,
        "transactions": 494
      }
    ]
  }
}
```

### Get Network Analytics

```http
GET /analytics/network?period=7d
```

**Response:**
```json
{
  "success": true,
  "data": {
    "period": "7d",
    "metrics": {
      "total_transactions": 1247689,
      "avg_tps": 4567,
      "total_addresses": 89234,
      "active_addresses": 12456,
      "total_dapps": 1247,
      "active_dapps": 892,
      "network_value": "1500000000"
    },
    "daily_stats": [
      {
        "date": "2023-01-01",
        "transactions": 178234,
        "tps": 4567,
        "active_addresses": 1789
      }
    ]
  }
}
```

## 🛠️ Development Tools

### Validate Contract

```http
POST /tools/validate-contract
Content-Type: application/json

{
  "source_code": "contract MyContract { ... }",
  "compiler_version": "0.8.19"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "warnings": [
      "Unused variable: temp"
    ],
    "gas_estimate": 456789,
    "security_score": 95
  }
}
```

### Estimate Gas

```http
POST /tools/estimate-gas
Content-Type: application/json

{
  "to": "0x...",
  "data": "0x...",
  "value": "0"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "gas_estimate": 45000,
    "gas_price": "20000000000",
    "cost_estimate": "0.0009"
  }
}
```

### Generate Wallet

```http
POST /tools/generate-wallet
```

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x1234567890abcdef...",
    "private_key": "0x...",
    "public_key": "0x...",
    "mnemonic": "abandon ability able about above absent absorb abstract absurd abuse access accident"
  }
}
```

## 🔍 Advanced Queries

### Query DApps with Complex Filters

```http
POST /query/dapps
Content-Type: application/json

{
  "filters": {
    "categories": ["defi", "gaming"],
    "min_rating": 4.0,
    "max_size": 10485760,
    "deployed_after": 1672531200,
    "has_contracts": true,
    "verified": true
  },
  "sort": {
    "field": "access_count",
    "order": "desc"
  },
  "limit": 50,
  "offset": 0
}
```

### Aggregate Transaction Data

```http
POST /query/transactions/aggregate
Content-Type: application/json

{
  "group_by": "date",
  "filters": {
    "from_date": 1672531200,
    "to_date": 1672617600,
    "min_amount": "1.0",
    "addresses": ["0x...", "0x..."]
  },
  "aggregations": ["sum", "count", "avg"]
}
```

## 📡 WebSocket API

### Connect to WebSocket

```javascript
const ws = new WebSocket('wss://api.zhtp.network/ws');

ws.onopen = () => {
  // Subscribe to events
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['blocks', 'transactions', 'dapps'],
    auth_token: 'jwt_token'
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

### Available Channels

- `blocks`: New block notifications
- `transactions`: New transaction notifications
- `dapps`: DApp deployment/updates
- `contracts`: Contract events
- `dns`: DNS record changes

### Event Format

```json
{
  "channel": "blocks",
  "type": "new_block",
  "data": {
    "number": 892457,
    "hash": "0x...",
    "timestamp": 1672531260,
    "transaction_count": 89
  },
  "timestamp": 1672531260
}
```

## ❌ Error Handling

### Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "INVALID_ADDRESS",
    "message": "The provided address is not a valid Ethereum address",
    "details": {
      "field": "address",
      "value": "invalid_address_here"
    }
  },
  "timestamp": 1672531200
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Invalid request format or parameters |
| `UNAUTHORIZED` | 401 | Invalid or missing authentication |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMITED` | 429 | Rate limit exceeded |
| `INTERNAL_ERROR` | 500 | Internal server error |
| `NETWORK_ERROR` | 503 | Network connectivity issues |
| `INVALID_ADDRESS` | 400 | Invalid blockchain address |
| `INSUFFICIENT_FUNDS` | 400 | Insufficient balance for transaction |
| `GAS_LIMIT_EXCEEDED` | 400 | Transaction exceeds gas limit |
| `CONTRACT_EXECUTION_FAILED` | 400 | Smart contract execution failed |

## 🔒 Rate Limiting

### Rate Limits

| Endpoint Type | Requests per Minute | Requests per Hour |
|---------------|-------------------|------------------|
| Read operations | 1000 | 10000 |
| Write operations | 100 | 1000 |
| Authentication | 10 | 100 |
| File uploads | 20 | 200 |

### Rate Limit Headers

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1672531260
```

## 📚 SDKs and Examples

### cURL Examples

```bash
# Get network info
curl -X GET "https://api.zhtp.network/network/info" \
  -H "Accept: application/json"

# Deploy DApp
curl -X POST "https://api.zhtp.network/dapps/deploy" \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: multipart/form-data" \
  -F "name=MyApp" \
  -F "package=@myapp.zip"

# Call contract function
curl -X POST "https://api.zhtp.network/contracts/0x.../call" \
  -H "Content-Type: application/json" \
  -d '{
    "function": "balanceOf",
    "args": ["0x1234567890abcdef..."]
  }'
```

### Python Example

```python
import requests
import json

# Base configuration
BASE_URL = "https://api.zhtp.network"
JWT_TOKEN = "your_jwt_token"

headers = {
    "Authorization": f"Bearer {JWT_TOKEN}",
    "Content-Type": "application/json"
}

# Get network info
response = requests.get(f"{BASE_URL}/network/info")
print(json.dumps(response.json(), indent=2))

# Deploy DApp
files = {"package": open("myapp.zip", "rb")}
data = {
    "name": "MyApp",
    "version": "1.0.0",
    "description": "My awesome DApp"
}

response = requests.post(
    f"{BASE_URL}/dapps/deploy",
    headers={"Authorization": f"Bearer {JWT_TOKEN}"},
    files=files,
    data=data
)
print(json.dumps(response.json(), indent=2))
```

### JavaScript Example

```javascript
const axios = require('axios');

const client = axios.create({
  baseURL: 'https://api.zhtp.network',
  headers: {
    'Authorization': `Bearer ${process.env.JWT_TOKEN}`,
    'Content-Type': 'application/json'
  }
});

// Get network info
async function getNetworkInfo() {
  try {
    const response = await client.get('/network/info');
    console.log(response.data);
  } catch (error) {
    console.error('Error:', error.response.data);
  }
}

// Deploy DApp
async function deployDApp() {
  const formData = new FormData();
  formData.append('name', 'MyApp');
  formData.append('package', fs.createReadStream('myapp.zip'));
  
  try {
    const response = await client.post('/dapps/deploy', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    });
    console.log(response.data);
  } catch (error) {
    console.error('Error:', error.response.data);
  }
}

getNetworkInfo();
```

---

## 📚 Next Steps

- **[JavaScript SDK Reference](javascript.md)** - Client-side development with JS/TS
- **[Python SDK Reference](python.md)** - Server-side development with Python
- **[Rust SDK Reference](rust.md)** - Native development with Rust
- **[Smart Contract Guide](../guides/smart-contracts.md)** - Advanced contract development
- **[DApp Templates](../templates/)** - Ready-to-use DApp templates
- **[Community Discord](https://discord.gg/zhtp)** - Get help and connect with developers

For more examples and tutorials, visit our **[Developer Portal](../README.md)**.

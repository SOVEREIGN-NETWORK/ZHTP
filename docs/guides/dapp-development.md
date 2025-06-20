# 🚀 Decentralized DApp Development Guide - Complete Traditional Internet Replacement

**Complete guide to building decentralized applications that completely replace traditional web infrastructure**

## Overview

ZHTP DApps completely replace traditional web technologies and infrastructure. Instead of HTML served from web servers, DNS resolution, SSL certificates, and HTTP protocols, ZHTP DApps use blockchain DNS, zero-knowledge certificates, decentralized storage, and the native ZHTP protocol.

> **Key Principle**: ZHTP DApps use ZERO traditional internet infrastructure - no web servers, no DNS servers, no SSL certificates, no HTTP/HTTPS, no cloud hosting.

## 🏗️ Decentralized Architecture Overview

### ZHTP DApp Stack (Traditional Internet Replacement)

```
┌─────────────────────────────────────┐
│     Decentralized Frontend Layer    │
│  Content served from distributed    │
│  storage, no web servers required   │
├─────────────────────────────────────┤
│      Native ZHTP Protocol Layer     │
│  ZK proofs, blockchain DNS,         │
│  anonymous routing (no HTTP/HTTPS)  │
├─────────────────────────────────────┤
│     Zero-Knowledge Contract Layer   │
│  Quantum-resistant smart contracts, │
│  privacy-preserving business logic  │
├─────────────────────────────────────┤
│     Decentralized Storage Layer     │
│  DHT-based content distribution,    │
│  redundancy, incentivized hosting   │
├─────────────────────────────────────┤
│        Blockchain DNS Layer         │
│  Decentralized domain resolution,   │
│  censorship-resistant naming        │
└─────────────────────────────────────┘
```

### Core Components (Traditional Replacements)

1. **Decentralized Frontend**: Content distributed across storage network (replaces web servers)
2. **Blockchain DNS**: Domain resolution via smart contracts (replaces DNS servers)  
3. **ZK Certificates**: Quantum-resistant certificates (replaces SSL/TLS certificates)
4. **Anonymous Routing**: Built-in privacy layer (replaces VPNs)
5. **Decentralized Storage**: Incentivized content hosting (replaces cloud hosting)
6. **Native Protocol**: ZHTP protocol communication (replaces HTTP/HTTPS)

## 🎯 Decentralized Development Workflow

### 1. Decentralized Project Setup (No Traditional Infrastructure)

```bash
# Create new ZHTP DApp project (no web server needed)
npx create-zhtp-app my-dapp --template decentralized
cd my-dapp

# Install ZHTP dependencies (no HTTP libraries)
npm install  # Only @zhtp/sdk, no axios/fetch/http libraries

# Start decentralized development (no HTTP server)
npm run dev  # Connects to ZHTP network directly
```
```

### Project Structure

```
my-dapp/
├── src/
│   ├── contracts/          # Smart contracts
│   ├── frontend/           # Web interface
│   ├── components/         # Reusable UI components
│   ├── utils/              # Helper functions
│   └── config/             # Configuration files
├── public/                 # Static assets
├── tests/                  # Test files
├── scripts/                # Deployment scripts
├── docs/                   # Documentation
├── package.json
├── zhtp.config.js
└── README.md
```

### 2. Configuration

```javascript
// zhtp.config.js
module.exports = {
  networks: {
    mainnet: {
      url: "https://mainnet.zhtp.network",
      chainId: 1337
    },
    testnet: {
      url: "https://testnet.zhtp.network",
      chainId: 31337
    },
    local: {
      url: "http://localhost:8080",
      chainId: 1337
    }
  },
  contracts: {
    outputDir: "./src/contracts/artifacts",
    includes: ["./src/contracts/**/*.sol"]
  },
  dapp: {
    name: "My Awesome DApp",
    version: "1.0.0",
    description: "A decentralized application on ZHTP",
    author: "Your Name",
    license: "MIT"
  },
  build: {
    outputDir: "./dist",
    compression: true,
    optimization: true
  }
};
```

## 📱 Building Frontend Components

### Basic ZHTP Integration

```html
<!-- public/index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My ZHTP DApp</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <header>
            <h1>My ZHTP DApp</h1>
            <div id="wallet-info"></div>
        </header>
        
        <main>
            <section id="connect-section">
                <button id="connect-btn">Connect Wallet</button>
            </section>
            
            <section id="app-content" style="display: none;">
                <div id="user-profile"></div>
                <div id="main-features"></div>
            </section>
        </main>
    </div>
    
    <script src="https://cdn.zhtp.network/sdk/zhtp.min.js"></script>
    <script src="app.js"></script>
</body>
</html>
```

### Core JavaScript Application

```javascript
// src/frontend/app.js
class ZhtpDApp {
    constructor() {
        this.zhtp = null;
        this.user = null;
        this.contracts = {};
        this.init();
    }
    
    async init() {
        try {
            // Initialize ZHTP SDK
            this.zhtp = new ZHTP({
                network: 'testnet',
                autoConnect: false
            });
            
            // Setup event listeners
            this.setupEventListeners();
            
            // Check for existing connection
            if (await this.zhtp.isConnected()) {
                await this.handleConnection();
            }
            
            console.log('DApp initialized successfully');
        } catch (error) {
            console.error('Failed to initialize DApp:', error);
            this.showError('Failed to initialize application');
        }
    }
    
    setupEventListeners() {
        // Connect wallet button
        document.getElementById('connect-btn').addEventListener('click', () => {
            this.connectWallet();
        });
        
        // Account change listener
        if (this.zhtp) {
            this.zhtp.on('accountsChanged', (accounts) => {
                if (accounts.length === 0) {
                    this.handleDisconnection();
                } else {
                    this.handleAccountChange(accounts[0]);
                }
            });
            
            this.zhtp.on('chainChanged', (chainId) => {
                this.handleChainChange(chainId);
            });
        }
    }
    
    async connectWallet() {
        try {
            const accounts = await this.zhtp.connect();
            if (accounts && accounts.length > 0) {
                await this.handleConnection();
            }
        } catch (error) {
            console.error('Failed to connect wallet:', error);
            this.showError('Failed to connect wallet');
        }
    }
    
    async handleConnection() {
        try {
            // Get user account
            this.user = await this.zhtp.getAccount();
            
            // Load smart contracts
            await this.loadContracts();
            
            // Update UI
            this.updateConnectionUI();
            
            // Load user data
            await this.loadUserData();
            
            console.log('Connected to ZHTP network');
        } catch (error) {
            console.error('Failed to handle connection:', error);
            this.showError('Failed to complete connection');
        }
    }
    
    async loadContracts() {
        // Load main application contract
        this.contracts.main = await this.zhtp.contract(
            CONTRACT_ADDRESSES.main,
            CONTRACT_ABIS.main
        );
        
        // Load additional contracts as needed
        this.contracts.token = await this.zhtp.contract(
            CONTRACT_ADDRESSES.token,
            CONTRACT_ABIS.token
        );
    }
    
    updateConnectionUI() {
        // Hide connect section
        document.getElementById('connect-section').style.display = 'none';
        
        // Show app content
        document.getElementById('app-content').style.display = 'block';
        
        // Update wallet info
        const walletInfo = document.getElementById('wallet-info');
        walletInfo.innerHTML = `
            <div class="wallet-connected">
                <span class="wallet-address">${this.truncateAddress(this.user.address)}</span>
                <span class="network-indicator">${this.zhtp.network}</span>
                <button id="disconnect-btn">Disconnect</button>
            </div>
        `;
        
        // Add disconnect listener
        document.getElementById('disconnect-btn').addEventListener('click', () => {
            this.disconnect();
        });
    }
    
    async loadUserData() {
        try {
            // Get user balance
            const balance = await this.zhtp.getBalance();
            
            // Get user profile from contract
            const profile = await this.contracts.main.getUserProfile(this.user.address);
            
            // Update UI with user data
            this.updateUserProfile(balance, profile);
            
        } catch (error) {
            console.error('Failed to load user data:', error);
        }
    }
    
    updateUserProfile(balance, profile) {
        const userProfileEl = document.getElementById('user-profile');
        userProfileEl.innerHTML = `
            <div class="user-card">
                <h3>Your Profile</h3>
                <div class="profile-info">
                    <div class="balance">
                        <label>Balance:</label>
                        <span>${this.formatBalance(balance)} ZHTP</span>
                    </div>
                    <div class="profile-data">
                        <label>Username:</label>
                        <span>${profile.username || 'Not set'}</span>
                    </div>
                    <div class="profile-data">
                        <label>Level:</label>
                        <span>${profile.level || 1}</span>
                    </div>
                </div>
                <button id="edit-profile-btn">Edit Profile</button>
            </div>
        `;
        
        // Add edit profile listener
        document.getElementById('edit-profile-btn').addEventListener('click', () => {
            this.showEditProfileModal();
        });
    }
    
    async disconnect() {
        await this.zhtp.disconnect();
        this.handleDisconnection();
    }
    
    handleDisconnection() {
        this.user = null;
        this.contracts = {};
        
        // Reset UI
        document.getElementById('connect-section').style.display = 'block';
        document.getElementById('app-content').style.display = 'none';
        document.getElementById('wallet-info').innerHTML = '';
    }
    
    // Utility functions
    truncateAddress(address) {
        return `${address.slice(0, 6)}...${address.slice(-4)}`;
    }
    
    formatBalance(balance) {
        return (parseFloat(balance) / 1e18).toFixed(4);
    }
    
    showError(message) {
        // Implement error notification
        console.error(message);
        // You could show a toast notification here
    }
}

// Contract addresses and ABIs
const CONTRACT_ADDRESSES = {
    main: '0x...',
    token: '0x...'
};

const CONTRACT_ABIS = {
    main: [...], // ABI array
    token: [...] // ABI array
};

// Initialize DApp when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new ZhtpDApp();
});
```

### React Component Example

```jsx
// src/components/DAppConnection.jsx
import React, { useState, useEffect } from 'react';
import { ZHTP } from '@zhtp/sdk';

const DAppConnection = ({ onConnect, onDisconnect }) => {
    const [zhtp, setZhtp] = useState(null);
    const [isConnected, setIsConnected] = useState(false);
    const [account, setAccount] = useState(null);
    const [balance, setBalance] = useState('0');
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        initializeZhtp();
    }, []);

    const initializeZhtp = async () => {
        try {
            const zhtpInstance = new ZHTP({
                network: process.env.REACT_APP_ZHTP_NETWORK || 'testnet'
            });
            setZhtp(zhtpInstance);

            // Check if already connected
            if (await zhtpInstance.isConnected()) {
                const accounts = await zhtpInstance.getAccounts();
                if (accounts.length > 0) {
                    handleConnectionSuccess(zhtpInstance, accounts[0]);
                }
            }
        } catch (error) {
            console.error('Failed to initialize ZHTP:', error);
        }
    };

    const connectWallet = async () => {
        if (!zhtp) return;

        setLoading(true);
        try {
            const accounts = await zhtp.connect();
            if (accounts && accounts.length > 0) {
                handleConnectionSuccess(zhtp, accounts[0]);
            }
        } catch (error) {
            console.error('Failed to connect:', error);
        } finally {
            setLoading(false);
        }
    };

    const handleConnectionSuccess = async (zhtpInstance, accountAddress) => {
        setAccount(accountAddress);
        setIsConnected(true);

        // Get balance
        try {
            const userBalance = await zhtpInstance.getBalance();
            setBalance(userBalance);
        } catch (error) {
            console.error('Failed to get balance:', error);
        }

        // Notify parent component
        if (onConnect) {
            onConnect(zhtpInstance, accountAddress);
        }
    };

    const disconnect = async () => {
        if (zhtp) {
            await zhtp.disconnect();
        }
        setIsConnected(false);
        setAccount(null);
        setBalance('0');

        if (onDisconnect) {
            onDisconnect();
        }
    };

    if (!isConnected) {
        return (
            <div className="connection-prompt">
                <h3>Connect to ZHTP</h3>
                <p>Connect your wallet to use this DApp</p>
                <button 
                    onClick={connectWallet} 
                    disabled={loading || !zhtp}
                    className="connect-button"
                >
                    {loading ? 'Connecting...' : 'Connect Wallet'}
                </button>
            </div>
        );
    }

    return (
        <div className="wallet-info">
            <div className="account-info">
                <span className="address">
                    {account.slice(0, 6)}...{account.slice(-4)}
                </span>
                <span className="balance">
                    {(parseFloat(balance) / 1e18).toFixed(4)} ZHTP
                </span>
            </div>
            <button onClick={disconnect} className="disconnect-button">
                Disconnect
            </button>
        </div>
    );
};

export default DAppConnection;
```

## 🔗 Smart Contract Integration

### Contract Interaction Component

```jsx
// src/components/ContractInteraction.jsx
import React, { useState, useEffect } from 'react';

const ContractInteraction = ({ zhtp, account }) => {
    const [contract, setContract] = useState(null);
    const [userData, setUserData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [transactions, setTransactions] = useState([]);

    useEffect(() => {
        if (zhtp && account) {
            loadContract();
        }
    }, [zhtp, account]);

    const loadContract = async () => {
        try {
            const contractInstance = await zhtp.contract(
                process.env.REACT_APP_CONTRACT_ADDRESS,
                CONTRACT_ABI
            );
            setContract(contractInstance);
            loadUserData(contractInstance);
        } catch (error) {
            console.error('Failed to load contract:', error);
        }
    };

    const loadUserData = async (contractInstance) => {
        try {
            const data = await contractInstance.getUserData(account);
            setUserData(data);
        } catch (error) {
            console.error('Failed to load user data:', error);
        }
    };

    const updateUserProfile = async (username, bio) => {
        if (!contract) return;

        setLoading(true);
        try {
            const tx = await contract.updateProfile(username, bio);
            
            // Add to pending transactions
            setTransactions(prev => [...prev, {
                hash: tx.hash,
                type: 'Profile Update',
                status: 'pending',
                timestamp: Date.now()
            }]);

            // Wait for confirmation
            const receipt = await tx.wait();
            
            // Update transaction status
            setTransactions(prev => 
                prev.map(t => 
                    t.hash === tx.hash 
                        ? { ...t, status: 'confirmed', blockNumber: receipt.blockNumber }
                        : t
                )
            );

            // Reload user data
            await loadUserData(contract);
            
        } catch (error) {
            console.error('Failed to update profile:', error);
            
            // Update transaction status
            setTransactions(prev => 
                prev.map(t => 
                    t.hash === tx.hash 
                        ? { ...t, status: 'failed', error: error.message }
                        : t
                )
            );
        } finally {
            setLoading(false);
        }
    };

    const stakeTokens = async (amount) => {
        if (!contract) return;

        setLoading(true);
        try {
            const tx = await contract.stake(amount);
            
            setTransactions(prev => [...prev, {
                hash: tx.hash,
                type: 'Stake Tokens',
                amount: amount,
                status: 'pending',
                timestamp: Date.now()
            }]);

            const receipt = await tx.wait();
            
            setTransactions(prev => 
                prev.map(t => 
                    t.hash === tx.hash 
                        ? { ...t, status: 'confirmed', blockNumber: receipt.blockNumber }
                        : t
                )
            );

            await loadUserData(contract);
            
        } catch (error) {
            console.error('Failed to stake tokens:', error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="contract-interaction">
            <div className="user-profile">
                <h3>Your Profile</h3>
                {userData ? (
                    <div className="profile-display">
                        <p><strong>Username:</strong> {userData.username}</p>
                        <p><strong>Bio:</strong> {userData.bio}</p>
                        <p><strong>Staked:</strong> {userData.stakedAmount} ZHTP</p>
                        <p><strong>Rewards:</strong> {userData.pendingRewards} ZHTP</p>
                    </div>
                ) : (
                    <p>Loading profile...</p>
                )}
                
                <ProfileEditor 
                    onUpdate={updateUserProfile}
                    loading={loading}
                    currentData={userData}
                />
            </div>

            <div className="staking-section">
                <h3>Staking</h3>
                <StakingInterface 
                    onStake={stakeTokens}
                    loading={loading}
                    userData={userData}
                />
            </div>

            <div className="transaction-history">
                <h3>Recent Transactions</h3>
                <TransactionList transactions={transactions} />
            </div>
        </div>
    );
};

// Profile Editor Component
const ProfileEditor = ({ onUpdate, loading, currentData }) => {
    const [username, setUsername] = useState('');
    const [bio, setBio] = useState('');
    const [showEditor, setShowEditor] = useState(false);

    useEffect(() => {
        if (currentData) {
            setUsername(currentData.username || '');
            setBio(currentData.bio || '');
        }
    }, [currentData]);

    const handleSubmit = async (e) => {
        e.preventDefault();
        await onUpdate(username, bio);
        setShowEditor(false);
    };

    return (
        <div className="profile-editor">
            {!showEditor ? (
                <button 
                    onClick={() => setShowEditor(true)}
                    className="edit-button"
                >
                    Edit Profile
                </button>
            ) : (
                <form onSubmit={handleSubmit} className="edit-form">
                    <div className="form-group">
                        <label>Username:</label>
                        <input
                            type="text"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            required
                        />
                    </div>
                    <div className="form-group">
                        <label>Bio:</label>
                        <textarea
                            value={bio}
                            onChange={(e) => setBio(e.target.value)}
                            rows={3}
                        />
                    </div>
                    <div className="form-actions">
                        <button type="submit" disabled={loading}>
                            {loading ? 'Updating...' : 'Update Profile'}
                        </button>
                        <button 
                            type="button" 
                            onClick={() => setShowEditor(false)}
                        >
                            Cancel
                        </button>
                    </div>
                </form>
            )}
        </div>
    );
};

// Contract ABI
const CONTRACT_ABI = [
    {
        "inputs": [
            {"name": "username", "type": "string"},
            {"name": "bio", "type": "string"}
        ],
        "name": "updateProfile",
        "outputs": [],
        "type": "function"
    },
    {
        "inputs": [{"name": "amount", "type": "uint256"}],
        "name": "stake",
        "outputs": [],
        "type": "function"
    },
    {
        "inputs": [{"name": "user", "type": "address"}],
        "name": "getUserData",
        "outputs": [
            {"name": "username", "type": "string"},
            {"name": "bio", "type": "string"},
            {"name": "stakedAmount", "type": "uint256"},
            {"name": "pendingRewards", "type": "uint256"}
        ],
        "type": "function"
    }
];

export default ContractInteraction;
```

## 🔐 Zero-Knowledge Features

### ZK Identity Verification

```javascript
// src/utils/zkIdentity.js
import { ZkProof } from '@zhtp/sdk';

class ZKIdentityManager {
    constructor(zhtp) {
        this.zhtp = zhtp;
        this.zkProof = new ZkProof();
    }

    async verifyAge(actualAge, minimumAge) {
        try {
            // Generate proof that actualAge >= minimumAge without revealing actualAge
            const inputs = {
                age: actualAge,
                minAge: minimumAge,
                userAddress: this.zhtp.account
            };

            const proof = await this.zkProof.generate('age_verification', inputs);
            
            return proof;
        } catch (error) {
            console.error('Failed to generate age proof:', error);
            throw error;
        }
    }

    async verifyIdentityAttributes(attributes, requirements) {
        try {
            // Generate proof that attributes meet requirements
            const inputs = {
                ...attributes,
                ...requirements,
                userAddress: this.zhtp.account
            };

            const proof = await this.zkProof.generate('identity_verification', inputs);
            
            return proof;
        } catch (error) {
            console.error('Failed to generate identity proof:', error);
            throw error;
        }
    }

    async submitVerificationProof(proof, contractAddress) {
        try {
            const contract = await this.zhtp.contract(contractAddress, VERIFICATION_ABI);
            const tx = await contract.submitVerification(proof.serialize());
            
            return tx;
        } catch (error) {
            console.error('Failed to submit verification proof:', error);
            throw error;
        }
    }
}

// Age Verification Component
class AgeVerificationComponent {
    constructor(zkManager) {
        this.zkManager = zkManager;
        this.setupUI();
    }

    setupUI() {
        this.container = document.createElement('div');
        this.container.className = 'age-verification';
        this.container.innerHTML = `
            <div class="verification-form">
                <h3>Age Verification</h3>
                <p>Prove you are at least 18 years old without revealing your exact age.</p>
                
                <div class="form-group">
                    <label>Your Age (this will not be revealed):</label>
                    <input type="number" id="actual-age" min="1" max="120" required>
                </div>
                
                <div class="form-group">
                    <label>Minimum Required Age:</label>
                    <input type="number" id="min-age" value="18" readonly>
                </div>
                
                <button id="verify-age-btn">Generate Verification Proof</button>
                
                <div id="verification-result" style="display: none;"></div>
            </div>
        `;

        // Add event listeners
        this.container.querySelector('#verify-age-btn').addEventListener('click', () => {
            this.performVerification();
        });
    }

    async performVerification() {
        const actualAge = parseInt(this.container.querySelector('#actual-age').value);
        const minimumAge = parseInt(this.container.querySelector('#min-age').value);
        const resultDiv = this.container.querySelector('#verification-result');

        if (!actualAge || actualAge < 1) {
            this.showError('Please enter a valid age');
            return;
        }

        try {
            this.showLoading('Generating zero-knowledge proof...');
            
            // Generate ZK proof
            const proof = await this.zkManager.verifyAge(actualAge, minimumAge);
            
            this.showLoading('Submitting proof to blockchain...');
            
            // Submit to smart contract
            const tx = await this.zkManager.submitVerificationProof(
                proof, 
                AGE_VERIFICATION_CONTRACT_ADDRESS
            );
            
            this.showSuccess(`
                <h4>Verification Successful!</h4>
                <p>Your age has been verified without revealing the exact number.</p>
                <p><small>Transaction: ${tx.hash}</small></p>
            `);
            
        } catch (error) {
            console.error('Verification failed:', error);
            this.showError('Verification failed. Please try again.');
        }
    }

    showLoading(message) {
        const resultDiv = this.container.querySelector('#verification-result');
        resultDiv.style.display = 'block';
        resultDiv.className = 'verification-result loading';
        resultDiv.innerHTML = `<p>${message}</p>`;
    }

    showSuccess(html) {
        const resultDiv = this.container.querySelector('#verification-result');
        resultDiv.style.display = 'block';
        resultDiv.className = 'verification-result success';
        resultDiv.innerHTML = html;
    }

    showError(message) {
        const resultDiv = this.container.querySelector('#verification-result');
        resultDiv.style.display = 'block';
        resultDiv.className = 'verification-result error';
        resultDiv.innerHTML = `<p>${message}</p>`;
    }

    render(parentElement) {
        parentElement.appendChild(this.container);
    }
}

export { ZKIdentityManager, AgeVerificationComponent };
```

### Private Messaging System

```javascript
// src/features/PrivateMessaging.js
import { ZkProof } from '@zhtp/sdk';

class PrivateMessagingSystem {
    constructor(zhtp) {
        this.zhtp = zhtp;
        this.zkProof = new ZkProof();
        this.messageContract = null;
    }

    async initialize() {
        this.messageContract = await this.zhtp.contract(
            MESSAGING_CONTRACT_ADDRESS,
            MESSAGING_ABI
        );
    }

    async sendPrivateMessage(recipientAddress, message) {
        try {
            // Encrypt message
            const encryptedMessage = await this.encryptMessage(message, recipientAddress);
            
            // Generate proof that sender has permission to message recipient
            const authProof = await this.generateMessagingProof(recipientAddress);
            
            // Send message to contract
            const tx = await this.messageContract.sendMessage(
                recipientAddress,
                encryptedMessage,
                authProof.serialize()
            );
            
            return tx;
        } catch (error) {
            console.error('Failed to send private message:', error);
            throw error;
        }
    }

    async encryptMessage(message, recipientAddress) {
        // Get recipient's public key
        const recipientPubKey = await this.messageContract.getPublicKey(recipientAddress);
        
        // Encrypt message using ECIES
        const encrypted = await this.zhtp.crypto.encrypt(message, recipientPubKey);
        
        return encrypted;
    }

    async generateMessagingProof(recipientAddress) {
        // Generate proof that sender is authorized to message recipient
        // This could verify subscription status, relationship, etc.
        const inputs = {
            sender: this.zhtp.account,
            recipient: recipientAddress,
            timestamp: Math.floor(Date.now() / 1000)
        };
        
        const proof = await this.zkProof.generate('messaging_auth', inputs);
        return proof;
    }

    async getMessages(limit = 50) {
        try {
            const encryptedMessages = await this.messageContract.getMessages(
                this.zhtp.account,
                limit
            );
            
            // Decrypt messages
            const decryptedMessages = await Promise.all(
                encryptedMessages.map(msg => this.decryptMessage(msg))
            );
            
            return decryptedMessages;
        } catch (error) {
            console.error('Failed to get messages:', error);
            throw error;
        }
    }

    async decryptMessage(encryptedMessage) {
        try {
            const decrypted = await this.zhtp.crypto.decrypt(
                encryptedMessage.content,
                this.zhtp.privateKey
            );
            
            return {
                id: encryptedMessage.id,
                from: encryptedMessage.from,
                to: encryptedMessage.to,
                content: decrypted,
                timestamp: encryptedMessage.timestamp,
                verified: encryptedMessage.verified
            };
        } catch (error) {
            console.error('Failed to decrypt message:', error);
            return {
                ...encryptedMessage,
                content: '[Failed to decrypt]',
                error: true
            };
        }
    }
}

// Messaging UI Component
class MessagingInterface {
    constructor(messagingSystem) {
        this.messaging = messagingSystem;
        this.messages = [];
        this.setupUI();
    }

    setupUI() {
        this.container = document.createElement('div');
        this.container.className = 'messaging-interface';
        this.container.innerHTML = `
            <div class="messaging-container">
                <div class="message-list" id="message-list">
                    <div class="loading">Loading messages...</div>
                </div>
                
                <div class="message-compose">
                    <div class="compose-form">
                        <input 
                            type="text" 
                            id="recipient-address" 
                            placeholder="Recipient address (0x...)"
                        >
                        <textarea 
                            id="message-content" 
                            placeholder="Type your private message..."
                            rows="3"
                        ></textarea>
                        <button id="send-message-btn">Send Private Message</button>
                    </div>
                </div>
            </div>
        `;

        // Add event listeners
        this.container.querySelector('#send-message-btn').addEventListener('click', () => {
            this.sendMessage();
        });

        // Auto-refresh messages
        setInterval(() => {
            this.loadMessages();
        }, 10000); // Refresh every 10 seconds
    }

    async loadMessages() {
        try {
            this.messages = await this.messaging.getMessages();
            this.renderMessages();
        } catch (error) {
            console.error('Failed to load messages:', error);
        }
    }

    renderMessages() {
        const messageList = this.container.querySelector('#message-list');
        
        if (this.messages.length === 0) {
            messageList.innerHTML = '<div class="no-messages">No messages yet</div>';
            return;
        }

        messageList.innerHTML = this.messages.map(msg => `
            <div class="message ${msg.error ? 'error' : ''} ${msg.from === this.messaging.zhtp.account ? 'sent' : 'received'}">
                <div class="message-header">
                    <span class="sender">${this.formatAddress(msg.from)}</span>
                    <span class="timestamp">${this.formatTimestamp(msg.timestamp)}</span>
                    ${msg.verified ? '<span class="verified">✓</span>' : ''}
                </div>
                <div class="message-content">${msg.content}</div>
            </div>
        `).join('');
    }

    async sendMessage() {
        const recipientInput = this.container.querySelector('#recipient-address');
        const contentInput = this.container.querySelector('#message-content');
        const sendBtn = this.container.querySelector('#send-message-btn');

        const recipient = recipientInput.value.trim();
        const content = contentInput.value.trim();

        if (!recipient || !content) {
            alert('Please enter both recipient address and message content');
            return;
        }

        sendBtn.disabled = true;
        sendBtn.textContent = 'Sending...';

        try {
            const tx = await this.messaging.sendPrivateMessage(recipient, content);
            
            // Clear form
            recipientInput.value = '';
            contentInput.value = '';
            
            // Refresh messages
            await this.loadMessages();
            
            console.log('Message sent:', tx.hash);
        } catch (error) {
            console.error('Failed to send message:', error);
            alert('Failed to send message. Please try again.');
        } finally {
            sendBtn.disabled = false;
            sendBtn.textContent = 'Send Private Message';
        }
    }

    formatAddress(address) {
        return `${address.slice(0, 6)}...${address.slice(-4)}`;
    }

    formatTimestamp(timestamp) {
        return new Date(timestamp * 1000).toLocaleString();
    }

    render(parentElement) {
        parentElement.appendChild(this.container);
        this.loadMessages();
    }
}

export { PrivateMessagingSystem, MessagingInterface };
```

## 💾 Data Storage Solutions

### IPFS Integration

```javascript
// src/utils/storage.js
import { IPFS } from '@zhtp/sdk';

class DecentralizedStorage {
    constructor() {
        this.ipfs = new IPFS();
    }

    async initialize() {
        await this.ipfs.connect();
    }

    // Store file and return IPFS hash
    async storeFile(file) {
        try {
            const result = await this.ipfs.add(file);
            return result.hash;
        } catch (error) {
            console.error('Failed to store file:', error);
            throw error;
        }
    }

    // Store JSON data
    async storeJSON(data) {
        try {
            const jsonString = JSON.stringify(data);
            const file = new Blob([jsonString], { type: 'application/json' });
            return await this.storeFile(file);
        } catch (error) {
            console.error('Failed to store JSON:', error);
            throw error;
        }
    }

    // Retrieve file by hash
    async getFile(hash) {
        try {
            const result = await this.ipfs.get(hash);
            return result;
        } catch (error) {
            console.error('Failed to get file:', error);
            throw error;
        }
    }

    // Retrieve and parse JSON data
    async getJSON(hash) {
        try {
            const file = await this.getFile(hash);
            const text = await file.text();
            return JSON.parse(text);
        } catch (error) {
            console.error('Failed to get JSON:', error);
            throw error;
        }
    }

    // Pin content to ensure availability
    async pinContent(hash) {
        try {
            await this.ipfs.pin(hash);
        } catch (error) {
            console.error('Failed to pin content:', error);
            throw error;
        }
    }
}

// File Upload Component
class FileUploadComponent {
    constructor(storage, onUpload) {
        this.storage = storage;
        this.onUpload = onUpload;
        this.setupUI();
    }

    setupUI() {
        this.container = document.createElement('div');
        this.container.className = 'file-upload';
        this.container.innerHTML = `
            <div class="upload-area" id="upload-area">
                <div class="upload-placeholder">
                    <p>Drag & drop files here or click to select</p>
                    <input type="file" id="file-input" multiple hidden>
                    <button id="select-files-btn">Select Files</button>
                </div>
                <div class="upload-progress" id="upload-progress" style="display: none;">
                    <div class="progress-bar">
                        <div class="progress-fill" id="progress-fill"></div>
                    </div>
                    <div class="progress-text" id="progress-text">Uploading...</div>
                </div>
            </div>
            <div class="uploaded-files" id="uploaded-files"></div>
        `;

        this.setupEventListeners();
    }

    setupEventListeners() {
        const uploadArea = this.container.querySelector('#upload-area');
        const fileInput = this.container.querySelector('#file-input');
        const selectBtn = this.container.querySelector('#select-files-btn');

        // Click to select files
        selectBtn.addEventListener('click', () => {
            fileInput.click();
        });

        // File input change
        fileInput.addEventListener('change', (e) => {
            this.handleFiles(Array.from(e.target.files));
        });

        // Drag and drop
        uploadArea.addEventListener('dragover', (e) => {
            e.preventDefault();
            uploadArea.classList.add('dragover');
        });

        uploadArea.addEventListener('dragleave', () => {
            uploadArea.classList.remove('dragover');
        });

        uploadArea.addEventListener('drop', (e) => {
            e.preventDefault();
            uploadArea.classList.remove('dragover');
            this.handleFiles(Array.from(e.dataTransfer.files));
        });
    }

    async handleFiles(files) {
        const progressDiv = this.container.querySelector('#upload-progress');
        const uploadedDiv = this.container.querySelector('#uploaded-files');

        progressDiv.style.display = 'block';

        for (let i = 0; i < files.length; i++) {
            const file = files[i];
            
            try {
                // Update progress
                this.updateProgress((i / files.length) * 100, `Uploading ${file.name}...`);
                
                // Store file in IPFS
                const hash = await this.storage.storeFile(file);
                
                // Pin the content
                await this.storage.pinContent(hash);
                
                // Add to uploaded files list
                this.addUploadedFile(file.name, hash, file.size);
                
                // Notify parent component
                if (this.onUpload) {
                    this.onUpload({
                        name: file.name,
                        hash: hash,
                        size: file.size,
                        type: file.type
                    });
                }
                
            } catch (error) {
                console.error(`Failed to upload ${file.name}:`, error);
                this.addUploadError(file.name, error.message);
            }
        }

        this.updateProgress(100, 'Upload complete!');
        setTimeout(() => {
            progressDiv.style.display = 'none';
        }, 2000);
    }

    updateProgress(percent, text) {
        const progressFill = this.container.querySelector('#progress-fill');
        const progressText = this.container.querySelector('#progress-text');
        
        progressFill.style.width = `${percent}%`;
        progressText.textContent = text;
    }

    addUploadedFile(name, hash, size) {
        const uploadedDiv = this.container.querySelector('#uploaded-files');
        
        const fileDiv = document.createElement('div');
        fileDiv.className = 'uploaded-file';
        fileDiv.innerHTML = `
            <div class="file-info">
                <span class="file-name">${name}</span>
                <span class="file-size">${this.formatFileSize(size)}</span>
            </div>
            <div class="file-actions">
                <span class="file-hash" title="${hash}">${hash.slice(0, 12)}...</span>
                <button onclick="navigator.clipboard.writeText('${hash}')">Copy Hash</button>
                <a href="https://ipfs.io/ipfs/${hash}" target="_blank">View</a>
            </div>
        `;
        
        uploadedDiv.appendChild(fileDiv);
    }

    addUploadError(name, error) {
        const uploadedDiv = this.container.querySelector('#uploaded-files');
        
        const errorDiv = document.createElement('div');
        errorDiv.className = 'upload-error';
        errorDiv.innerHTML = `
            <span class="file-name">${name}</span>
            <span class="error-message">Error: ${error}</span>
        `;
        
        uploadedDiv.appendChild(errorDiv);
    }

    formatFileSize(bytes) {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    render(parentElement) {
        parentElement.appendChild(this.container);
    }
}

export { DecentralizedStorage, FileUploadComponent };
```

## 🎨 UI/UX Best Practices

### Responsive Design

```css
/* src/styles/responsive.css */

/* Mobile-first approach */
.dapp-container {
    max-width: 100%;
    padding: 1rem;
    margin: 0 auto;
}

/* Connection status */
.connection-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border-radius: 8px;
    margin-bottom: 1rem;
}

.connection-status.connected {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
}

.connection-status.disconnected {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
}

/* Wallet info */
.wallet-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #dee2e6;
}

.wallet-address {
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    background: #e9ecef;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
}

.wallet-balance {
    font-weight: 600;
    color: #28a745;
}

/* Transaction status */
.transaction-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    margin-bottom: 0.5rem;
}

.transaction-status {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
}

.transaction-status.pending {
    background: #fff3cd;
    color: #856404;
}

.transaction-status.confirmed {
    background: #d4edda;
    color: #155724;
}

.transaction-status.failed {
    background: #f8d7da;
    color: #721c24;
}

/* Form elements */
.form-group {
    margin-bottom: 1rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
}

.form-group input,
.form-group textarea,
.form-group select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ced4da;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

.form-group input:focus,
.form-group textarea:focus,
.form-group select:focus {
    outline: 0;
    border-color: #80bdff;
    box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

/* Buttons */
.btn {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    margin-bottom: 0;
    font-size: 1rem;
    font-weight: 400;
    line-height: 1.5;
    text-align: center;
    text-decoration: none;
    vertical-align: middle;
    cursor: pointer;
    border: 1px solid transparent;
    border-radius: 6px;
    transition: all 0.15s ease-in-out;
}

.btn:disabled {
    opacity: 0.65;
    cursor: not-allowed;
}

.btn-primary {
    color: #fff;
    background-color: #007bff;
    border-color: #007bff;
}

.btn-primary:hover:not(:disabled) {
    background-color: #0056b3;
    border-color: #004085;
}

.btn-success {
    color: #fff;
    background-color: #28a745;
    border-color: #28a745;
}

.btn-success:hover:not(:disabled) {
    background-color: #1e7e34;
    border-color: #1c7430;
}

.btn-danger {
    color: #fff;
    background-color: #dc3545;
    border-color: #dc3545;
}

.btn-danger:hover:not(:disabled) {
    background-color: #c82333;
    border-color: #bd2130;
}

/* Loading states */
.loading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.loading-spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid #f3f3f3;
    border-top: 2px solid #007bff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

/* Responsive breakpoints */
@media (min-width: 576px) {
    .dapp-container {
        max-width: 540px;
    }
}

@media (min-width: 768px) {
    .dapp-container {
        max-width: 720px;
        padding: 2rem;
    }
    
    .wallet-info {
        justify-content: space-between;
    }
}

@media (min-width: 992px) {
    .dapp-container {
        max-width: 960px;
    }
    
    .form-row {
        display: flex;
        gap: 1rem;
    }
    
    .form-row .form-group {
        flex: 1;
    }
}

@media (min-width: 1200px) {
    .dapp-container {
        max-width: 1140px;
    }
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
    :root {
        --bg-color: #1a1a1a;
        --text-color: #ffffff;
        --border-color: #333333;
        --card-bg: #2d2d2d;
    }
    
    body {
        background-color: var(--bg-color);
        color: var(--text-color);
    }
    
    .wallet-info,
    .transaction-item {
        background-color: var(--card-bg);
        border-color: var(--border-color);
    }
    
    .form-group input,
    .form-group textarea,
    .form-group select {
        background-color: var(--card-bg);
        border-color: var(--border-color);
        color: var(--text-color);
    }
}
```

## 🚀 Deployment and Distribution

### Build Configuration

```javascript
// webpack.config.js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = (env, argv) => {
    const isProduction = argv.mode === 'production';
    
    return {
        entry: './src/index.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProduction ? '[name].[contenthash].js' : '[name].js',
            clean: true,
        },
        module: {
            rules: [
                {
                    test: /\.js$/,
                    exclude: /node_modules/,
                    use: {
                        loader: 'babel-loader',
                        options: {
                            presets: ['@babel/preset-env'],
                            plugins: ['@babel/plugin-proposal-class-properties']
                        }
                    }
                },
                {
                    test: /\.css$/,
                    use: [
                        isProduction ? MiniCssExtractPlugin.loader : 'style-loader',
                        'css-loader',
                        'postcss-loader'
                    ]
                },
                {
                    test: /\.(png|svg|jpg|jpeg|gif)$/i,
                    type: 'asset/resource',
                }
            ]
        },
        plugins: [
            new HtmlWebpackPlugin({
                template: './public/index.html',
                minify: isProduction
            }),
            ...(isProduction ? [
                new MiniCssExtractPlugin({
                    filename: '[name].[contenthash].css'
                })
            ] : [])
        ],
        optimization: {
            minimize: isProduction,
            minimizer: [new TerserPlugin()],
            splitChunks: {
                chunks: 'all',
                cacheGroups: {
                    vendor: {
                        test: /[\\/]node_modules[\\/]/,
                        name: 'vendors',
                        chunks: 'all',
                    }
                }
            }
        },
        devServer: {
            contentBase: './dist',
            port: 3000,
            hot: true,
            open: true
        }
    };
};
```

### Deployment Script

```javascript
// scripts/deploy-dapp.js
const { ZhtpClient, DAppBuilder } = require('@zhtp/sdk');
const fs = require('fs');
const path = require('path');

async function deployDApp() {
    console.log('Starting DApp deployment...');
    
    try {
        // Initialize ZHTP client
        const client = new ZhtpClient({
            network: process.env.ZHTP_NETWORK || 'testnet',
            privateKey: process.env.PRIVATE_KEY
        });
        
        await client.connect();
        console.log('Connected to ZHTP network');
        
        // Create DApp package
        const builder = new DAppBuilder(
            'My Awesome DApp',
            '1.0.0',
            'A decentralized application on ZHTP'
        );
        
        // Add all files from dist directory
        const distPath = path.join(__dirname, '../dist');
        addDirectoryToBuilder(builder, distPath, '');
        
        // Build package
        const package = builder.build();
        console.log(`Package size: ${package.length} bytes`);
        
        // Deploy to ZHTP
        const deployResult = await client.deployDApp(
            'My Awesome DApp',
            package,
            {
                version: '1.0.0',
                description: 'A decentralized application on ZHTP',
                author: process.env.AUTHOR_EMAIL || 'developer@example.com',
                license: 'MIT',
                tags: ['dapp', 'zhtp', 'decentralized']
            }
        );
        
        console.log('DApp deployed successfully!');
        console.log(`Address: ${deployResult.address}`);
        console.log(`Transaction: ${deployResult.transactionHash}`);
        console.log(`Gas used: ${deployResult.gasUsed}`);
        console.log(`Cost: ${deployResult.deploymentCost} ZHTP`);
        
        // Save deployment info
        const deploymentInfo = {
            address: deployResult.address,
            transactionHash: deployResult.transactionHash,
            network: client.network,
            timestamp: new Date().toISOString(),
            version: '1.0.0'
        };
        
        fs.writeFileSync(
            'deployment.json',
            JSON.stringify(deploymentInfo, null, 2)
        );
        
        console.log('Deployment info saved to deployment.json');
        
        // Optionally register DNS
        if (process.env.DOMAIN_NAME) {
            await registerDomain(client, process.env.DOMAIN_NAME, deployResult.address);
        }
        
    } catch (error) {
        console.error('Deployment failed:', error);
        process.exit(1);
    }
}

function addDirectoryToBuilder(builder, dirPath, prefix) {
    const files = fs.readdirSync(dirPath);
    
    for (const file of files) {
        const filePath = path.join(dirPath, file);
        const relativePath = path.join(prefix, file);
        
        if (fs.statSync(filePath).isDirectory()) {
            addDirectoryToBuilder(builder, filePath, relativePath);
        } else {
            const content = fs.readFileSync(filePath);
            builder.addFile(relativePath, content);
        }
    }
}

async function registerDomain(client, domain, address) {
    try {
        console.log(`Registering domain: ${domain}`);
        
        const dns = new BlockchainDNS(client);
        const txHash = await dns.register(domain, {
            address: address,
            contentHash: 'QmXoYCz...', // Would be actual content hash
            ttl: 3600
        });
        
        console.log(`Domain registered: ${txHash}`);
    } catch (error) {
        console.error('Failed to register domain:', error);
    }
}

// Run deployment
deployDApp();
```

---

## 📚 Next Steps

- **[Zero-Knowledge Guide](zero-knowledge.md)** - Advanced ZK integration
- **[Security Best Practices](security-audit.md)** - Secure DApp development
- **[Performance Optimization](performance.md)** - Scaling your DApp
- **[DApp Templates](../templates/)** - Ready-to-use DApp templates
- **[API Documentation](../api/)** - Complete SDK references

For community support and advanced tutorials, visit our **[Developer Portal](../README.md)**.

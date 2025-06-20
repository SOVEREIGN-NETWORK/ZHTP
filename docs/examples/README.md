# ZHTP Examples & Code Samples

**Production-ready code examples for rapid development**

## Featured Examples

### Quick Start Examples
- [Hello World DApp](./hello-world/) - Minimal ZHTP application
- [Wallet Connection](./wallet-connect/) - User authentication
- [Smart Contract Interaction](./contract-basics/) - Deploy and call contracts

### 🏗️ Complete Applications
- [🗳️ Voting DAO](./voting-dao/) - Decentralized governance
- [🛒 NFT Marketplace](./nft-marketplace/) - Buy/sell digital assets  
- [💰 DeFi Exchange](./defi-exchange/) - Token swapping
- [🎮 Gaming Platform](./gaming/) - Blockchain games
- [📱 Social Network](./social-network/) - Decentralized Twitter
- [🏥 Healthcare Records](./healthcare/) - Private medical data

### Integration Examples
- [React Integration](./react/) - Frontend frameworks
- [Python Backend](./python-backend/) - API services
- [Mobile Apps](./mobile/) - React Native & Flutter
- [WordPress Plugin](./wordpress/) - CMS integration

---

## Hello World DApp

The simplest possible ZHTP application.

### JavaScript/TypeScript
```typescript
// src/App.tsx
import React from 'react';
import { ZHTPProvider, useZHTP, ZHTPButton } from '@zhtp/react-components';

function HelloWorld() {
    const { account, connect, disconnect } = useZHTP();
    
    return (
        <div style={{ padding: '2rem', textAlign: 'center' }}>
            <h1>Hello ZHTP World!</h1>
            
            {!account ? (
                <ZHTPButton onClick={connect}>
                    Connect Wallet
                </ZHTPButton>
            ) : (
                <div>
                    <p>✅ Connected: {account}</p>
                    <ZHTPButton onClick={disconnect}>
                        Disconnect
                    </ZHTPButton>
                </div>
            )}
        </div>
    );
}

export default function App() {
    return (
        <ZHTPProvider network="mainnet">
            <HelloWorld />
        </ZHTPProvider>
    );
}
```

### Python Flask
```python
# app.py
from flask import Flask, render_template, request, jsonify
from zhtp_sdk import ZHTPClient, Wallet

app = Flask(__name__)
zhtp = ZHTPClient(network="mainnet")

@app.route('/')
def hello_world():
    return render_template('index.html')

@app.route('/api/status')
def api_status():
    network = zhtp.get_network()
    latest_block = zhtp.get_latest_block()
    
    return jsonify({
        'message': 'Hello ZHTP World!',
        'network': network,
        'latest_block': latest_block,
        'connected': True
    })

if __name__ == '__main__':
    app.run(debug=True)
```

### Rust
```rust
// src/main.rs
use zhtp_sdk::{ZHTPClient, Network};
use warp::Filter;

#[tokio::main]
async fn main() {
    let client = ZHTPClient::new(Network::Mainnet).await
        .expect("Failed to connect to ZHTP network");
    
    let hello = warp::path("hello")
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "message": "Hello ZHTP World!",
                "network": "mainnet",
                "status": "connected"
            }))
        });
    
    println!("Hello ZHTP server running on http://localhost:3030");
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

---

## 🔌 Wallet Connection Examples

### React Hook
```typescript
// hooks/useWallet.ts
import { useState, useEffect } from 'react';
import { ZHTPWallet, WalletProvider } from '@zhtp/sdk';

export function useWallet() {
    const [wallet, setWallet] = useState<ZHTPWallet | null>(null);
    const [account, setAccount] = useState<string | null>(null);
    const [balance, setBalance] = useState<string>('0');
    const [isConnecting, setIsConnecting] = useState(false);

    const connect = async (provider: WalletProvider = 'metamask') => {
        setIsConnecting(true);
        try {
            const wallet = new ZHTPWallet(provider);
            await wallet.connect();
            
            const account = await wallet.getAccount();
            const balance = await wallet.getBalance();
            
            setWallet(wallet);
            setAccount(account);
            setBalance(balance);
            
            // Listen for account changes
            wallet.on('accountsChanged', (accounts) => {
                setAccount(accounts[0] || null);
            });
            
            // Listen for network changes
            wallet.on('chainChanged', () => {
                window.location.reload();
            });
            
        } catch (error) {
            console.error('Failed to connect wallet:', error);
        } finally {
            setIsConnecting(false);
        }
    };

    const disconnect = () => {
        if (wallet) {
            wallet.disconnect();
        }
        setWallet(null);
        setAccount(null);
        setBalance('0');
    };

    return {
        wallet,
        account,
        balance,
        isConnecting,
        connect,
        disconnect,
        isConnected: !!account
    };
}

// Component usage
function WalletConnection() {
    const { account, balance, isConnecting, connect, disconnect, isConnected } = useWallet();

    return (
        <div className="wallet-connection">
            {!isConnected ? (
                <div>
                    <button 
                        onClick={() => connect('metamask')}
                        disabled={isConnecting}
                    >
                        {isConnecting ? 'Connecting...' : 'Connect MetaMask'}
                    </button>
                    <button 
                        onClick={() => connect('walletconnect')}
                        disabled={isConnecting}
                    >
                        {isConnecting ? 'Connecting...' : 'Connect WalletConnect'}
                    </button>
                </div>
            ) : (
                <div>
                    <p>Account: {account}</p>
                    <p>Balance: {balance} ZHTP</p>
                    <button onClick={disconnect}>Disconnect</button>
                </div>
            )}
        </div>
    );
}
```

### Python Backend
```python
# wallet_service.py
from zhtp_sdk import Wallet, WalletProvider
import asyncio

class WalletService:
    def __init__(self):
        self.connected_wallets = {}
    
    async def connect_wallet(self, user_id: str, provider: str = 'metamask'):
        """Connect a wallet for a user"""
        try:
            wallet = Wallet(provider=WalletProvider(provider))
            await wallet.connect()
            
            account = await wallet.get_account()
            balance = await wallet.get_balance()
            
            self.connected_wallets[user_id] = {
                'wallet': wallet,
                'account': account,
                'balance': balance,
                'provider': provider
            }
            
            return {
                'success': True,
                'account': account,
                'balance': str(balance)
            }
            
        except Exception as e:
            return {
                'success': False,
                'error': str(e)
            }
    
    def disconnect_wallet(self, user_id: str):
        """Disconnect a wallet for a user"""
        if user_id in self.connected_wallets:
            wallet_info = self.connected_wallets[user_id]
            wallet_info['wallet'].disconnect()
            del self.connected_wallets[user_id]
            return {'success': True}
        
        return {'success': False, 'error': 'Wallet not connected'}
    
    def get_wallet_info(self, user_id: str):
        """Get wallet information for a user"""
        if user_id in self.connected_wallets:
            wallet_info = self.connected_wallets[user_id]
            return {
                'connected': True,
                'account': wallet_info['account'],
                'balance': wallet_info['balance'],
                'provider': wallet_info['provider']
            }
        
        return {'connected': False}

# Flask integration
from flask import Flask, request, jsonify, session

app = Flask(__name__)
app.secret_key = 'your-secret-key'
wallet_service = WalletService()

@app.route('/api/wallet/connect', methods=['POST'])
async def connect_wallet():
    user_id = session.get('user_id', 'anonymous')
    provider = request.json.get('provider', 'metamask')
    
    result = await wallet_service.connect_wallet(user_id, provider)
    return jsonify(result)

@app.route('/api/wallet/disconnect', methods=['POST'])
def disconnect_wallet():
    user_id = session.get('user_id', 'anonymous')
    result = wallet_service.disconnect_wallet(user_id)
    return jsonify(result)

@app.route('/api/wallet/info')
def wallet_info():
    user_id = session.get('user_id', 'anonymous')
    result = wallet_service.get_wallet_info(user_id)
    return jsonify(result)
```

---

## 📜 Smart Contract Examples

### Simple Storage Contract
```solidity
// contracts/SimpleStorage.sol
pragma solidity ^0.8.0;

import "@zhtp/contracts/ZKStorage.sol";

contract SimpleStorage is ZKStorage {
    mapping(address => string) private userMessages;
    mapping(address => bool) public hasMessage;
    
    event MessageStored(address indexed user, string message);
    event MessageUpdated(address indexed user, string oldMessage, string newMessage);
    
    function storeMessage(string memory _message) external {
        require(bytes(_message).length > 0, "Message cannot be empty");
        require(bytes(_message).length <= 280, "Message too long");
        
        string memory oldMessage = userMessages[msg.sender];
        userMessages[msg.sender] = _message;
        hasMessage[msg.sender] = true;
        
        if (bytes(oldMessage).length == 0) {
            emit MessageStored(msg.sender, _message);
        } else {
            emit MessageUpdated(msg.sender, oldMessage, _message);
        }
    }
    
    function getMessage(address _user) external view returns (string memory) {
        require(hasMessage[_user], "User has no message");
        return userMessages[_user];
    }
    
    function getMyMessage() external view returns (string memory) {
        return getMessage(msg.sender);
    }
    
    function deleteMessage() external {
        require(hasMessage[msg.sender], "No message to delete");
        
        delete userMessages[msg.sender];
        hasMessage[msg.sender] = false;
    }
}
```

### Interacting with Contract (JavaScript)
```typescript
// contracts/SimpleStorageClient.ts
import { Contract, ZHTPProvider } from '@zhtp/sdk';
import SimpleStorageABI from './SimpleStorage.json';

export class SimpleStorageClient {
    private contract: Contract;
    
    constructor(contractAddress: string, provider: ZHTPProvider) {
        this.contract = new Contract(contractAddress, SimpleStorageABI, provider);
    }
    
    async storeMessage(message: string) {
        const tx = await this.contract.storeMessage(message);
        return await tx.wait();
    }
    
    async getMessage(userAddress: string): Promise<string> {
        return await this.contract.getMessage(userAddress);
    }
    
    async getMyMessage(): Promise<string> {
        return await this.contract.getMyMessage();
    }
    
    async deleteMessage() {
        const tx = await this.contract.deleteMessage();
        return await tx.wait();
    }
    
    async hasMessage(userAddress: string): Promise<boolean> {
        return await this.contract.hasMessage(userAddress);
    }
    
    // Event listeners
    onMessageStored(callback: (user: string, message: string) => void) {
        this.contract.on('MessageStored', callback);
    }
    
    onMessageUpdated(callback: (user: string, oldMessage: string, newMessage: string) => void) {
        this.contract.on('MessageUpdated', callback);
    }
}

// React component usage
import React, { useState, useEffect } from 'react';
import { useZHTP } from '@zhtp/react-components';

function MessageStorage() {
    const { provider, account } = useZHTP();
    const [storageClient, setStorageClient] = useState<SimpleStorageClient | null>(null);
    const [message, setMessage] = useState('');
    const [storedMessage, setStoredMessage] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        if (provider) {
            const client = new SimpleStorageClient(
                '0x1234567890123456789012345678901234567890', // Contract address
                provider
            );
            setStorageClient(client);
            
            // Listen for events
            client.onMessageStored((user, message) => {
                if (user.toLowerCase() === account?.toLowerCase()) {
                    setStoredMessage(message);
                }
            });
        }
    }, [provider, account]);

    const handleStoreMessage = async () => {
        if (!storageClient || !message.trim()) return;
        
        setIsLoading(true);
        try {
            await storageClient.storeMessage(message.trim());
            setMessage('');
            await loadStoredMessage();
        } catch (error) {
            console.error('Failed to store message:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const loadStoredMessage = async () => {
        if (!storageClient || !account) return;
        
        try {
            const hasMsg = await storageClient.hasMessage(account);
            if (hasMsg) {
                const msg = await storageClient.getMessage(account);
                setStoredMessage(msg);
            }
        } catch (error) {
            console.error('Failed to load message:', error);
        }
    };

    useEffect(() => {
        loadStoredMessage();
    }, [storageClient, account]);

    return (
        <div className="message-storage">
            <h3>Message Storage</h3>
            
            <div className="input-section">
                <textarea
                    value={message}
                    onChange={(e) => setMessage(e.target.value)}
                    placeholder="Enter your message (max 280 characters)"
                    maxLength={280}
                    rows={3}
                />
                <button 
                    onClick={handleStoreMessage}
                    disabled={isLoading || !message.trim()}
                >
                    {isLoading ? 'Storing...' : 'Store Message'}
                </button>
            </div>
            
            {storedMessage && (
                <div className="stored-message">
                    <h4>Your Stored Message:</h4>
                    <p>{storedMessage}</p>
                </div>
            )}
        </div>
    );
}
```

---

## 🎮 Gaming Example - Simple NFT Game

```typescript
// contracts/SimpleGame.sol
pragma solidity ^0.8.0;

import "@zhtp/contracts/ZKNFTGame.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract SimpleGame is ERC721, ZKNFTGame {
    struct Character {
        string name;
        uint256 level;
        uint256 experience;
        uint256 strength;
        uint256 defense;
        uint256 lastBattle;
    }
    
    mapping(uint256 => Character) public characters;
    mapping(address => uint256[]) public playerCharacters;
    uint256 public nextTokenId = 1;
    
    event CharacterCreated(uint256 indexed tokenId, address indexed owner, string name);
    event CharacterLevelUp(uint256 indexed tokenId, uint256 newLevel);
    event BattleResult(uint256 indexed attacker, uint256 indexed defender, bool attackerWon);
    
    constructor() ERC721("ZHTP Game Characters", "ZGC") {}
    
    function createCharacter(string memory _name) external returns (uint256) {
        uint256 tokenId = nextTokenId++;
        
        characters[tokenId] = Character({
            name: _name,
            level: 1,
            experience: 0,
            strength: 10,
            defense: 10,
            lastBattle: block.timestamp
        });
        
        _mint(msg.sender, tokenId);
        playerCharacters[msg.sender].push(tokenId);
        
        emit CharacterCreated(tokenId, msg.sender, _name);
        return tokenId;
    }
    
    function battle(uint256 _attackerId, uint256 _defenderId) external {
        require(ownerOf(_attackerId) == msg.sender, "Not your character");
        require(_attackerId != _defenderId, "Cannot battle yourself");
        require(
            block.timestamp >= characters[_attackerId].lastBattle + 1 hours,
            "Character must rest between battles"
        );
        
        Character storage attacker = characters[_attackerId];
        Character storage defender = characters[_defenderId];
        
        // Simple battle mechanics
        uint256 attackPower = attacker.strength + (attacker.level * 2);
        uint256 defensePower = defender.defense + (defender.level * 2);
        
        // Add randomness using ZK proofs
        uint256 randomness = _zkRandomness(_attackerId, _defenderId);
        bool attackerWins = (attackPower + randomness % 10) > (defensePower + randomness % 5);
        
        if (attackerWins) {
            attacker.experience += 10;
            if (attacker.experience >= attacker.level * 100) {
                _levelUp(_attackerId);
            }
        }
        
        attacker.lastBattle = block.timestamp;
        emit BattleResult(_attackerId, _defenderId, attackerWins);
    }
    
    function _levelUp(uint256 _tokenId) internal {
        Character storage character = characters[_tokenId];
        character.level++;
        character.experience = 0;
        character.strength += 5;
        character.defense += 5;
        
        emit CharacterLevelUp(_tokenId, character.level);
    }
    
    function getPlayerCharacters(address _player) external view returns (uint256[] memory) {
        return playerCharacters[_player];
    }
}
```

### Game Client (React)
```typescript
// components/GameClient.tsx
import React, { useState, useEffect } from 'react';
import { useZHTP } from '@zhtp/react-components';
import { GameContract } from '../contracts/GameContract';

interface Character {
    id: number;
    name: string;
    level: number;
    experience: number;
    strength: number;
    defense: number;
    lastBattle: number;
}

function GameClient() {
    const { provider, account } = useZHTP();
    const [gameContract, setGameContract] = useState<GameContract | null>(null);
    const [characters, setCharacters] = useState<Character[]>([]);
    const [newCharacterName, setNewCharacterName] = useState('');
    const [selectedCharacter, setSelectedCharacter] = useState<number | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        if (provider) {
            const contract = new GameContract(
                '0x1234567890123456789012345678901234567890', // Game contract address
                provider
            );
            setGameContract(contract);
            
            // Listen for events
            contract.onCharacterCreated((tokenId, owner, name) => {
                if (owner.toLowerCase() === account?.toLowerCase()) {
                    loadCharacters();
                }
            });
            
            contract.onCharacterLevelUp((tokenId, newLevel) => {
                loadCharacters();
            });
        }
    }, [provider, account]);

    const loadCharacters = async () => {
        if (!gameContract || !account) return;
        
        try {
            const characterIds = await gameContract.getPlayerCharacters(account);
            const charactersData = await Promise.all(
                characterIds.map(async (id) => {
                    const character = await gameContract.getCharacter(id);
                    return {
                        id: parseInt(id),
                        name: character.name,
                        level: parseInt(character.level),
                        experience: parseInt(character.experience),
                        strength: parseInt(character.strength),
                        defense: parseInt(character.defense),
                        lastBattle: parseInt(character.lastBattle)
                    };
                })
            );
            setCharacters(charactersData);
        } catch (error) {
            console.error('Failed to load characters:', error);
        }
    };

    const createCharacter = async () => {
        if (!gameContract || !newCharacterName.trim()) return;
        
        setIsLoading(true);
        try {
            await gameContract.createCharacter(newCharacterName.trim());
            setNewCharacterName('');
        } catch (error) {
            console.error('Failed to create character:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const battle = async (defenderId: number) => {
        if (!gameContract || selectedCharacter === null) return;
        
        setIsLoading(true);
        try {
            await gameContract.battle(selectedCharacter, defenderId);
            await loadCharacters();
        } catch (error) {
            console.error('Battle failed:', error);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        loadCharacters();
    }, [gameContract, account]);

    return (
        <div className="game-client">
            <h2>⚔️ ZHTP Battle Game</h2>
            
            <div className="create-character">
                <h3>Create New Character</h3>
                <input
                    type="text"
                    value={newCharacterName}
                    onChange={(e) => setNewCharacterName(e.target.value)}
                    placeholder="Character name"
                    maxLength={20}
                />
                <button 
                    onClick={createCharacter}
                    disabled={isLoading || !newCharacterName.trim()}
                >
                    {isLoading ? 'Creating...' : 'Create Character'}
                </button>
            </div>
            
            <div className="characters">
                <h3>Your Characters</h3>
                {characters.length === 0 ? (
                    <p>No characters yet. Create your first character!</p>
                ) : (
                    <div className="character-grid">
                        {characters.map((character) => (
                            <div 
                                key={character.id}
                                className={`character-card ${selectedCharacter === character.id ? 'selected' : ''}`}
                                onClick={() => setSelectedCharacter(character.id)}
                            >
                                <h4>{character.name}</h4>
                                <p>Level: {character.level}</p>
                                <p>XP: {character.experience}/{character.level * 100}</p>
                                <p>Strength: {character.strength}</p>
                                <p>Defense: {character.defense}</p>
                                <div className="character-actions">
                                    {Date.now() / 1000 >= character.lastBattle + 3600 ? (
                                        <span className="ready">⚔️ Ready to battle</span>
                                    ) : (
                                        <span className="resting">😴 Resting</span>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>
            
            {selectedCharacter !== null && (
                <div className="battle-section">
                    <h3>Battle Arena</h3>
                    <p>Select an opponent to battle with character #{selectedCharacter}</p>
                    <div className="opponents">
                        {characters
                            .filter(c => c.id !== selectedCharacter)
                            .map(character => (
                                <button
                                    key={character.id}
                                    onClick={() => battle(character.id)}
                                    disabled={isLoading}
                                    className="opponent-button"
                                >
                                    Battle {character.name} (Lvl {character.level})
                                </button>
                            ))
                        }
                    </div>
                </div>
            )}
        </div>
    );
}

export default GameClient;
```

---

## 📱 Mobile App Example (React Native)

```typescript
// App.tsx
import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { ZHTPMobileProvider, useZHTPMobile } from '@zhtp/react-native';

function WalletScreen() {
    const { account, balance, connect, disconnect, isConnected } = useZHTPMobile();
    
    return (
        <View style={styles.container}>
            <Text style={styles.title}>ZHTP Mobile Wallet</Text>
            
            {!isConnected ? (
                <TouchableOpacity style={styles.button} onPress={connect}>
                    <Text style={styles.buttonText}>Connect Wallet</Text>
                </TouchableOpacity>
            ) : (
                <View style={styles.walletInfo}>
                    <Text style={styles.label}>Account:</Text>
                    <Text style={styles.account}>{account}</Text>
                    
                    <Text style={styles.label}>Balance:</Text>
                    <Text style={styles.balance}>{balance} ZHTP</Text>
                    
                    <TouchableOpacity style={styles.button} onPress={disconnect}>
                        <Text style={styles.buttonText}>Disconnect</Text>
                    </TouchableOpacity>
                </View>
            )}
        </View>
    );
}

export default function App() {
    return (
        <ZHTPMobileProvider network="mainnet">
            <WalletScreen />
        </ZHTPMobileProvider>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: '#f0f0f0',
        padding: 20,
    },
    title: {
        fontSize: 24,
        fontWeight: 'bold',
        marginBottom: 30,
        color: '#333',
    },
    button: {
        backgroundColor: '#007AFF',
        paddingHorizontal: 30,
        paddingVertical: 15,
        borderRadius: 25,
        marginTop: 20,
    },
    buttonText: {
        color: 'white',
        fontSize: 16,
        fontWeight: '600',
    },
    walletInfo: {
        alignItems: 'center',
        width: '100%',
    },
    label: {
        fontSize: 16,
        fontWeight: '600',
        color: '#666',
        marginTop: 20,
    },
    account: {
        fontSize: 14,
        color: '#333',
        marginTop: 5,
        textAlign: 'center',
    },
    balance: {
        fontSize: 18,
        fontWeight: 'bold',
        color: '#007AFF',
        marginTop: 5,
    },
});
```

---

## More Complete Examples

### 🛒 [NFT Marketplace](./nft-marketplace/)
Complete marketplace with minting, buying, selling, and bidding.

### 💰 [DeFi Exchange](./defi-exchange/)
Token swapping, liquidity pools, and yield farming.

### 🏥 [Healthcare Records](./healthcare/)
Private medical records with zero-knowledge proofs.

### 📱 [Social Network](./social-network/)
Decentralized Twitter with posts, follows, and messaging.

### 🎮 [Gaming Platform](./gaming/)
Multi-game platform with NFT characters and tournaments.

---

## Running Examples

### Clone and Setup
```bash
# Clone examples repository
git clone https://github.com/zhtp/examples.git
cd examples

# Install dependencies
npm install

# Set up environment
cp .env.example .env
# Edit .env with your configuration
```

### Run Individual Examples
```bash
# Hello World
npm run example:hello-world

# Voting DAO
npm run example:voting-dao

# NFT Marketplace
npm run example:nft-marketplace

# Gaming Platform
npm run example:gaming
```

### Deploy to ZHTP Network
```bash
# Deploy contracts
npm run deploy:contracts

# Deploy frontend
npm run deploy:frontend

# Register domain
npm run deploy:domain
```

---

## Example Templates

Use these as starting points for your own projects:

```bash
# Create from template
npx create-zhtp-app my-project --template [template-name]

# Available templates:
# - hello-world
# - voting-dao
# - nft-marketplace
# - defi-exchange
# - gaming-platform
# - social-network
# - healthcare-records
```

---

## Learn More

- [📖 API Documentation](../api/)
- [UI Components](../ui/)
- [🏗️ Advanced Guides](../guides/)
- [Best Practices](../best-practices.md)

**Start building with these examples and create the next generation of decentralized applications!**

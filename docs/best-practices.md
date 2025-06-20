# 🛡️ ZHTP Best Practices - Decentralized Internet Development

**Production-ready guidelines for building secure, scalable, and maintainable decentralized applications that completely replace traditional internet infrastructure**

> **Key Principle**: ZHTP applications use zero traditional internet infrastructure - no DNS servers, no SSL certificates, no HTTP/HTTPS, no cloud hosting. Everything runs on the decentralized ZHTP network.

## Architecture Best Practices

### Project Structure
```
my-zhtp-dapp/
├── contracts/               # Smart contracts
│   ├── src/
│   │   ├── voting.rs
│   │   └── governance.rs
│   ├── test/
│   │   ├── voting.test.js
│   │   └── governance.test.js
│   └── deploy/
│       └── deploy.js
├── frontend/               # React/Vue frontend
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── utils/
│   │   └── contracts/
│   ├── public/
│   └── package.json
├── backend/                # Optional API server
│   ├── src/
│   │   ├── routes/
│   │   ├── services/
│   │   └── middleware/
│   └── package.json
├── docs/                   # Documentation
│   ├── README.md
│   ├── API.md
│   └── DEPLOYMENT.md
└── package.json
```

### Configuration Management
```typescript
// config/index.ts - Native ZHTP Configuration (No HTTP/RPC)
interface ZhtpConfig {
  network: 'mainnet' | 'testnet' | 'local';
  nodeEndpoints: string[];  // ZHTP native protocol endpoints
  contractAddresses: Record<string, string>;
  zk_privacy_level: 'standard' | 'high' | 'maximum';
  storage_cache_size: number;
}

const configs: Record<string, ZhtpConfig> = {
  production: {
    network: 'mainnet',
    nodeEndpoints: [
      'zhtp://node1.zhtp.network:8443',
      'zhtp://node2.zhtp.network:8443',
      'zhtp://node3.zhtp.network:8443'
    ],
    contractAddresses: {
      voting: 'zhtp-contract://voting.zhtp/0x...',
      governance: 'zhtp-contract://governance.zhtp/0x...'
    },
    zk_privacy_level: 'high',
    storage_cache_size: 100_000_000
  },
  development: {
    network: 'testnet',
    nodeEndpoints: [
      'zhtp://testnet-node1.zhtp.network:8443',
      'zhtp://testnet-node2.zhtp.network:8443'
    ],
    contractAddresses: {
      voting: 'zhtp-contract://voting-test.zhtp/0x...',
      governance: 'zhtp-contract://governance-test.zhtp/0x...'
    },
    zk_privacy_level: 'standard',
    storage_cache_size: 50_000_000
  }
};

export const zhtpConfig = configs[process.env.NODE_ENV || 'development'];

// Usage Example - Connecting to decentralized network
import { ZhtpClient } from '@zhtp/sdk';

const client = new ZhtpClient({
  ...zhtpConfig,
  privacy_level: 'maximum',  // Built-in anonymity
  quantum_security: true     // Post-quantum cryptography
});

await client.connect();  // No HTTP, no traditional infrastructure
```

## 🌐 Native ZHTP Development Patterns

### Decentralized Domain Resolution (Replaces DNS)
```typescript
// Traditional DNS resolution - NEVER DO THIS IN ZHTP
// const response = await fetch('https://api.example.com/data');  ❌

// Native ZHTP blockchain DNS resolution
const client = new ZhtpClient(zhtpConfig);
const domainInfo = await client.blockchain_dns.resolve('api.zhtp');
const content = await client.fetch_content({
  domain: 'api.zhtp',
  path: '/data',
  anonymous: true  // Built-in privacy
});
```

### ZK Certificate Authority (Replaces SSL/TLS)
```typescript
// Traditional SSL/TLS certificates - NOT NEEDED IN ZHTP
// const httpsAgent = new https.Agent({ ... });  ❌

// Native ZHTP zero-knowledge certificates
const zkCert = await client.zk_certificate_authority.issue_certificate({
  domain: 'my-dapp.zhtp',
  security_level: 'quantum_resistant',
  privacy_level: 'maximum'
});

// Automatic verification during content fetching
const secureContent = await client.fetch_content({
  domain: 'my-dapp.zhtp',
  verify_zk_certificate: true  // Quantum-resistant verification
});
```

---

## Security Best Practices

### Smart Contract Security
```rust
// Secure contract patterns
use zhtp_sdk::prelude::*;
use zhtp_security::ZKSecurity;

#[contract]
pub struct SecureVoting {
    // Use specific Solidity version
    // Import security libraries
    // Implement access controls
    
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    bytes32 public constant VOTER_ROLE = keccak256("VOTER_ROLE");
    
    #[storage]
    struct Storage {
        proposals: Map<u64, Proposal>,
        proposal_count: u64,
        admin_role: RoleId,
        voter_role: RoleId,
    }

    #[derive(Storage)]
    struct Proposal {
        title: String,
        description: String,
        end_time: u64,
        yes_votes: u64,
        no_votes: u64,
        executed: bool,
        has_voted: Map<Address, bool>,
    }

    // Events for transparency
    #[event]
    struct ProposalCreated {
        #[indexed]
        id: u64,
        title: String,
        #[indexed]
        creator: Address,
    }

    #[event]
    struct VoteCast {
        #[indexed]
        proposal_id: u64,
        #[indexed]
        voter: Address,
        support: bool,
    }

    impl SecureVoting {
        #[init]
        pub fn new(deployer: Address) -> Self {
            let mut storage = Storage::default();
            storage.admin_role = RoleId::new("ADMIN");
            storage.voter_role = RoleId::new("VOTER");
            
            // Grant roles to deployer
            storage.grant_role(storage.admin_role, deployer);
            storage.grant_role(storage.voter_role, deployer);
            
            Self { storage }
        }
    
    // Input validation
    function createProposal(
        string calldata title,
        string calldata description,
        uint256 votingPeriod
    ) external onlyRole(ADMIN_ROLE) whenNotPaused returns (uint256) {
        require(bytes(title).length > 0 && bytes(title).length <= 100, "Invalid title length");
        require(bytes(description).length > 0 && bytes(description).length <= 1000, "Invalid description length");
        require(votingPeriod >= 1 days && votingPeriod <= 30 days, "Invalid voting period");
        
        uint256 proposalId = proposalCount++;
        Proposal storage proposal = proposals[proposalId];
        
        proposal.title = title;
        proposal.description = description;
        proposal.endTime = block.timestamp + votingPeriod;
        
        emit ProposalCreated(proposalId, title, msg.sender);
        return proposalId;
    }
    
    // Prevent reentrancy
    function vote(uint256 proposalId, bool support) 
        external 
        onlyRole(VOTER_ROLE) 
        nonReentrant 
        whenNotPaused 
    {
        require(proposalId < proposalCount, "Proposal does not exist");
        
        Proposal storage proposal = proposals[proposalId];
        require(block.timestamp < proposal.endTime, "Voting period ended");
        require(!proposal.hasVoted[msg.sender], "Already voted");
        
        // Update state before external calls
        proposal.hasVoted[msg.sender] = true;
        
        if (support) {
            proposal.yesVotes++;
        } else {
            proposal.noVotes++;
        }
        
        emit VoteCast(proposalId, msg.sender, support);
    }
    
    // Emergency pause mechanism
    function pause() external onlyRole(ADMIN_ROLE) {
        _pause();
    }
    
    function unpause() external onlyRole(ADMIN_ROLE) {
        _unpause();
    }
}
```

### Frontend Security
```typescript
// Security utilities
import DOMPurify from 'dompurify';
import { ethers } from 'ethers';

export class SecurityUtils {
    // Input sanitization
    static sanitizeHTML(input: string): string {
        return DOMPurify.sanitize(input, {
            ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a'],
            ALLOWED_ATTR: ['href', 'title']
        });
    }
    
    // Address validation
    static isValidAddress(address: string): boolean {
        try {
            return ethers.utils.isAddress(address);
        } catch {
            return false;
        }
    }
    
    // Amount validation
    static validateAmount(amount: string, decimals: number = 18): boolean {
        try {
            const parsed = ethers.utils.parseUnits(amount, decimals);
            return parsed.gt(0);
        } catch {
            return false;
        }
    }
    
    // Rate limiting
    private static requestCounts = new Map<string, number>();
    private static lastReset = Date.now();
    
    static checkRateLimit(identifier: string, limit: number = 10, windowMs: number = 60000): boolean {
        const now = Date.now();
        
        // Reset counts if window expired
        if (now - this.lastReset > windowMs) {
            this.requestCounts.clear();
            this.lastReset = now;
        }
        
        const count = this.requestCounts.get(identifier) || 0;
        
        if (count >= limit) {
            throw new Error('Rate limit exceeded. Please try again later.');
        }
        
        this.requestCounts.set(identifier, count + 1);
        return true;
    }
    
    // Secure transaction confirmation
    static async confirmTransaction(
        transaction: any,
        onConfirm: () => void
    ): Promise<boolean> {
        const confirmation = await this.showSecureModal({
            title: 'Confirm Transaction',
            content: `
                <div class="transaction-details">
                    <p><strong>To:</strong> ${transaction.to}</p>
                    <p><strong>Value:</strong> ${transaction.value} ZHTP</p>
                    <p><strong>Gas:</strong> ${transaction.gasLimit}</p>
                    <p><strong>Function:</strong> ${transaction.data}</p>
                </div>
            `,
            confirmText: 'Sign Transaction',
            cancelText: 'Cancel'
        });
        
        if (confirmation) {
            onConfirm();
        }
        
        return confirmation;
    }
}

// Secure component patterns
interface SecureInputProps {
    value: string;
    onChange: (value: string) => void;
    validate?: (value: string) => string | null;
    sanitize?: (value: string) => string;
}

export function SecureInput({ value, onChange, validate, sanitize }: SecureInputProps) {
    const [error, setError] = useState<string | null>(null);
    
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        let newValue = e.target.value;
        
        // Sanitize input
        if (sanitize) {
            newValue = sanitize(newValue);
        }
        
        // Validate input
        if (validate) {
            const validationError = validate(newValue);
            setError(validationError);
        }
        
        onChange(newValue);
    };
    
    return (
        <div>
            <input
                value={value}
                onChange={handleChange}
                className={error ? 'error' : ''}
            />
            {error && <span className="error-message">{error}</span>}
        </div>
    );
}
```

---

## Performance Best Practices

### Smart Contract Optimization
```rust
// Gas-optimized patterns
#[contract]
pub struct OptimizedContract {
    #[storage]
    struct PackedUser {
        balance: u128,    // 16 bytes
        last_active: u64, // 8 bytes
        nonce: u32,       // 4 bytes
        is_active: bool,  // 1 byte
        // Total: 29 bytes (optimized for storage)
    }

    #[storage]
    struct Storage {
        users: Map<Address, PackedUser>,
    }
    
    // Cache storage reads
    function updateUser(address userAddr, uint128 newBalance) external {
        PackedUser storage user = users[userAddr];
        
        // Multiple operations on same storage slot
        user.balance = newBalance;
        user.lastActive = uint64(block.timestamp);
        user.nonce++;
    }
    
    // Use unchecked for safe arithmetic
    function efficientSum(uint256[] calldata numbers) external pure returns (uint256) {
        uint256 sum = 0;
        
        for (uint256 i = 0; i < numbers.length;) {
            sum += numbers[i];
            unchecked { i++; }
        }
        
        return sum;
    }
    
    // Batch operations
    function batchTransfer(
        address[] calldata recipients,
        uint256[] calldata amounts
    ) external {
        require(recipients.length == amounts.length, "Length mismatch");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            _transfer(msg.sender, recipients[i], amounts[i]);
        }
    }
}
```

### Frontend Optimization
```typescript
// React performance patterns
import { memo, useMemo, useCallback, lazy, Suspense } from 'react';

// Lazy load heavy components
const HeavyChart = lazy(() => import('./HeavyChart'));

// Memoize expensive components
const OptimizedTransactionList = memo(function TransactionList({ 
    transactions, 
    onTransactionClick 
}: TransactionListProps) {
    // Memoize expensive calculations
    const processedTransactions = useMemo(() => {
        return transactions.map(tx => ({
            ...tx,
            formattedAmount: formatCurrency(tx.amount),
            relativeTime: formatRelativeTime(tx.timestamp)
        }));
    }, [transactions]);
    
    // Memoize event handlers
    const handleClick = useCallback((txId: string) => {
        onTransactionClick(txId);
    }, [onTransactionClick]);
    
    return (
        <div className="transaction-list">
            {processedTransactions.map(tx => (
                <TransactionItem 
                    key={tx.id}
                    transaction={tx}
                    onClick={handleClick}
                />
            ))}
        </div>
    );
});

// Virtual scrolling for large lists
import { FixedSizeList as List } from 'react-window';

function VirtualizedList({ items }: { items: any[] }) {
    const Row = ({ index, style }: { index: number; style: CSSProperties }) => (
        <div style={style}>
            <ListItem item={items[index]} />
        </div>
    );
    
    return (
        <List
            height={600}
            itemCount={items.length}
            itemSize={80}
            overscanCount={5}
        >
            {Row}
        </List>
    );
}

// Optimized ZHTP client
class OptimizedZHTPClient {
    private cache = new Map();
    private batchQueue: any[] = [];
    private batchTimer: NodeJS.Timeout | null = null;
    
    constructor(private client: ZHTPClient) {}
    
    // Request batching
    async batchedCall(method: string, params: any[]): Promise<any> {
        return new Promise((resolve, reject) => {
            this.batchQueue.push({ method, params, resolve, reject });
            
            if (!this.batchTimer) {
                this.batchTimer = setTimeout(() => {
                    this.processBatch();
                }, 10); // Batch requests for 10ms
            }
        });
    }
    
    private async processBatch() {
        const batch = this.batchQueue.splice(0);
        this.batchTimer = null;
        
        try {
            const results = await this.client.batch(
                batch.map(item => ({ method: item.method, params: item.params }))
            );
            
            batch.forEach((item, index) => {
                item.resolve(results[index]);
            });
        } catch (error) {
            batch.forEach(item => item.reject(error));
        }
    }
    
    // Intelligent caching
    async cachedCall(cacheKey: string, fetcher: () => Promise<any>, ttl = 30000): Promise<any> {
        const cached = this.cache.get(cacheKey);
        
        if (cached && Date.now() - cached.timestamp < ttl) {
            return cached.data;
        }
        
        const data = await fetcher();
        this.cache.set(cacheKey, { data, timestamp: Date.now() });
        
        return data;
    }
}
```

---

## Testing Best Practices

### Smart Contract Testing
```rust
// Comprehensive contract testing
#[cfg(test)]
mod tests {
    use super::*;
    use zhtp_test_utils::{setup_test_env, TestEnv};

    fn setup() -> TestEnv {
        let env = setup_test_env();
        let owner = env.accounts.get(0);
        let voter1 = env.accounts.get(1);
        let voter2 = env.accounts.get(2);
        let attacker = env.accounts.get(3);

        let contract = SecureVoting::new(owner.address);
        
        // Grant voter roles
        contract.grant_role(contract.storage.voter_role, voter1.address);
        contract.grant_role(contract.storage.voter_role, voter2.address);

        TestEnv { env, contract }
    }
        
        return { voting, owner, voter1, voter2, attacker };
    }
    
    describe('Proposal Creation', function () {
        it('Should create proposal with valid parameters', async function () {
            const { voting, owner } = await loadFixture(deployVotingFixture);
            
            const tx = await voting.createProposal(
                'Test Proposal',
                'This is a test proposal',
                7 * 24 * 60 * 60 // 7 days
            );
            
            await expect(tx)
                .to.emit(voting, 'ProposalCreated')
                .withArgs(0, 'Test Proposal', owner.address);
        });
        
        it('Should reject proposal with invalid title', async function () {
            const { voting } = await loadFixture(deployVotingFixture);
            
            await expect(
                voting.createProposal('', 'Description', 7 * 24 * 60 * 60)
            ).to.be.revertedWith('Invalid title length');
        });
        
        it('Should reject proposal from non-admin', async function () {
            const { voting, voter1 } = await loadFixture(deployVotingFixture);
            
            await expect(
                voting.connect(voter1).createProposal(
                    'Title', 'Description', 7 * 24 * 60 * 60
                )
            ).to.be.revertedWith('AccessControl: account');
        });
    });
    
    describe('Voting', function () {
        it('Should allow valid vote', async function () {
            const { voting, voter1 } = await loadFixture(deployVotingFixture);
            
            // Create proposal first
            await voting.createProposal('Title', 'Description', 7 * 24 * 60 * 60);
            
            const tx = await voting.connect(voter1).vote(0, true);
            
            await expect(tx)
                .to.emit(voting, 'VoteCast')
                .withArgs(0, voter1.address, true);
        });
        
        it('Should prevent double voting', async function () {
            const { voting, voter1 } = await loadFixture(deployVotingFixture);
            
            await voting.createProposal('Title', 'Description', 7 * 24 * 60 * 60);
            await voting.connect(voter1).vote(0, true);
            
            await expect(
                voting.connect(voter1).vote(0, false)
            ).to.be.revertedWith('Already voted');
        });
        
        it('Should prevent voting by non-voters', async function () {
            const { voting, attacker } = await loadFixture(deployVotingFixture);
            
            await voting.createProposal('Title', 'Description', 7 * 24 * 60 * 60);
            
            await expect(
                voting.connect(attacker).vote(0, true)
            ).to.be.revertedWith('AccessControl: account');
        });
    });
    
    describe('Security', function () {
        it('Should handle reentrancy attacks', async function () {
            // Test reentrancy protection
        });
        
        it('Should pause in emergency', async function () {
            const { voting, owner, voter1 } = await loadFixture(deployVotingFixture);
            
            await voting.createProposal('Title', 'Description', 7 * 24 * 60 * 60);
            await voting.pause();
            
            await expect(
                voting.connect(voter1).vote(0, true)
            ).to.be.revertedWith('Pausable: paused');
        });
    });
});
```

### Frontend Testing
```typescript
// React component testing
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { ZHTPProvider } from '@zhtp/react-components';
import VotingComponent from './VotingComponent';

// Mock ZHTP SDK
jest.mock('@zhtp/sdk', () => ({
    ZHTPClient: jest.fn().mockImplementation(() => ({
        connect: jest.fn(),
        getContract: jest.fn().mockReturnValue({
            vote: jest.fn(),
            getProposal: jest.fn()
        })
    }))
}));

describe('VotingComponent', () => {
    const renderWithProvider = (component: React.ReactElement) => {
        return render(
            <ZHTPProvider network="testnet">
                {component}
            </ZHTPProvider>
        );
    };
    
    it('should render voting interface', () => {
        renderWithProvider(<VotingComponent proposalId={1} />);
        
        expect(screen.getByText('Vote Yes')).toBeInTheDocument();
        expect(screen.getByText('Vote No')).toBeInTheDocument();
    });
    
    it('should handle vote submission', async () => {
        const mockVote = jest.fn().mockResolvedValue({ wait: () => Promise.resolve() });
        
        // Mock contract method
        jest.spyOn(require('@zhtp/sdk'), 'useContract').mockReturnValue({
            vote: mockVote
        });
        
        renderWithProvider(<VotingComponent proposalId={1} />);
        
        fireEvent.click(screen.getByText('Vote Yes'));
        
        await waitFor(() => {
            expect(mockVote).toHaveBeenCalledWith(1, true);
        });
    });
    
    it('should show loading state during vote', async () => {
        const mockVote = jest.fn().mockImplementation(() => 
            new Promise(resolve => setTimeout(resolve, 1000))
        );
        
        renderWithProvider(<VotingComponent proposalId={1} />);
        
        fireEvent.click(screen.getByText('Vote Yes'));
        
        expect(screen.getByText('Voting...')).toBeInTheDocument();
    });
    
    it('should handle vote errors gracefully', async () => {
        const mockVote = jest.fn().mockRejectedValue(new Error('Vote failed'));
        
        renderWithProvider(<VotingComponent proposalId={1} />);
        
        fireEvent.click(screen.getByText('Vote Yes'));
        
        await waitFor(() => {
            expect(screen.getByText('Vote failed')).toBeInTheDocument();
        });
    });
});

// Integration testing
describe('Voting Integration', () => {
    it('should complete full voting flow', async () => {
        // Test complete user journey
        const { voting, voter } = await setupTestEnvironment();
        
        // 1. Connect wallet
        await voting.connectWallet();
        expect(voting.isConnected()).toBe(true);
        
        // 2. Load proposals
        await voting.loadProposals();
        expect(voting.getProposals()).toHaveLength(1);
        
        // 3. Cast vote
        await voting.vote(0, true);
        
        // 4. Verify vote was recorded
        const proposal = await voting.getProposal(0);
        expect(proposal.yesVotes).toBe(1);
    });
});
```

---

## Monitoring & Analytics

### Application Monitoring
```typescript
// Comprehensive monitoring setup
import { ZHTPMonitor, MetricsCollector } from '@zhtp/monitoring';

class ApplicationMonitor {
    private monitor: ZHTPMonitor;
    private metrics: MetricsCollector;
    
    constructor() {
        this.monitor = new ZHTPMonitor({
            service: 'voting-dapp',
            version: process.env.APP_VERSION || '1.0.0',
            environment: process.env.NODE_ENV || 'development'
        });
        
        this.metrics = new MetricsCollector();
        this.setupMetrics();
    }
    
    private setupMetrics() {
        // Track user actions
        this.metrics.track('user_actions', {
            labels: ['action_type', 'user_id'],
            help: 'User interactions with the DApp'
        });
        
        // Track transaction metrics
        this.metrics.track('transactions', {
            labels: ['type', 'status', 'network'],
            help: 'Transaction statistics'
        });
        
        // Track performance metrics
        this.metrics.track('response_times', {
            labels: ['endpoint', 'method'],
            help: 'API response times'
        });
        
        // Track errors
        this.metrics.track('errors', {
            labels: ['error_type', 'component'],
            help: 'Application errors'
        });
    }
    
    // Custom event tracking
    trackUserAction(action: string, userId: string, metadata?: any) {
        this.metrics.increment('user_actions', { action_type: action, user_id: userId });
        
        this.monitor.log('info', `User action: ${action}`, {
            userId,
            timestamp: new Date().toISOString(),
            ...metadata
        });
    }
    
    // Transaction monitoring
    trackTransaction(txHash: string, type: string, status: 'pending' | 'success' | 'failed') {
        this.metrics.increment('transactions', { type, status, network: 'zhtp' });
        
        this.monitor.log('info', `Transaction ${status}`, {
            txHash,
            type,
            status,
            timestamp: new Date().toISOString()
        });
    }
    
    // Performance tracking
    trackPerformance(operation: string, duration: number) {
        this.metrics.observe('response_times', duration, { endpoint: operation, method: 'POST' });
        
        if (duration > 2000) { // Slow operation warning
            this.monitor.log('warn', `Slow operation detected: ${operation}`, {
                duration,
                threshold: 2000
            });
        }
    }
    
    // Error tracking
    trackError(error: Error, component: string, context?: any) {
        this.metrics.increment('errors', { error_type: error.name, component });
        
        this.monitor.log('error', error.message, {
            error: error.name,
            component,
            stack: error.stack,
            context,
            timestamp: new Date().toISOString()
        });
    }
    
    // Health check
    async getHealthStatus() {
        const metrics = await this.metrics.getMetrics();
        
        return {
            status: 'healthy',
            timestamp: new Date().toISOString(),
            uptime: process.uptime(),
            memory: process.memoryUsage(),
            metrics: {
                totalUsers: metrics.user_actions?.total || 0,
                totalTransactions: metrics.transactions?.total || 0,
                errorRate: this.calculateErrorRate(metrics),
                avgResponseTime: metrics.response_times?.avg || 0
            }
        };
    }
    
    private calculateErrorRate(metrics: any): number {
        const total = metrics.transactions?.total || 0;
        const errors = metrics.errors?.total || 0;
        return total > 0 ? (errors / total) * 100 : 0;
    }
}

// Usage in React components
export function useAnalytics() {
    const monitor = useContext(MonitoringContext);
    
    const trackPageView = useCallback((page: string) => {
        monitor.trackUserAction('page_view', getCurrentUserId(), { page });
    }, [monitor]);
    
    const trackButtonClick = useCallback((buttonName: string) => {
        monitor.trackUserAction('button_click', getCurrentUserId(), { buttonName });
    }, [monitor]);
    
    const trackTransactionStart = useCallback((type: string) => {
        const startTime = Date.now();
        
        return {
            finish: (txHash: string, status: 'success' | 'failed') => {
                const duration = Date.now() - startTime;
                monitor.trackTransaction(txHash, type, status);
                monitor.trackPerformance(`transaction_${type}`, duration);
            }
        };
    }, [monitor]);
    
    return {
        trackPageView,
        trackButtonClick,
        trackTransactionStart,
        trackError: monitor.trackError.bind(monitor)
    };
}
```

---

## Deployment Best Practices

### Production Deployment
```bash
#!/bin/bash
# deploy.sh - Production deployment script

set -e  # Exit on any error

echo "Starting ZHTP DApp deployment..."

# Environment validation
if [ -z "$ZHTP_API_KEY" ]; then
    echo "ZHTP_API_KEY not set"
    exit 1
fi

if [ -z "$CONTRACT_PRIVATE_KEY" ]; then
    echo "CONTRACT_PRIVATE_KEY not set"
    exit 1
fi

# Build and test
echo "Building application..."
npm ci --only=production
npm run build
npm run test:production

# Contract deployment
echo "Deploying smart contracts..."
npx hardhat deploy --network mainnet --verify

# Frontend deployment
echo "Deploying frontend..."
npm run deploy:frontend

# Domain setup
echo "Setting up domain..."
zhtp domain configure my-dapp.zhtp --contract-address $CONTRACT_ADDRESS

# SSL certificate
echo "Setting up SSL..."
zhtp cert create my-dapp.zhtp --auto-renew

# Health check
echo "Running health checks..."
curl -f https://my-dapp.zhtp/health || exit 1

echo "Deployment successful!"
echo "DApp available at: https://my-dapp.zhtp"
```

### CI/CD Pipeline
```yaml
# .github/workflows/deploy.yml
name: Deploy ZHTP DApp

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Run tests
      run: |
        npm run test:contracts
        npm run test:frontend
        npm run test:integration
    
    - name: Security audit
      run: npm audit --audit-level high
    
    - name: Lint code
      run: |
        npm run lint:contracts
        npm run lint:frontend

  deploy-testnet:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Deploy to testnet
      env:
        ZHTP_API_KEY: ${{ secrets.ZHTP_TESTNET_API_KEY }}
        PRIVATE_KEY: ${{ secrets.TESTNET_PRIVATE_KEY }}
      run: |
        npm ci
        npm run deploy:testnet
    
    - name: Run integration tests
      run: npm run test:integration:testnet

  deploy-mainnet:
    needs: [test, deploy-testnet]
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    environment: production
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Deploy to mainnet
      env:
        ZHTP_API_KEY: ${{ secrets.ZHTP_MAINNET_API_KEY }}
        PRIVATE_KEY: ${{ secrets.MAINNET_PRIVATE_KEY }}
      run: |
        npm ci --only=production
        npm run deploy:mainnet
    
    - name: Verify deployment
      run: npm run verify:mainnet
    
    - name: Update documentation
      run: npm run docs:update
```

---

## Documentation Best Practices

### Code Documentation
```typescript
/**
 * Voting DApp main contract interface
 * 
 * This contract manages proposal creation and voting with zero-knowledge privacy.
 * All votes are anonymous while maintaining verifiable integrity.
 * 
 * @example
 * ```typescript
 * const voting = new VotingContract('0x...');
 * 
 * // Create a proposal
 * const proposalId = await voting.createProposal(
 *   'Increase block rewards',
 *   'Proposal to increase mining rewards by 10%',
 *   7 * 24 * 60 * 60 // 7 days voting period
 * );
 * 
 * // Cast anonymous vote
 * await voting.vote(proposalId, true); // Vote yes
 * ```
 */
export class VotingContract {
    private contract: Contract;
    private signer: Signer;
    
    /**
     * Creates a new VotingContract instance
     * 
     * @param contractAddress - The deployed contract address
     * @param signerOrProvider - Ethereum signer or provider
     * 
     * @throws {Error} When contract address is invalid
     * @throws {Error} When ABI is not found
     */
    constructor(contractAddress: string, signerOrProvider: Signer | Provider) {
        if (!ethers.utils.isAddress(contractAddress)) {
            throw new Error('Invalid contract address');
        }
        
        this.contract = new Contract(contractAddress, VOTING_ABI, signerOrProvider);
        this.signer = signerOrProvider as Signer;
    }
    
    /**
     * Creates a new proposal for voting
     * 
     * @param title - Proposal title (max 100 characters)
     * @param description - Detailed description (max 1000 characters)
     * @param votingPeriod - Voting period in seconds (1-30 days)
     * 
     * @returns Promise resolving to the proposal ID
     * 
     * @throws {Error} When title/description is invalid
     * @throws {Error} When voting period is out of range
     * @throws {Error} When caller lacks admin privileges
     * 
     * @example
     * ```typescript
     * const proposalId = await voting.createProposal(
     *   'Network Upgrade',
     *   'Upgrade to new consensus mechanism',
     *   14 * 24 * 60 * 60 // 14 days
     * );
     * ```
     */
    async createProposal(
        title: string,
        description: string,
        votingPeriod: number
    ): Promise<number> {
        // Input validation
        if (!title || title.length > 100) {
            throw new Error('Title must be 1-100 characters');
        }
        
        if (!description || description.length > 1000) {
            throw new Error('Description must be 1-1000 characters');
        }
        
        const minPeriod = 24 * 60 * 60; // 1 day
        const maxPeriod = 30 * 24 * 60 * 60; // 30 days
        
        if (votingPeriod < minPeriod || votingPeriod > maxPeriod) {
            throw new Error('Voting period must be 1-30 days');
        }
        
        try {
            const tx = await this.contract.createProposal(title, description, votingPeriod);
            const receipt = await tx.wait();
            
            // Extract proposal ID from events
            const event = receipt.events?.find(e => e.event === 'ProposalCreated');
            return event?.args?.id?.toNumber() || 0;
            
        } catch (error) {
            if (error.message.includes('AccessControl')) {
                throw new Error('Only admins can create proposals');
            }
            throw error;
        }
    }
}
```

### API Documentation
```markdown
# Voting API Documentation

## Authentication

All API endpoints require authentication via JWT token or API key.

```bash
# Get JWT token
curl -X POST https://api.my-dapp.zhtp/auth \
  -H "Content-Type: application/json" \
  -d '{"wallet": "0x...", "signature": "0x..."}'

# Use token in requests
curl -H "Authorization: Bearer $JWT_TOKEN" \
  https://api.my-dapp.zhtp/proposals
```

## Endpoints

### GET /proposals

Returns a list of all proposals.

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10, max: 100)
- `status` (optional): Filter by status (`active`, `ended`, `executed`)

**Response:**
```json
{
  "proposals": [
    {
      "id": 1,
      "title": "Network Upgrade",
      "description": "Upgrade to new consensus mechanism",
      "status": "active",
      "endTime": "2024-01-15T00:00:00Z",
      "yesVotes": 1250,
      "noVotes": 320,
      "totalVotes": 1570
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 25,
    "totalPages": 3
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid query parameters
- `401 Unauthorized`: Missing or invalid authentication
- `429 Too Many Requests`: Rate limit exceeded

### POST /proposals

Creates a new proposal.

**Request Body:**
```json
{
  "title": "Network Upgrade",
  "description": "Detailed description of the proposal",
  "votingPeriod": 1209600
}
```

**Response:**
```json
{
  "id": 1,
  "title": "Network Upgrade",
  "status": "active",
  "txHash": "0x..."
}
```

**Error Responses:**
- `400 Bad Request`: Invalid proposal data
- `403 Forbidden`: Insufficient permissions
- `500 Internal Server Error`: Transaction failed
```

---

## Development Workflow

### Git Workflow
```bash
# Feature development workflow
git checkout -b feature/voting-improvements
git add .
git commit -m "feat: add proposal categories

- Add category field to proposals
- Update UI to display categories
- Add category filtering

Closes #123"

git push origin feature/voting-improvements

# Create pull request with:
# - Clear description
# - Screenshots/demos
# - Test coverage report
# - Security checklist
```

### Code Review Checklist
```markdown
## Code Review Checklist

### Functionality
- [ ] Code works as intended
- [ ] Edge cases are handled
- [ ] Error handling is appropriate
- [ ] Performance is acceptable

### Security
- [ ] Input validation is present
- [ ] No sensitive data in logs
- [ ] Access controls are correct
- [ ] No known vulnerabilities

### Code Quality
- [ ] Code is readable and well-documented
- [ ] Follows project conventions
- [ ] No code duplication
- [ ] Tests are comprehensive

### Smart Contracts
- [ ] Gas optimization applied
- [ ] Reentrancy protection present
- [ ] Events are emitted appropriately
- [ ] Access controls implemented

### Frontend
- [ ] Responsive design
- [ ] Accessibility compliance
- [ ] Loading states handled
- [ ] Error states handled
```

---

**Following these best practices will ensure your ZHTP DApp is secure, performant, and maintainable!**

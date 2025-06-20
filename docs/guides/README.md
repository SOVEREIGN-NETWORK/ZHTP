# Advanced ZHTP Development Guides

**Master-level guides for building production ZHTP applications**

## Featured Guides

### Getting Started
- [Smart Contracts](./smart-contracts.md) - Contract development
- [DApp Development](./dapp-development.md) - Frontend applications
- [Zero-Knowledge Integration](./zero-knowledge.md) - Privacy features
- [Domain & DNS](./domains-dns.md) - Decentralized naming

### Production Ready
- [Security Audit](./security-audit.md) - Security best practices
- [Performance Optimization](./performance.md) - Speed & scalability
- [Analytics & Monitoring](./analytics.md) - Track your DApp
- [UI/UX Design](./ui-ux-design.md) - Beautiful interfaces

### Advanced Topics
- [Multi-Chain Deployment](./multi-chain.md) - Cross-chain DApps
- [Automated Testing](./automated-testing.md) - Testing strategies
- [CI/CD Pipelines](./ci-cd.md) - Deployment automation
- [Monetization](./monetization.md) - Revenue strategies

---

## Zero-Knowledge Integration

**Build privacy-first applications with ZHTP's ZK infrastructure**

### Understanding ZK Proofs
```typescript
import { ZKProof, ZKCircuit } from '@zhtp/zk';

// Define your circuit
const circuit = new ZKCircuit({
  template: 'voting',
  inputs: ['vote', 'nullifier'],
  outputs: ['commitment']
});

// Generate proof
const proof = await ZKProof.generate({
  circuit,
  privateInputs: {
    vote: 1, // Private: your actual vote
    nullifier: generateNullifier()
  },
  publicInputs: {
    proposalId: 123 // Public: which proposal
  }
});

// Verify proof
const isValid = await ZKProof.verify({
  proof,
  verificationKey: circuit.verificationKey,
  publicSignals: proof.publicSignals
});
```

### Private Voting System
```solidity
// contracts/private_voting.rs
use zhtp_sdk::prelude::*;
use zhtp_contracts::ZKVoting;

#[contract]
pub struct PrivateVoting {
    #[storage]
    struct Storage {
        proposals: Map<u64, Proposal>,
        nullifier_hashes: Map<[u8; 32], bool>,
        proposal_count: u64,
    }

    #[derive(Storage)]
    struct Proposal {
        title: String,
        end_time: u64,
        total_votes: u64,
        executed: bool,
    }

    #[event]
    struct VoteCast {
        #[indexed]
        proposal_id: u64,
        nullifier_hash: [u8; 32],
    }
    
    impl PrivateVoting {
        #[transaction]
        pub fn vote(
            &mut self,
            proposal_id: u64,
            proof: ZKProof,
            public_signals: Vec<u8>,
        ) -> Result<(), ContractError> {
            // Verify the ZK proof
            if !self.verify_proof(&proof, &public_signals) {
                return Err(ContractError::InvalidProof);
            }

            let nullifier_hash = public_signals[0..32].try_into()?;
            if self.storage.nullifier_hashes.get(&nullifier_hash) {
                return Err(ContractError::DoubleVoting);
            }

            // Record the vote anonymously
            self.storage.nullifier_hashes.insert(nullifier_hash, true);
            
            let mut proposal = self.storage.proposals.get(proposal_id)
                .ok_or(ContractError::ProposalNotFound)?;
            proposal.total_votes += 1;
            self.storage.proposals.insert(proposal_id, proposal);

            self.emit(VoteCast {
                proposal_id,
                nullifier_hash,
            });

            Ok(())
        }
    }
```

### Frontend ZK Integration
```typescript
// components/PrivateVoting.tsx
import React, { useState } from 'react';
import { useZKProof, useContract } from '@zhtp/react-components';

function PrivateVoting({ proposalId }) {
    const [isGeneratingProof, setIsGeneratingProof] = useState(false);
    const contract = useContract('PrivateVoting');
    const { generateProof } = useZKProof();
    
    const castPrivateVote = async (vote: boolean) => {
        setIsGeneratingProof(true);
        
        try {
            // Generate ZK proof
            const proof = await generateProof({
                circuit: 'voting',
                privateInputs: {
                    vote: vote ? 1 : 0,
                    nullifier: generateNullifier()
                },
                publicInputs: {
                    proposalId
                }
            });
            
            // Submit vote to contract
            const tx = await contract.vote(
                proposalId,
                proof.a,
                proof.b,
                proof.c,
                proof.publicSignals
            );
            
            await tx.wait();
            console.log('Vote cast anonymously!');
            
        } catch (error) {
            console.error('Failed to cast vote:', error);
        } finally {
            setIsGeneratingProof(false);
        }
    };
    
    return (
        <div className="private-voting">
            <h3>Cast Anonymous Vote</h3>
            <div className="vote-buttons">
                <button 
                    onClick={() => castPrivateVote(true)}
                    disabled={isGeneratingProof}
                >
                    {isGeneratingProof ? 'Generating Proof...' : 'Vote Yes'}
                </button>
                <button 
                    onClick={() => castPrivateVote(false)}
                    disabled={isGeneratingProof}
                >
                    {isGeneratingProof ? 'Generating Proof...' : 'Vote No'}
                </button>
            </div>
        </div>
    );
}
```

---

## Security Audit Guide

**Comprehensive security practices for ZHTP DApps**

### Smart Contract Security
```solidity
// Security checklist for smart contracts
pragma solidity ^0.8.0;

import "@zhtp/contracts/ZKSecurity.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SecureDApp is ZKSecurity, ReentrancyGuard, Pausable, Ownable {
    // Use specific Solidity version
    // Import security contracts
    // Implement access controls
    
    mapping(address => uint256) private balances;
    
    // Prevent reentrancy attacks
    function withdraw(uint256 amount) external nonReentrant {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        // Update state before external calls
        balances[msg.sender] -= amount;
        
        // Use .call() instead of .transfer()
        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(success, "Transfer failed");
    }
    
    // Implement circuit breaker
    function emergencyPause() external onlyOwner {
        _pause();
    }
    
    // Input validation
    function setUserData(string memory data) external {
        require(bytes(data).length <= 280, "Data too long");
        require(bytes(data).length > 0, "Data cannot be empty");
        // Process data...
    }
    
    // Use events for transparency
    event SecurityEvent(address indexed user, string action, uint256 timestamp);
}
```

### Frontend Security
```typescript
// Frontend security best practices
import { sanitizeInput, validateAddress } from '@zhtp/security';

class SecurityManager {
    // Input sanitization
    static sanitizeUserInput(input: string): string {
        return sanitizeInput(input, {
            allowedTags: [],
            allowedAttributes: {}
        });
    }
    
    // Address validation
    static validateEthereumAddress(address: string): boolean {
        return validateAddress(address) && address.length === 42;
    }
    
    // Secure transaction signing
    static async secureSignTransaction(transaction: any, wallet: any) {
        // Validate transaction data
        if (!this.validateTransaction(transaction)) {
            throw new Error('Invalid transaction data');
        }
        
        // Show user confirmation
        const confirmed = await this.showTransactionConfirmation(transaction);
        if (!confirmed) {
            throw new Error('Transaction cancelled by user');
        }
        
        // Sign with user consent
        return await wallet.signTransaction(transaction);
    }
    
    // Rate limiting
    private static requestCounts = new Map<string, number>();
    
    static checkRateLimit(userAddress: string, maxRequests = 10): boolean {
        const count = this.requestCounts.get(userAddress) || 0;
        if (count >= maxRequests) {
            throw new Error('Rate limit exceeded');
        }
        this.requestCounts.set(userAddress, count + 1);
        return true;
    }
}
```

### Security Audit Checklist
```markdown
## Smart Contract Security

### Access Control
- [ ] Proper role-based access control
- [ ] Owner functions protected
- [ ] Multi-signature for critical operations
- [ ] Time locks for admin functions

### Reentrancy Protection
- [ ] ReentrancyGuard implemented
- [ ] State changes before external calls
- [ ] Check-Effect-Interaction pattern

### Input Validation
- [ ] All inputs validated
- [ ] Array length checks
- [ ] Integer overflow protection
- [ ] String length limits

### External Calls
- [ ] Use .call() instead of .transfer()
- [ ] Handle failed calls gracefully
- [ ] Avoid delegatecall with user input
- [ ] Gas limit considerations

## Frontend Security

### User Input
- [ ] All inputs sanitized
- [ ] XSS prevention
- [ ] CSRF protection
- [ ] SQL injection prevention

### Wallet Integration
- [ ] Secure key management
- [ ] Transaction confirmation
- [ ] Address validation
- [ ] Network verification

### Data Protection
- [ ] HTTPS everywhere
- [ ] Secure storage
- [ ] Privacy compliance
- [ ] Data encryption
```

---

## Performance Optimization

**Maximize speed and minimize costs**

### Smart Contract Optimization
```solidity
// Gas-optimized contract patterns
pragma solidity ^0.8.0;

contract OptimizedContract {
    // Pack structs efficiently
    struct User {
        uint128 balance;    // 16 bytes
        uint64 timestamp;   // 8 bytes
        uint32 nonce;       // 4 bytes
        bool active;        // 1 byte
        // Total: 32 bytes (1 slot)
    }
    
    // Use mappings instead of arrays for lookups
    mapping(address => User) public users;
    
    // Batch operations
    function batchTransfer(
        address[] calldata recipients,
        uint256[] calldata amounts
    ) external {
        require(recipients.length == amounts.length, "Array length mismatch");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            _transfer(msg.sender, recipients[i], amounts[i]);
        }
    }
    
    // Use unchecked for safe operations
    function efficientLoop(uint256 length) external pure returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < length;) {
            sum += i;
            unchecked { i++; }
        }
        return sum;
    }
    
    // Cache storage reads
    function optimizedFunction() external {
        User storage user = users[msg.sender];
        
        // Multiple operations on cached storage
        user.balance += 100;
        user.nonce++;
        user.timestamp = uint64(block.timestamp);
    }
}
```

### Frontend Performance
```typescript
// React performance optimization
import React, { memo, useMemo, useCallback, lazy, Suspense } from 'react';
import { useVirtualized } from '@zhtp/react-components';

// Lazy load components
const ExpensiveComponent = lazy(() => import('./ExpensiveComponent'));

// Memoize expensive components
const OptimizedList = memo(function OptimizedList({ items, onItemClick }) {
    // Memoize expensive calculations
    const processedItems = useMemo(() => {
        return items.map(item => ({
            ...item,
            displayName: formatDisplayName(item.name)
        }));
    }, [items]);
    
    // Memoize event handlers
    const handleItemClick = useCallback((itemId) => {
        onItemClick(itemId);
    }, [onItemClick]);
    
    // Use virtualization for large lists
    const { virtualItems, totalSize } = useVirtualized({
        count: processedItems.length,
        size: 50, // Item height
        overscan: 5
    });
    
    return (
        <div style={{ height: totalSize }}>
            {virtualItems.map(virtualItem => (
                <div
                    key={virtualItem.index}
                    style={{
                        position: 'absolute',
                        top: virtualItem.start,
                        height: virtualItem.size
                    }}
                >
                    <ItemComponent 
                        item={processedItems[virtualItem.index]}
                        onClick={handleItemClick}
                    />
                </div>
            ))}
        </div>
    );
});

// Suspense for code splitting
function App() {
    return (
        <Suspense fallback={<LoadingSpinner />}>
            <ExpensiveComponent />
        </Suspense>
    );
}
```

### ZHTP Network Optimization
```typescript
// Optimize ZHTP interactions
import { ZHTPClient, BatchRequest } from '@zhtp/sdk';

class OptimizedZHTPClient {
    private client: ZHTPClient;
    private cache = new Map();
    
    constructor() {
        this.client = new ZHTPClient({
            // Connection pooling
            maxConnections: 10,
            // Request batching
            batchRequests: true,
            // Caching
            cacheEnabled: true,
            cacheTTL: 30000 // 30 seconds
        });
    }
    
    // Batch multiple requests
    async batchGetBalances(addresses: string[]) {
        const batch = new BatchRequest();
        
        addresses.forEach(address => {
            batch.add('getBalance', [address]);
        });
        
        return await this.client.executeBatch(batch);
    }
    
    // Cache expensive calls
    async getCachedContractData(contractAddress: string, method: string) {
        const cacheKey = `${contractAddress}:${method}`;
        
        if (this.cache.has(cacheKey)) {
            return this.cache.get(cacheKey);
        }
        
        const result = await this.client.call(contractAddress, method);
        this.cache.set(cacheKey, result);
        
        // Auto-expire cache
        setTimeout(() => {
            this.cache.delete(cacheKey);
        }, 30000);
        
        return result;
    }
    
    // Optimistic updates
    async optimisticTransfer(to: string, amount: string) {
        // Update UI immediately
        this.updateUIBalance(to, amount);
        
        try {
            const tx = await this.client.transfer(to, amount);
            // Confirm success
            await tx.wait();
        } catch (error) {
            // Revert UI changes on failure
            this.revertUIBalance(to, amount);
            throw error;
        }
    }
}
```

---

## Analytics & Monitoring

**Track performance and user behavior**

### DApp Analytics Setup
```typescript
// Analytics integration
import { ZHTPAnalytics, EventTracker } from '@zhtp/analytics';

class DAppAnalytics {
    private analytics: ZHTPAnalytics;
    private eventTracker: EventTracker;
    
    constructor(config: AnalyticsConfig) {
        this.analytics = new ZHTPAnalytics({
            domain: config.domain,
            apiKey: config.apiKey,
            privacy: {
                respectDNT: true,
                anonymizeIPs: true,
                cookieConsent: true
            }
        });
        
        this.eventTracker = new EventTracker(this.analytics);
    }
    
    // Track user interactions
    trackUserAction(action: string, data?: any) {
        this.eventTracker.track('user_action', {
            action,
            timestamp: Date.now(),
            ...data
        });
    }
    
    // Track transaction metrics
    trackTransaction(txHash: string, type: string, value?: string) {
        this.eventTracker.track('transaction', {
            hash: txHash,
            type,
            value,
            network: this.analytics.network,
            timestamp: Date.now()
        });
    }
    
    // Track performance metrics
    trackPerformance(metric: string, value: number) {
        this.eventTracker.track('performance', {
            metric,
            value,
            userAgent: navigator.userAgent,
            timestamp: Date.now()
        });
    }
    
    // Track errors
    trackError(error: Error, context?: any) {
        this.eventTracker.track('error', {
            message: error.message,
            stack: error.stack,
            context,
            timestamp: Date.now()
        });
    }
}

// Usage in React components
function useAnalytics() {
    const analytics = useContext(AnalyticsContext);
    
    const trackPageView = useCallback((page: string) => {
        analytics.trackUserAction('page_view', { page });
    }, [analytics]);
    
    const trackButtonClick = useCallback((buttonName: string) => {
        analytics.trackUserAction('button_click', { buttonName });
    }, [analytics]);
    
    return {
        trackPageView,
        trackButtonClick,
        trackTransaction: analytics.trackTransaction.bind(analytics),
        trackError: analytics.trackError.bind(analytics)
    };
}
```

### Monitoring Dashboard
```typescript
// Real-time monitoring
import { ZHTPMonitor, AlertManager } from '@zhtp/monitoring';

class DAppMonitor {
    private monitor: ZHTPMonitor;
    private alerts: AlertManager;
    
    constructor() {
        this.monitor = new ZHTPMonitor({
            checkInterval: 30000, // 30 seconds
            metrics: [
                'response_time',
                'error_rate',
                'user_activity',
                'transaction_volume'
            ]
        });
        
        this.alerts = new AlertManager({
            webhooks: [
                'https://discord.com/api/webhooks/...',
                'https://hooks.slack.com/services/...'
            ]
        });
        
        this.setupAlerts();
    }
    
    private setupAlerts() {
        // High error rate alert
        this.monitor.addAlert({
            metric: 'error_rate',
            condition: 'greater_than',
            threshold: 5, // 5%
            duration: 300, // 5 minutes
            action: () => {
                this.alerts.send('High error rate detected!');
            }
        });
        
        // Slow response time alert
        this.monitor.addAlert({
            metric: 'response_time',
            condition: 'greater_than',
            threshold: 2000, // 2 seconds
            duration: 180, // 3 minutes
            action: () => {
                this.alerts.send('Slow response times detected!');
            }
        });
        
        // Low user activity alert
        this.monitor.addAlert({
            metric: 'user_activity',
            condition: 'less_than',
            threshold: 10, // 10 users/hour
            duration: 3600, // 1 hour
            action: () => {
                this.alerts.send('User activity is low');
            }
        });
    }
    
    // Custom metrics
    trackCustomMetric(name: string, value: number, tags?: any) {
        this.monitor.recordMetric(name, value, {
            timestamp: Date.now(),
            ...tags
        });
    }
    
    // Health check endpoint
    async getHealthStatus() {
        return {
            status: 'healthy',
            uptime: process.uptime(),
            metrics: await this.monitor.getMetrics(),
            alerts: this.alerts.getActiveAlerts()
        };
    }
}
```

---

## UI/UX Design Guide

**Create beautiful, intuitive interfaces**

### Design System
```typescript
// Design tokens
export const theme = {
    colors: {
        primary: {
            50: '#EFF6FF',
            100: '#DBEAFE', 
            500: '#3B82F6',
            600: '#2563EB',
            900: '#1E3A8A'
        },
        semantic: {
            success: '#10B981',
            warning: '#F59E0B',
            error: '#EF4444',
            info: '#3B82F6'
        }
    },
    spacing: {
        xs: '0.25rem',   // 4px
        sm: '0.5rem',    // 8px
        md: '1rem',      // 16px
        lg: '1.5rem',    // 24px
        xl: '2rem',      // 32px
        '2xl': '3rem'    // 48px
    },
    typography: {
        fontFamily: {
            sans: ['Inter', 'system-ui', 'sans-serif'],
            mono: ['Monaco', 'Consolas', 'monospace']
        },
        fontSize: {
            xs: '0.75rem',
            sm: '0.875rem',
            base: '1rem',
            lg: '1.125rem',
            xl: '1.25rem',
            '2xl': '1.5rem',
            '3xl': '1.875rem'
        }
    },
    borderRadius: {
        none: '0',
        sm: '0.125rem',
        md: '0.375rem',
        lg: '0.5rem',
        full: '9999px'
    }
};
```

### Component Patterns
```tsx
// Consistent component patterns
import { clsx } from 'clsx';

// Variant-based components
interface ButtonProps {
    variant?: 'primary' | 'secondary' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    loading?: boolean;
    children: React.ReactNode;
    onClick?: () => void;
}

export function Button({ 
    variant = 'primary', 
    size = 'md', 
    loading = false,
    children,
    onClick 
}: ButtonProps) {
    return (
        <button
            className={clsx(
                // Base styles
                'font-medium rounded-md transition-colors focus:outline-none focus:ring-2',
                // Variant styles
                {
                    'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500': variant === 'primary',
                    'bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500': variant === 'secondary',
                    'text-blue-600 hover:bg-blue-50 focus:ring-blue-500': variant === 'ghost'
                },
                // Size styles
                {
                    'px-3 py-1.5 text-sm': size === 'sm',
                    'px-4 py-2 text-base': size === 'md',
                    'px-6 py-3 text-lg': size === 'lg'
                },
                // Loading state
                {
                    'opacity-50 cursor-not-allowed': loading
                }
            )}
            disabled={loading}
            onClick={onClick}
        >
            {loading && (
                <svg className="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z" />
                </svg>
            )}
            {children}
        </button>
    );
}

// Responsive layouts
export function ResponsiveGrid({ children }: { children: React.ReactNode }) {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {children}
        </div>
    );
}

// Loading states
export function LoadingCard() {
    return (
        <div className="bg-white p-6 rounded-lg shadow animate-pulse">
            <div className="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
            <div className="h-4 bg-gray-200 rounded w-1/2 mb-2"></div>
            <div className="h-4 bg-gray-200 rounded w-5/6"></div>
        </div>
    );
}
```

### Accessibility
```tsx
// Accessible components
import { useId, useRef, useEffect } from 'react';

interface AccessibleInputProps {
    label: string;
    error?: string;
    required?: boolean;
    type?: string;
}

export function AccessibleInput({ 
    label, 
    error, 
    required = false,
    type = 'text',
    ...props 
}: AccessibleInputProps) {
    const id = useId();
    const errorId = useId();
    const inputRef = useRef<HTMLInputElement>(null);
    
    // Focus management
    useEffect(() => {
        if (error && inputRef.current) {
            inputRef.current.focus();
        }
    }, [error]);
    
    return (
        <div className="mb-4">
            <label 
                htmlFor={id}
                className="block text-sm font-medium text-gray-700 mb-1"
            >
                {label}
                {required && <span className="text-red-500 ml-1">*</span>}
            </label>
            
            <input
                ref={inputRef}
                id={id}
                type={type}
                required={required}
                aria-invalid={!!error}
                aria-describedby={error ? errorId : undefined}
                className={clsx(
                    'w-full px-3 py-2 border rounded-md focus:ring-2 focus:outline-none',
                    {
                        'border-gray-300 focus:ring-blue-500': !error,
                        'border-red-300 focus:ring-red-500': !!error
                    }
                )}
                {...props}
            />
            
            {error && (
                <p id={errorId} className="mt-1 text-sm text-red-600" role="alert">
                    {error}
                </p>
            )}
        </div>
    );
}

// Keyboard navigation
export function AccessibleModal({ isOpen, onClose, children }: ModalProps) {
    const modalRef = useRef<HTMLDivElement>(null);
    
    useEffect(() => {
        if (!isOpen) return;
        
        // Trap focus within modal
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === 'Escape') {
                onClose();
            }
            
            if (e.key === 'Tab') {
                // Handle tab trapping logic
                trapFocus(e, modalRef.current);
            }
        };
        
        document.addEventListener('keydown', handleKeyDown);
        
        // Focus first element
        const firstFocusable = modalRef.current?.querySelector(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        ) as HTMLElement;
        firstFocusable?.focus();
        
        return () => {
            document.removeEventListener('keydown', handleKeyDown);
        };
    }, [isOpen, onClose]);
    
    if (!isOpen) return null;
    
    return (
        <div 
            className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
            role="dialog"
            aria-modal="true"
        >
            <div ref={modalRef} className="bg-white p-6 rounded-lg max-w-md w-full">
                {children}
            </div>
        </div>
    );
}
```

---

## Monetization Strategies

**Turn your ZHTP DApp into a profitable business**

### Revenue Models
```typescript
// Subscription-based revenue
class SubscriptionManager {
    private plans = {
        basic: { price: 10, features: ['feature1', 'feature2'] },
        premium: { price: 25, features: ['feature1', 'feature2', 'feature3'] },
        enterprise: { price: 100, features: ['all'] }
    };
    
    async createSubscription(userAddress: string, plan: string) {
        const subscription = await this.contract.createSubscription(
            userAddress,
            plan,
            this.plans[plan].price
        );
        
        return subscription;
    }
    
    async checkAccess(userAddress: string, feature: string) {
        const subscription = await this.getUserSubscription(userAddress);
        
        if (!subscription || subscription.expired) {
            return false;
        }
        
        const plan = this.plans[subscription.plan];
        return plan.features.includes(feature) || plan.features.includes('all');
    }
}

// Transaction fee revenue
class TransactionFeeManager {
    private feePercentage = 0.025; // 2.5%
    
    async processTransaction(from: string, to: string, amount: bigint) {
        const fee = (amount * BigInt(Math.floor(this.feePercentage * 10000))) / 10000n;
        const netAmount = amount - fee;
        
        // Transfer net amount to recipient
        await this.transfer(from, to, netAmount);
        
        // Transfer fee to protocol
        await this.transfer(from, this.protocolAddress, fee);
        
        return { netAmount, fee };
    }
}

// NFT marketplace revenue
class NFTMarketplace {
    private marketplaceFee = 250; // 2.5% in basis points
    
    async listNFT(tokenId: number, price: bigint, seller: string) {
        await this.contract.listNFT(tokenId, price, seller);
    }
    
    async buyNFT(tokenId: number, buyer: string) {
        const listing = await this.contract.getListing(tokenId);
        const fee = (listing.price * BigInt(this.marketplaceFee)) / 10000n;
        const sellerAmount = listing.price - fee;
        
        // Transfer payment
        await this.transfer(buyer, listing.seller, sellerAmount);
        await this.transfer(buyer, this.marketplaceAddress, fee);
        
        // Transfer NFT
        await this.contract.transferNFT(listing.seller, buyer, tokenId);
    }
}
```

### Token Economics
```solidity
// Utility token with staking rewards
use zhtp_sdk::prelude::*;
use zhtp_token::Token;

#[contract]
pub struct UtilityToken {
    #[storage]
    struct Storage {
        stakes: Map<Address, StakeInfo>,
        total_staked: u128,
        reward_rate: u128, // 1% per day
        token: Token,
    }

    #[derive(Storage)]
    struct StakeInfo {
        amount: u128,
        timestamp: u64,
        reward_debt: u128,
    }

    impl UtilityToken {
        #[init]
        pub fn new() -> Self {
            let mut storage = Storage::default();
            storage.reward_rate = 100;
            storage.token = Token::new("UTIL", "Utility Token", 18);
            Self { storage }
        }

        // Staking for governance and rewards
        #[transaction]
        pub fn stake(&mut self, amount: u128) -> Result<(), ContractError> {
            if amount == 0 {
                return Err(ContractError::InvalidAmount);
            }

            let sender = self.context().sender();
            if self.storage.token.balance_of(&sender) < amount {
                return Err(ContractError::InsufficientBalance);
            }

            // Claim pending rewards
            self.claim_rewards()?;

            // Update stake
            let mut stake = self.storage.stakes.get(&sender)
                .unwrap_or_default();
            stake.amount += amount;
            stake.timestamp = self.context().block_time();
            self.storage.total_staked += amount;
            self.storage.stakes.insert(&sender, stake);

            // Transfer tokens to contract
            self.storage.token.transfer_from(
                &sender,
                &self.context().contract_address(),
                amount,
            )?;

            Ok(())
        }

        #[transaction]
        pub fn claim_rewards(&mut self) -> Result<(), ContractError> {
            let sender = self.context().sender();
            let mut stake = match self.storage.stakes.get(&sender) {
                Some(s) if s.amount > 0 => s,
                _ => return Ok(()),
            };

            let time_staked = self.context().block_time()
                .saturating_sub(stake.timestamp);
            let rewards = stake.amount
                .saturating_mul(self.storage.reward_rate)
                .saturating_mul(time_staked.into())
                .checked_div(100 * 86400)
                .ok_or(ContractError::MathError)?;

            if rewards > 0 {
                stake.reward_debt += rewards;
                stake.timestamp = self.context().block_time();
                self.storage.stakes.insert(&sender, stake);

                // Mint rewards
                self.storage.token.mint(&sender, rewards)?;
            }

            Ok(())
        }

        // Governance voting power based on stake
        #[view]
        pub fn get_voting_power(&self, user: Address) -> u128 {
            self.storage.stakes.get(&user)
                .map(|stake| stake.amount)
                .unwrap_or_default()
        }
    }
}
```

---

## Multi-Chain Deployment

**Deploy your DApp across multiple blockchain networks**

### Cross-Chain Architecture
```typescript
// Multi-chain contract manager
import { ChainConfig, BridgeManager } from '@zhtp/multi-chain';

class MultiChainDApp {
    private chains: Map<string, ChainConfig> = new Map();
    private bridge: BridgeManager;
    
    constructor() {
        // Configure supported chains
        this.chains.set('ethereum', {
            name: 'Ethereum',
            rpcUrl: 'https://eth-mainnet.alchemyapi.io/v2/...',
            contractAddress: '0x...',
            nativeToken: 'ETH'
        });
        
        this.chains.set('polygon', {
            name: 'Polygon',
            rpcUrl: 'https://polygon-rpc.com',
            contractAddress: '0x...',
            nativeToken: 'MATIC'
        });
        
        this.chains.set('zhtp', {
            name: 'ZHTP Network',
            rpcUrl: 'https://rpc.zhtp.network',
            contractAddress: '0x...',
            nativeToken: 'ZHTP'
        });
        
        this.bridge = new BridgeManager(this.chains);
    }
    
    // Deploy to multiple chains
    async deployToAllChains(contractCode: string) {
        const deployments = new Map();
        
        for (const [chainId, config] of this.chains) {
            try {
                const address = await this.deployContract(chainId, contractCode);
                deployments.set(chainId, address);
                console.log(`Deployed to ${config.name}: ${address}`);
            } catch (error) {
                console.error(`Failed to deploy to ${config.name}:`, error);
            }
        }
        
        return deployments;
    }
    
    // Cross-chain asset transfer
    async bridgeAssets(
        fromChain: string,
        toChain: string,
        amount: bigint,
        token: string
    ) {
        return await this.bridge.transfer({
            fromChain,
            toChain,
            amount,
            token,
            recipient: await this.getCurrentUser()
        });
    }
    
    // Unified state management
    async syncState(contractMethod: string, args: any[]) {
        const results = new Map();
        
        for (const [chainId] of this.chains) {
            try {
                const result = await this.callContract(chainId, contractMethod, args);
                results.set(chainId, result);
            } catch (error) {
                console.error(`Failed to call ${contractMethod} on ${chainId}:`, error);
            }
        }
        
        return results;
    }
}
```

---

## More Guides Coming Soon

- **Automated Testing** - Comprehensive testing strategies
- **CI/CD Pipelines** - Deployment automation
- **Domain & DNS** - Decentralized naming systems
- **API Integration** - External service integration

---

## Resources

- [API Documentation](../api/)
- [UI Components](../ui/)
- [Examples](../examples/)
- [Developer Discord](https://discord.gg/zhtp-dev)

**Master ZHTP development with these comprehensive guides!**

# ZHTP React Components

**Production-ready React components for building ZHTP DApps**

## Quick Start

```bash
npm install @zhtp/react-components
```

```tsx
import { ZHTPProvider, ZHTPButton, ZHTPCard } from '@zhtp/react-components';

function App() {
    return (
        <ZHTPProvider network="mainnet">
            <ZHTPCard title="Welcome to ZHTP">
                <ZHTPButton>Get Started</ZHTPButton>
            </ZHTPCard>
        </ZHTPProvider>
    );
}
```

## Core Components

### `<ZHTPProvider />`
Root provider that connects your app to the ZHTP network.

```tsx
import { ZHTPProvider } from '@zhtp/react-components';

function App() {
    return (
        <ZHTPProvider 
            network="mainnet"  // or "testnet"
            theme="dark"       // or "light"
            autoConnect={true}
        >
            <YourApp />
        </ZHTPProvider>
    );
}
```

**Props:**
- `network`: `"mainnet" | "testnet"` - ZHTP network to connect to
- `theme`: `"light" | "dark"` - UI theme
- `autoConnect`: `boolean` - Auto-connect to wallet on load
- `config`: `ZHTPConfig` - Advanced configuration

---

### `<ZHTPButton />`
Smart button with built-in ZHTP styling and loading states.

```tsx
import { ZHTPButton } from '@zhtp/react-components';

function MyComponent() {
    return (
        <div>
            {/* Basic button */}
            <ZHTPButton onClick={() => console.log('clicked')}>
                Click Me
            </ZHTPButton>
            
            {/* Loading state */}
            <ZHTPButton loading={true}>
                Processing...
            </ZHTPButton>
            
            {/* Variants */}
            <ZHTPButton variant="primary">Primary</ZHTPButton>
            <ZHTPButton variant="secondary">Secondary</ZHTPButton>
            <ZHTPButton variant="success">Success</ZHTPButton>
            <ZHTPButton variant="danger">Danger</ZHTPButton>
            <ZHTPButton variant="ghost">Ghost</ZHTPButton>
            
            {/* Sizes */}
            <ZHTPButton size="sm">Small</ZHTPButton>
            <ZHTPButton size="md">Medium</ZHTPButton>
            <ZHTPButton size="lg">Large</ZHTPButton>
            
            {/* Icons */}
            <ZHTPButton icon="wallet">Connect Wallet</ZHTPButton>
            <ZHTPButton icon="send" iconPosition="right">Send</ZHTPButton>
        </div>
    );
}
```

**Props:**
- `variant`: `"primary" | "secondary" | "success" | "danger" | "ghost"`
- `size`: `"sm" | "md" | "lg"`
- `loading`: `boolean`
- `disabled`: `boolean`
- `icon`: `string` - Icon name
- `iconPosition`: `"left" | "right"`
- `fullWidth`: `boolean`

---

### `<ZHTPCard />`
Container component with ZHTP styling.

```tsx
import { ZHTPCard } from '@zhtp/react-components';

function MyComponent() {
    return (
        <div>
            {/* Basic card */}
            <ZHTPCard title="Card Title">
                <p>Card content goes here</p>
            </ZHTPCard>
            
            {/* With actions */}
            <ZHTPCard 
                title="DApp Details"
                subtitle="Decentralized Application"
                actions={
                    <ZHTPButton size="sm">View More</ZHTPButton>
                }
            >
                <p>This is a sample DApp description.</p>
            </ZHTPCard>
            
            {/* Loading state */}
            <ZHTPCard title="Loading..." loading={true}>
                <p>Content loading...</p>
            </ZHTPCard>
        </div>
    );
}
```

**Props:**
- `title`: `string`
- `subtitle`: `string`
- `loading`: `boolean`
- `actions`: `ReactNode`
- `className`: `string`

---

## Wallet Components

### `<WalletConnection />`
Complete wallet connection interface.

```tsx
import { WalletConnection } from '@zhtp/react-components';

function MyApp() {
    return (
        <WalletConnection
            onConnect={(account) => console.log('Connected:', account)}
            onDisconnect={() => console.log('Disconnected')}
            showBalance={true}
            showNetwork={true}
            providers={['metamask', 'walletconnect', 'coinbase']}
        />
    );
}
```

### `<WalletButton />`
Simple connect/disconnect button.

```tsx
import { WalletButton } from '@zhtp/react-components';

function Header() {
    return (
        <nav>
            <h1>My DApp</h1>
            <WalletButton />
        </nav>
    );
}
```

### `<AccountInfo />`
Display connected account information.

```tsx
import { AccountInfo } from '@zhtp/react-components';

function Sidebar() {
    return (
        <div>
            <AccountInfo 
                showAddress={true}
                showBalance={true}
                showNetwork={true}
                copyable={true}
            />
        </div>
    );
}
```

---

## Contract Components

### `<ContractCall />`
Easy contract interaction component.

```tsx
import { ContractCall } from '@zhtp/react-components';

function VotingInterface() {
    return (
        <ContractCall
            contractAddress="0x1234..."
            abi={VotingABI}
            method="vote"
            args={[proposalId, true]}
            onSuccess={(result) => console.log('Vote cast!', result)}
            onError={(error) => console.error('Vote failed:', error)}
        >
            {({ call, loading, error }) => (
                <div>
                    <ZHTPButton 
                        onClick={call}
                        loading={loading}
                        disabled={!!error}
                    >
                        Vote Yes
                    </ZHTPButton>
                    {error && <p className="error">{error.message}</p>}
                </div>
            )}
        </ContractCall>
    );
}
```

### `<ContractRead />`
Read data from contracts.

```tsx
import { ContractRead } from '@zhtp/react-components';

function ProposalInfo({ proposalId }) {
    return (
        <ContractRead
            contractAddress="0x1234..."
            abi={VotingABI}
            method="getProposal"
            args={[proposalId]}
            watch={true} // Auto-refresh on changes
        >
            {({ data, loading, error }) => (
                <div>
                    {loading && <p>Loading proposal...</p>}
                    {error && <p>Error: {error.message}</p>}
                    {data && (
                        <div>
                            <h3>{data.title}</h3>
                            <p>{data.description}</p>
                            <p>Yes votes: {data.yesVotes}</p>
                            <p>No votes: {data.noVotes}</p>
                        </div>
                    )}
                </div>
            )}
        </ContractRead>
    );
}
```

---

## UI Components

### `<ZHTPInput />`
Styled input fields.

```tsx
import { ZHTPInput } from '@zhtp/react-components';

function Form() {
    const [value, setValue] = useState('');
    
    return (
        <div>
            <ZHTPInput
                label="Username"
                placeholder="Enter username"
                value={value}
                onChange={setValue}
                required={true}
                helpText="Must be unique"
            />
            
            <ZHTPInput
                type="email"
                label="Email"
                error="Invalid email format"
            />
            
            <ZHTPInput
                type="password"
                label="Password"
                secure={true}
            />
        </div>
    );
}
```

### `<ZHTPModal />`
Modal dialogs.

```tsx
import { ZHTPModal, ZHTPButton } from '@zhtp/react-components';

function MyComponent() {
    const [showModal, setShowModal] = useState(false);
    
    return (
        <div>
            <ZHTPButton onClick={() => setShowModal(true)}>
                Open Modal
            </ZHTPButton>
            
            <ZHTPModal
                isOpen={showModal}
                onClose={() => setShowModal(false)}
                title="Confirm Transaction"
                size="md"
            >
                <p>Are you sure you want to proceed?</p>
                <div className="modal-actions">
                    <ZHTPButton variant="ghost" onClick={() => setShowModal(false)}>
                        Cancel
                    </ZHTPButton>
                    <ZHTPButton variant="primary">
                        Confirm
                    </ZHTPButton>
                </div>
            </ZHTPModal>
        </div>
    );
}
```

### `<ZHTPTabs />`
Tab navigation.

```tsx
import { ZHTPTabs } from '@zhtp/react-components';

function Dashboard() {
    const tabs = [
        {
            id: 'overview',
            label: 'Overview',
            content: <OverviewPanel />
        },
        {
            id: 'transactions',
            label: 'Transactions',
            content: <TransactionsPanel />
        },
        {
            id: 'settings',
            label: 'Settings',
            content: <SettingsPanel />
        }
    ];
    
    return (
        <ZHTPTabs 
            tabs={tabs}
            defaultTab="overview"
            onChange={(tabId) => console.log('Tab changed:', tabId)}
        />
    );
}
```

---

## Data Components

### `<TokenBalance />`
Display token balances.

```tsx
import { TokenBalance } from '@zhtp/react-components';

function Portfolio() {
    return (
        <div>
            <TokenBalance 
                token="ZHTP"
                address="0x1234..."
                format="short" // or "full"
                showUSD={true}
            />
            
            <TokenBalance 
                token="USDC"
                contractAddress="0x5678..."
                refreshInterval={5000}
            />
        </div>
    );
}
```

### `<TransactionHistory />`
Display transaction history.

```tsx
import { TransactionHistory } from '@zhtp/react-components';

function AccountPage() {
    return (
        <TransactionHistory
            address="0x1234..."
            limit={10}
            showPagination={true}
            onTransactionClick={(tx) => console.log('Transaction:', tx)}
        />
    );
}
```

### `<NFTGallery />`
Display NFT collections.

```tsx
import { NFTGallery } from '@zhtp/react-components';

function NFTPage() {
    return (
        <NFTGallery
            owner="0x1234..."
            collection="0x5678..." // optional
            gridSize="md"
            showMetadata={true}
            onNFTClick={(nft) => console.log('NFT clicked:', nft)}
        />
    );
}
```

---

## Advanced Components

### `<ZKProofVerifier />`
Verify zero-knowledge proofs.

```tsx
import { ZKProofVerifier } from '@zhtp/react-components';

function PrivateVoting() {
    return (
        <ZKProofVerifier
            proof={voterProof}
            publicSignals={publicSignals}
            verificationKey={vk}
            onVerified={(isValid) => {
                if (isValid) {
                    console.log('Proof verified! Vote is anonymous.');
                }
            }}
        >
            {({ isVerifying, isValid, error }) => (
                <div>
                    {isVerifying && <p>Verifying proof...</p>}
                    {isValid && <p>Anonymous vote verified</p>}
                    {error && <p>Proof verification failed</p>}
                </div>
            )}
        </ZKProofVerifier>
    );
}
```

### `<ConsensusStatus />`
Display network consensus information.

```tsx
import { ConsensusStatus } from '@zhtp/react-components';

function NetworkDashboard() {
    return (
        <ConsensusStatus
            showValidators={true}
            showProposals={true}
            refreshInterval={10000}
        />
    );
}
```

---

## Theming

### Custom Theme
```tsx
import { ZHTPProvider, createTheme } from '@zhtp/react-components';

const customTheme = createTheme({
    colors: {
        primary: '#007AFF',
        secondary: '#5856D6',
        success: '#34C759',
        danger: '#FF3B30',
        warning: '#FF9500',
        background: '#FFFFFF',
        surface: '#F8F9FA',
        text: '#1C1C1E'
    },
    fonts: {
        sans: 'Inter, sans-serif',
        mono: 'Monaco, monospace'
    },
    spacing: {
        xs: '0.25rem',
        sm: '0.5rem',
        md: '1rem',
        lg: '1.5rem',
        xl: '2rem'
    },
    borderRadius: {
        sm: '0.25rem',
        md: '0.5rem',
        lg: '1rem'
    }
});

function App() {
    return (
        <ZHTPProvider theme={customTheme}>
            <YourApp />
        </ZHTPProvider>
    );
}
```

### Dark Mode
```tsx
import { ZHTPProvider } from '@zhtp/react-components';

function App() {
    const [isDark, setIsDark] = useState(false);
    
    return (
        <ZHTPProvider theme={isDark ? 'dark' : 'light'}>
            <button onClick={() => setIsDark(!isDark)}>
                Toggle Theme
            </button>
            <YourApp />
        </ZHTPProvider>
    );
}
```

---

## Hooks

### `useZHTP()`
Main hook for ZHTP functionality.

```tsx
import { useZHTP } from '@zhtp/react-components';

function MyComponent() {
    const { 
        account,           // Connected account address
        balance,           // Account balance
        network,           // Current network
        provider,          // ZHTP provider
        connect,           // Connect function
        disconnect,        // Disconnect function
        isConnected,       // Connection status
        isConnecting,      // Connection loading
        switchNetwork      // Network switching
    } = useZHTP();
    
    return (
        <div>
            {isConnected ? (
                <p>Connected: {account}</p>
            ) : (
                <button onClick={connect}>Connect</button>
            )}
        </div>
    );
}
```

### `useContract()`
Interact with smart contracts.

```tsx
import { useContract } from '@zhtp/react-components';

function VotingComponent() {
    const contract = useContract({
        address: '0x1234...',
        abi: VotingABI
    });
    
    const vote = async (proposalId: number, support: boolean) => {
        const tx = await contract.vote(proposalId, support);
        await tx.wait();
    };
    
    return (
        <button onClick={() => vote(1, true)}>
            Vote Yes
        </button>
    );
}
```

### `useBalance()`
Get token balances.

```tsx
import { useBalance } from '@zhtp/react-components';

function BalanceDisplay() {
    const { 
        balance,
        loading,
        error,
        refresh
    } = useBalance({
        token: 'ZHTP',
        address: '0x1234...',
        watch: true // Auto-refresh
    });
    
    return (
        <div>
            {loading ? 'Loading...' : `${balance} ZHTP`}
            <button onClick={refresh}>Refresh</button>
        </div>
    );
}
```

---

## Responsive Design

All components are mobile-responsive by default:

```tsx
import { ZHTPCard, ZHTPButton } from '@zhtp/react-components';

function ResponsiveLayout() {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <ZHTPCard title="Mobile First">
                <p>Automatically responsive</p>
                <ZHTPButton fullWidth>Full Width on Mobile</ZHTPButton>
            </ZHTPCard>
        </div>
    );
}
```

---

## Best Practices

### Performance
```tsx
// Use memo for expensive components
import { memo } from 'react';

const ExpensiveComponent = memo(function ExpensiveComponent({ data }) {
    return <ZHTPCard>{/* complex rendering */}</ZHTPCard>;
});

// Use useMemo for expensive calculations
const processedData = useMemo(() => {
    return expensiveCalculation(rawData);
}, [rawData]);
```

### Error Handling
```tsx
import { ZHTPErrorBoundary } from '@zhtp/react-components';

function App() {
    return (
        <ZHTPProvider>
            <ZHTPErrorBoundary>
                <YourApp />
            </ZHTPErrorBoundary>
        </ZHTPProvider>
    );
}
```

### Loading States
```tsx
function DataComponent() {
    const { data, loading, error } = useContractRead(...);
    
    if (loading) return <ZHTPSkeleton />;
    if (error) return <ZHTPAlert variant="danger">{error.message}</ZHTPAlert>;
    
    return <div>{data}</div>;
}
```

---

## Installation & Setup

```bash
# Install React components
npm install @zhtp/react-components

# Install peer dependencies
npm install react react-dom

# TypeScript support (optional)
npm install --save-dev @types/react @types/react-dom
```

```tsx
// src/index.tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import { ZHTPProvider } from '@zhtp/react-components';
import '@zhtp/react-components/dist/styles.css';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));

root.render(
    <React.StrictMode>
        <ZHTPProvider network="mainnet">
            <App />
        </ZHTPProvider>
    </React.StrictMode>
);
```

---

## Links

- [Component Storybook](https://storybook.zhtp.dev)
- [Design System](https://design.zhtp.dev)
- [Report Issues](https://github.com/zhtp/react-components/issues)
- [Discord Support](https://discord.gg/zhtp)

**Build beautiful ZHTP DApps with confidence!**

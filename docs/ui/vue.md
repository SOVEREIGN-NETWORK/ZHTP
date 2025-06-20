# ZHTP Vue Components

**Vue.js components for building ZHTP DApps with ease**

## Quick Start

```bash
npm install @zhtp/vue-components
```

```vue
<template>
  <ZHTPProvider network="mainnet">
    <ZHTPCard title="Welcome to ZHTP">
      <ZHTPButton @click="handleClick">Get Started</ZHTPButton>
    </ZHTPCard>
  </ZHTPProvider>
</template>

<script setup>
import { ZHTPProvider, ZHTPCard, ZHTPButton } from '@zhtp/vue-components'

const handleClick = () => {
  console.log('Getting started with ZHTP!')
}
</script>
```

## Core Components

### `<ZHTPProvider />`
Root provider that connects your app to the ZHTP network.

```vue
<template>
  <ZHTPProvider 
    network="mainnet"
    theme="dark"
    :auto-connect="true"
    @connected="onConnected"
    @disconnected="onDisconnected"
  >
    <YourApp />
  </ZHTPProvider>
</template>

<script setup>
import { ZHTPProvider } from '@zhtp/vue-components'

const onConnected = (account) => {
  console.log('Connected:', account)
}

const onDisconnected = () => {
  console.log('Disconnected')
}
</script>
```

**Props:**
- `network`: `"mainnet" | "testnet"` - ZHTP network to connect to
- `theme`: `"light" | "dark"` - UI theme
- `autoConnect`: `boolean` - Auto-connect to wallet on load
- `config`: `ZHTPConfig` - Advanced configuration

**Events:**
- `@connected`: Fired when wallet connects
- `@disconnected`: Fired when wallet disconnects
- `@network-changed`: Fired when network changes

---

### `<ZHTPButton />`
Smart button with built-in ZHTP styling and loading states.

```vue
<template>
  <div>
    <!-- Basic button -->
    <ZHTPButton @click="handleClick">
      Click Me
    </ZHTPButton>
    
    <!-- Loading state -->
    <ZHTPButton :loading="isLoading">
      {{ isLoading ? 'Processing...' : 'Submit' }}
    </ZHTPButton>
    
    <!-- Variants -->
    <ZHTPButton variant="primary">Primary</ZHTPButton>
    <ZHTPButton variant="secondary">Secondary</ZHTPButton>
    <ZHTPButton variant="success">Success</ZHTPButton>
    <ZHTPButton variant="danger">Danger</ZHTPButton>
    <ZHTPButton variant="ghost">Ghost</ZHTPButton>
    
    <!-- Sizes -->
    <ZHTPButton size="sm">Small</ZHTPButton>
    <ZHTPButton size="md">Medium</ZHTPButton>
    <ZHTPButton size="lg">Large</ZHTPButton>
    
    <!-- Icons -->
    <ZHTPButton icon="wallet">Connect Wallet</ZHTPButton>
    <ZHTPButton icon="send" icon-position="right">Send</ZHTPButton>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ZHTPButton } from '@zhtp/vue-components'

const isLoading = ref(false)

const handleClick = async () => {
  isLoading.value = true
  try {
    await someAsyncOperation()
  } finally {
    isLoading.value = false
  }
}
</script>
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

```vue
<template>
  <div>
    <!-- Basic card -->
    <ZHTPCard title="Card Title">
      <p>Card content goes here</p>
    </ZHTPCard>
    
    <!-- With actions -->
    <ZHTPCard 
      title="DApp Details"
      subtitle="Decentralized Application"
    >
      <p>This is a sample DApp description.</p>
      
      <template #actions>
        <ZHTPButton size="sm">View More</ZHTPButton>
      </template>
    </ZHTPCard>
    
    <!-- Loading state -->
    <ZHTPCard title="Loading..." :loading="true">
      <p>Content loading...</p>
    </ZHTPCard>
  </div>
</template>

<script setup>
import { ZHTPCard, ZHTPButton } from '@zhtp/vue-components'
</script>
```

**Props:**
- `title`: `string`
- `subtitle`: `string`
- `loading`: `boolean`

**Slots:**
- `default`: Main content
- `actions`: Action buttons

---

## Wallet Components

### `<WalletConnection />`
Complete wallet connection interface.

```vue
<template>
  <WalletConnection
    :providers="['metamask', 'walletconnect', 'coinbase']"
    :show-balance="true"
    :show-network="true"
    @connect="onConnect"
    @disconnect="onDisconnect"
  />
</template>

<script setup>
import { WalletConnection } from '@zhtp/vue-components'

const onConnect = (account) => {
  console.log('Connected:', account)
}

const onDisconnect = () => {
  console.log('Disconnected')
}
</script>
```

### `<WalletButton />`
Simple connect/disconnect button.

```vue
<template>
  <nav>
    <h1>My DApp</h1>
    <WalletButton />
  </nav>
</template>

<script setup>
import { WalletButton } from '@zhtp/vue-components'
</script>
```

### `<AccountInfo />`
Display connected account information.

```vue
<template>
  <div>
    <AccountInfo 
      :show-address="true"
      :show-balance="true"
      :show-network="true"
      :copyable="true"
    />
  </div>
</template>

<script setup>
import { AccountInfo } from '@zhtp/vue-components'
</script>
```

---

## Contract Components

### `<ContractCall />`
Easy contract interaction component.

```vue
<template>
  <ContractCall
    contract-address="0x1234..."
    :abi="VotingABI"
    method="vote"
    :args="[proposalId, true]"
    @success="onVoteSuccess"
    @error="onVoteError"
    v-slot="{ call, loading, error }"
  >
    <ZHTPButton 
      @click="call"
      :loading="loading"
      :disabled="!!error"
    >
      Vote Yes
    </ZHTPButton>
    <p v-if="error" class="error">{{ error.message }}</p>
  </ContractCall>
</template>

<script setup>
import { ref } from 'vue'
import { ContractCall, ZHTPButton } from '@zhtp/vue-components'
import VotingABI from './contracts/VotingABI.json'

const proposalId = ref(1)

const onVoteSuccess = (result) => {
  console.log('Vote cast!', result)
}

const onVoteError = (error) => {
  console.error('Vote failed:', error)
}
</script>
```

### `<ContractRead />`
Read data from contracts.

```vue
<template>
  <ContractRead
    contract-address="0x1234..."
    :abi="VotingABI"
    method="getProposal"
    :args="[proposalId]"
    :watch="true"
    v-slot="{ data, loading, error }"
  >
    <div v-if="loading">Loading proposal...</div>
    <div v-else-if="error" class="error">Error: {{ error.message }}</div>
    <div v-else-if="data">
      <h3>{{ data.title }}</h3>
      <p>{{ data.description }}</p>
      <p>Yes votes: {{ data.yesVotes }}</p>
      <p>No votes: {{ data.noVotes }}</p>
    </div>
  </ContractRead>
</template>

<script setup>
import { ref } from 'vue'
import { ContractRead } from '@zhtp/vue-components'
import VotingABI from './contracts/VotingABI.json'

const proposalId = ref(1)
</script>
```

---

## UI Components

### `<ZHTPInput />`
Styled input fields.

```vue
<template>
  <form @submit.prevent="handleSubmit">
    <ZHTPInput
      v-model="form.username"
      label="Username"
      placeholder="Enter username"
      :required="true"
      help-text="Must be unique"
    />
    
    <ZHTPInput
      v-model="form.email"
      type="email"
      label="Email"
      :error="emailError"
    />
    
    <ZHTPInput
      v-model="form.password"
      type="password"
      label="Password"
      :secure="true"
    />
    
    <ZHTPButton type="submit">Submit</ZHTPButton>
  </form>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ZHTPInput, ZHTPButton } from '@zhtp/vue-components'

const form = ref({
  username: '',
  email: '',
  password: ''
})

const emailError = computed(() => {
  if (form.value.email && !isValidEmail(form.value.email)) {
    return 'Invalid email format'
  }
  return null
})

const isValidEmail = (email) => {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
}

const handleSubmit = () => {
  console.log('Form submitted:', form.value)
}
</script>
```

### `<ZHTPModal />`
Modal dialogs.

```vue
<template>
  <div>
    <ZHTPButton @click="showModal = true">
      Open Modal
    </ZHTPButton>
    
    <ZHTPModal
      v-model="showModal"
      title="Confirm Transaction"
      size="md"
    >
      <p>Are you sure you want to proceed?</p>
      
      <template #actions>
        <ZHTPButton variant="ghost" @click="showModal = false">
          Cancel
        </ZHTPButton>
        <ZHTPButton variant="primary" @click="confirmAction">
          Confirm
        </ZHTPButton>
      </template>
    </ZHTPModal>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ZHTPModal, ZHTPButton } from '@zhtp/vue-components'

const showModal = ref(false)

const confirmAction = () => {
  console.log('Action confirmed')
  showModal.value = false
}
</script>
```

### `<ZHTPTabs />`
Tab navigation.

```vue
<template>
  <ZHTPTabs 
    :tabs="tabs"
    default-tab="overview"
    @change="onTabChange"
  />
</template>

<script setup>
import { ref } from 'vue'
import { ZHTPTabs } from '@zhtp/vue-components'
import OverviewPanel from './components/OverviewPanel.vue'
import TransactionsPanel from './components/TransactionsPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'

const tabs = ref([
  {
    id: 'overview',
    label: 'Overview',
    component: OverviewPanel
  },
  {
    id: 'transactions',
    label: 'Transactions',
    component: TransactionsPanel
  },
  {
    id: 'settings',
    label: 'Settings',
    component: SettingsPanel
  }
])

const onTabChange = (tabId) => {
  console.log('Tab changed:', tabId)
}
</script>
```

---

## Data Components

### `<TokenBalance />`
Display token balances.

```vue
<template>
  <div>
    <TokenBalance 
      token="ZHTP"
      :address="userAddress"
      format="short"
      :show-usd="true"
    />
    
    <TokenBalance 
      token="USDC"
      contract-address="0x5678..."
      :refresh-interval="5000"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { TokenBalance } from '@zhtp/vue-components'

const userAddress = ref('0x1234...')
</script>
```

### `<TransactionHistory />`
Display transaction history.

```vue
<template>
  <TransactionHistory
    :address="userAddress"
    :limit="10"
    :show-pagination="true"
    @transaction-click="onTransactionClick"
  />
</template>

<script setup>
import { ref } from 'vue'
import { TransactionHistory } from '@zhtp/vue-components'

const userAddress = ref('0x1234...')

const onTransactionClick = (tx) => {
  console.log('Transaction clicked:', tx)
}
</script>
```

### `<NFTGallery />`
Display NFT collections.

```vue
<template>
  <NFTGallery
    :owner="userAddress"
    collection="0x5678..."
    grid-size="md"
    :show-metadata="true"
    @nft-click="onNFTClick"
  />
</template>

<script setup>
import { ref } from 'vue'
import { NFTGallery } from '@zhtp/vue-components'

const userAddress = ref('0x1234...')

const onNFTClick = (nft) => {
  console.log('NFT clicked:', nft)
}
</script>
```

---

## Advanced Components

### `<ZKProofVerifier />`
Verify zero-knowledge proofs.

```vue
<template>
  <ZKProofVerifier
    :proof="voterProof"
    :public-signals="publicSignals"
    :verification-key="vk"
    @verified="onVerified"
    v-slot="{ isVerifying, isValid, error }"
  >
    <div v-if="isVerifying">Verifying proof...</div>
    <div v-else-if="isValid">Anonymous vote verified</div>
    <div v-else-if="error">Proof verification failed</div>
  </ZKProofVerifier>
</template>

<script setup>
import { ref } from 'vue'
import { ZKProofVerifier } from '@zhtp/vue-components'

const voterProof = ref(null)
const publicSignals = ref([])
const vk = ref(null)

const onVerified = (isValid) => {
  if (isValid) {
    console.log('Proof verified! Vote is anonymous.')
  }
}
</script>
```

### `<ConsensusStatus />`
Display network consensus information.

```vue
<template>
  <ConsensusStatus
    :show-validators="true"
    :show-proposals="true"
    :refresh-interval="10000"
  />
</template>

<script setup>
import { ConsensusStatus } from '@zhtp/vue-components'
</script>
```

---

## Composables

### `useZHTP()`
Main composable for ZHTP functionality.

```vue
<template>
  <div>
    <div v-if="isConnected">
      <p>Connected: {{ account }}</p>
      <p>Balance: {{ balance }} ZHTP</p>
      <ZHTPButton @click="disconnect">Disconnect</ZHTPButton>
    </div>
    <div v-else>
      <ZHTPButton @click="connect" :loading="isConnecting">
        {{ isConnecting ? 'Connecting...' : 'Connect Wallet' }}
      </ZHTPButton>
    </div>
  </div>
</template>

<script setup>
import { useZHTP } from '@zhtp/vue-components'

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
} = useZHTP()
</script>
```

### `useContract()`
Interact with smart contracts.

```vue
<template>
  <div>
    <ZHTPButton @click="() => vote(1, true)" :loading="loading">
      Vote Yes
    </ZHTPButton>
    <p v-if="error" class="error">{{ error.message }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useContract } from '@zhtp/vue-components'
import VotingABI from './contracts/VotingABI.json'

const loading = ref(false)
const error = ref(null)

const contract = useContract({
  address: '0x1234...',
  abi: VotingABI
})

const vote = async (proposalId, support) => {
  loading.value = true
  error.value = null
  
  try {
    const tx = await contract.vote(proposalId, support)
    await tx.wait()
    console.log('Vote successful!')
  } catch (err) {
    error.value = err
  } finally {
    loading.value = false
  }
}
</script>
```

### `useBalance()`
Get token balances.

```vue
<template>
  <div>
    <div v-if="loading">Loading balance...</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <div v-else>
      Balance: {{ balance }} ZHTP
      <ZHTPButton @click="refresh" size="sm">Refresh</ZHTPButton>
    </div>
  </div>
</template>

<script setup>
import { useBalance } from '@zhtp/vue-components'

const { 
  balance,
  loading,
  error,
  refresh
} = useBalance({
  token: 'ZHTP',
  address: '0x1234...',
  watch: true // Auto-refresh
})
</script>
```

---

## Theming

### Custom Theme
```vue
<template>
  <ZHTPProvider :theme="customTheme">
    <YourApp />
  </ZHTPProvider>
</template>

<script setup>
import { createTheme, ZHTPProvider } from '@zhtp/vue-components'

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
})
</script>
```

### Dark Mode
```vue
<template>
  <div>
    <button @click="toggleTheme">Toggle Theme</button>
    <ZHTPProvider :theme="isDark ? 'dark' : 'light'">
      <YourApp />
    </ZHTPProvider>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { ZHTPProvider } from '@zhtp/vue-components'

const isDark = ref(false)

const toggleTheme = () => {
  isDark.value = !isDark.value
}
</script>
```

---

## Vue 3 + TypeScript

### Type Support
```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useZHTP, ZHTPButton } from '@zhtp/vue-components'

interface UserData {
  address: string
  balance: string
  network: string
}

const { account, balance, network, isConnected } = useZHTP()

const userData = computed((): UserData | null => {
  if (!isConnected.value) return null
  
  return {
    address: account.value!,
    balance: balance.value,
    network: network.value
  }
})

const isLoading = ref<boolean>(false)
</script>
```

### Plugin Installation
```typescript
// main.ts
import { createApp } from 'vue'
import ZHTPComponents from '@zhtp/vue-components'
import '@zhtp/vue-components/dist/style.css'
import App from './App.vue'

const app = createApp(App)

app.use(ZHTPComponents, {
  network: 'mainnet',
  theme: 'light'
})

app.mount('#app')
```

---

## Responsive Design

All components are mobile-responsive by default:

```vue
<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    <ZHTPCard title="Mobile First">
      <p>Automatically responsive</p>
      <ZHTPButton full-width>Full Width on Mobile</ZHTPButton>
    </ZHTPCard>
  </div>
</template>
```

---

## Best Practices

### Performance
```vue
<script setup>
import { ref, computed, shallowRef } from 'vue'

// Use shallowRef for large objects
const largeData = shallowRef({})

// Use computed for derived state
const processedData = computed(() => {
  return expensiveCalculation(rawData.value)
})

// Use v-memo for expensive lists
</script>

<template>
  <div v-for="item in items" :key="item.id" v-memo="[item.id, item.updated]">
    <ExpensiveComponent :data="item" />
  </div>
</template>
```

### Error Handling
```vue
<template>
  <Suspense>
    <template #default>
      <AsyncComponent />
    </template>
    <template #fallback>
      <ZHTPSkeleton />
    </template>
  </Suspense>
</template>

<script setup>
import { defineAsyncComponent } from 'vue'

const AsyncComponent = defineAsyncComponent(() => import('./AsyncComponent.vue'))
</script>
```

---

## Installation & Setup

```bash
# Install Vue components
npm install @zhtp/vue-components

# Install peer dependencies
npm install vue@^3.0.0

# TypeScript support (optional)
npm install --save-dev typescript @vue/tsconfig
```

```javascript
// main.js
import { createApp } from 'vue'
import ZHTPComponents from '@zhtp/vue-components'
import '@zhtp/vue-components/dist/style.css'
import App from './App.vue'

const app = createApp(App)

app.use(ZHTPComponents, {
  network: 'mainnet',
  theme: 'light'
})

app.mount('#app')
```

---

## Links

- [Component Playground](https://playground.zhtp.dev/vue)
- [Design System](https://design.zhtp.dev)
- [Report Issues](https://github.com/zhtp/vue-components/issues)
- [Discord Support](https://discord.gg/zhtp)

**Build beautiful ZHTP DApps with Vue.js!**

# Whisper: Decentralized Secure Messaging

Whisper is a decentralized messaging system built on Zero-Knowledge Hidden Transit Protocol (ZHTP) with blockchain-based message history and DHT storage.

## Features

- End-to-end encrypted messaging using ZHTP
- Decentralized discovery network for finding peers
- Blockchain-based message history and verification
- Distributed Hash Table (DHT) for redundant message storage
- Network reputation and routing metrics

## Setup Instructions

### Step 1: Set Up Discovery Node

First, set up a discovery node that helps peers find each other:

```bash
cargo run --example discovery_node
```

The discovery node runs on port 9000 and needs to be accessible to all clients.

### Step 2: Run Whisper Clients

Each user runs their own Whisper client:

```bash
# Format: cargo run --example whisper -- --id <name> --port <port>
cargo run --example whisper -- --id alice --port 8001
```

### Step 3: Connect and Chat

1. Set the DISCOVERY_NODE environment variable to the discovery node's address:
   ```bash
   export DISCOVERY_NODE=1.2.3.4:9000  # Use actual IP
   ```

2. Start your client - it will automatically:
   - Connect to the discovery network
   - Register with the DHT storage network
   - Join the blockchain network for message verification

3. Send messages:
   - Choose "1" from the menu
   - Enter recipient's address (IP:PORT)
   - Type your message
   - The message will be:
     * Encrypted and sent through ZHTP
     * Stored in the DHT for redundancy
     * Recorded in blockchain for verification

4. View messages:
   - Choose "2" to see messages
   - Shows both local and network-stored messages
   - Verifies message authenticity using blockchain

## Advanced Features

- Message Storage: All messages are stored in DHT with replication
- Blockchain Verification: Messages are recorded as transactions
- Network Metrics: View connection quality and peer reputation
- Contact Management: Add and manage trusted contacts

## Troubleshooting

1. Discovery Issues:
   - Check DISCOVERY_NODE environment variable
   - Ensure discovery node port (9000) is accessible
   - Verify network connectivity

2. Message Issues:
   - Check both DHT storage status
   - Verify blockchain transaction status
   - Ensure recipient is online and connected

3. Network Issues:
   - Each client needs unique port
   - Use public IPs for remote connections
   - Configure firewalls to allow ZHTP traffic
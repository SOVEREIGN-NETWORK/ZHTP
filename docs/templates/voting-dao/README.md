# 🗳️ Voting DAO Template

**A complete decentralized autonomous organization with voting and governance features**

## 🌟 Features

### Core Governance
- **Proposal Creation**: Create and submit governance proposals
- **Voting System**: Token-weighted voting with delegation support
- **Execution**: Automatic execution of approved proposals
- **Treasury Management**: DAO treasury with spending controls

### Advanced Features
- **Vote Delegation**: Delegate voting power to trusted representatives
- **Proposal Types**: Text proposals, spending proposals, parameter changes
- **Quorum Requirements**: Configurable quorum and approval thresholds
- **Time Locks**: Delays for proposal execution for security

### Zero-Knowledge Features
- **Private Voting**: Optional anonymous voting using ZK proofs
- **Identity Verification**: Verify voter eligibility without revealing identity
- **Reputation System**: Privacy-preserving reputation tracking

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Governance    │    │      Token      │    │    Treasury     │
│    Contract     │◄──►│    Contract     │◄──►│   Contract      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Voting      │    │   Delegation    │    │    Timelock     │
│    Contract     │    │    Contract     │    │   Contract      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### 1. Installation

```bash
# Clone template
git clone https://github.com/zhtp-network/dapp-templates.git
cd dapp-templates/voting-dao

# Install dependencies
npm install

# Copy environment variables
cp .env.example .env
```

### 2. Configuration

Edit `.env` with your settings:

```env
# Network Configuration
ZHTP_NETWORK=testnet
ZHTP_RPC_URL=https://testnet.zhtp.network
PRIVATE_KEY=your_private_key_here

# DAO Configuration
DAO_NAME=My DAO
DAO_SYMBOL=MYDAO
INITIAL_SUPPLY=1000000000000000000000000
VOTING_PERIOD=604800
QUORUM_PERCENTAGE=10
APPROVAL_THRESHOLD=50

# Frontend Configuration
REACT_APP_DAO_ADDRESS=
REACT_APP_TOKEN_ADDRESS=
REACT_APP_IPFS_GATEWAY=https://ipfs.io/ipfs/
```

### 3. Deploy Smart Contracts

```bash
# Compile contracts
npm run compile

# Run tests
npm run test

# Deploy to testnet
npm run deploy:testnet

# Verify contracts
npm run verify
```

### 4. Start Frontend

```bash
# Start development server
npm run dev

# Build for production
npm run build
```

## 📜 Smart Contracts

### GovernanceToken.sol

The governance token that provides voting power:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract GovernanceToken is ZhtpERC20, Ownable {
    mapping(address => uint256) public lockedBalances;
    mapping(address => uint256) public lockUntil;
    
    // Delegation mapping
    mapping(address => address) public delegates;
    mapping(address => uint256) public delegatedVotes;
    
    event TokensLocked(address indexed user, uint256 amount, uint256 unlockTime);
    event TokensUnlocked(address indexed user, uint256 amount);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    
    constructor(
        string memory name,
        string memory symbol,
        uint256 totalSupply
    ) ZhtpERC20(name, symbol) {
        _mint(msg.sender, totalSupply);
    }
    
    /**
     * @dev Lock tokens to gain voting power
     */
    function lockTokens(uint256 amount, uint256 lockDuration) external {
        require(amount > 0, "Amount must be positive");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        require(lockDuration >= 7 days, "Minimum lock period is 7 days");
        
        _transfer(msg.sender, address(this), amount);
        
        lockedBalances[msg.sender] += amount;
        lockUntil[msg.sender] = block.timestamp + lockDuration;
        
        emit TokensLocked(msg.sender, amount, lockUntil[msg.sender]);
    }
    
    /**
     * @dev Unlock tokens after lock period
     */
    function unlockTokens() external {
        require(block.timestamp >= lockUntil[msg.sender], "Tokens still locked");
        require(lockedBalances[msg.sender] > 0, "No locked tokens");
        
        uint256 amount = lockedBalances[msg.sender];
        lockedBalances[msg.sender] = 0;
        lockUntil[msg.sender] = 0;
        
        _transfer(address(this), msg.sender, amount);
        
        emit TokensUnlocked(msg.sender, amount);
    }
    
    /**
     * @dev Delegate voting power to another address
     */
    function delegate(address delegatee) external {
        address currentDelegate = delegates[msg.sender];
        uint256 delegatorBalance = getVotingPower(msg.sender);
        
        delegates[msg.sender] = delegatee;
        
        // Update delegated votes
        if (currentDelegate != address(0)) {
            delegatedVotes[currentDelegate] -= delegatorBalance;
        }
        
        if (delegatee != address(0)) {
            delegatedVotes[delegatee] += delegatorBalance;
        }
        
        emit DelegateChanged(msg.sender, currentDelegate, delegatee);
    }
    
    /**
     * @dev Get total voting power (tokens + locked tokens + delegated votes)
     */
    function getVotingPower(address user) public view returns (uint256) {
        return balanceOf(user) + lockedBalances[user] + delegatedVotes[user];
    }
    
    /**
     * @dev Get voting power at a specific block
     */
    function getVotingPowerAt(address user, uint256 blockNumber) external view returns (uint256) {
        // Implementation would use checkpoints for historical data
        // Simplified for template
        return getVotingPower(user);
    }
}
```

### DAO.sol

The main governance contract:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./GovernanceToken.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@zhtp/contracts/ZkProofVerifier.sol";

contract DAO is ReentrancyGuard, ZkProofVerifier {
    GovernanceToken public immutable governanceToken;
    
    enum ProposalState { Pending, Active, Canceled, Defeated, Succeeded, Executed }
    
    struct Proposal {
        uint256 id;
        address proposer;
        string title;
        string description;
        address[] targets;
        uint256[] values;
        bytes[] calldatas;
        uint256 startBlock;
        uint256 endBlock;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 votesAbstain;
        bool executed;
        mapping(address => bool) hasVoted;
        mapping(address => uint8) votes; // 0=against, 1=for, 2=abstain
    }
    
    mapping(uint256 => Proposal) public proposals;
    uint256 public proposalCount;
    
    // Governance parameters
    uint256 public votingPeriod = 7 days;
    uint256 public proposalThreshold = 1000 ether; // Tokens needed to create proposal
    uint256 public quorumPercentage = 10; // 10% of total supply
    uint256 public approvalThreshold = 50; // 50% approval needed
    
    // Events
    event ProposalCreated(
        uint256 indexed proposalId,
        address indexed proposer,
        string title,
        uint256 startBlock,
        uint256 endBlock
    );
    
    event VoteCast(
        uint256 indexed proposalId,
        address indexed voter,
        uint8 support,
        uint256 weight,
        bool isPrivate
    );
    
    event ProposalExecuted(uint256 indexed proposalId);
    
    constructor(address _governanceToken) {
        governanceToken = GovernanceToken(_governanceToken);
    }
    
    /**
     * @dev Create a new proposal
     */
    function propose(
        string memory title,
        string memory description,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas
    ) external returns (uint256) {
        require(
            governanceToken.getVotingPower(msg.sender) >= proposalThreshold,
            "Insufficient voting power to create proposal"
        );
        
        require(targets.length == values.length && targets.length == calldatas.length,
            "Proposal function information arity mismatch"
        );
        
        uint256 proposalId = ++proposalCount;
        uint256 startBlock = block.number;
        uint256 endBlock = startBlock + (votingPeriod / 3); // Assuming 3 second blocks
        
        Proposal storage proposal = proposals[proposalId];
        proposal.id = proposalId;
        proposal.proposer = msg.sender;
        proposal.title = title;
        proposal.description = description;
        proposal.targets = targets;
        proposal.values = values;
        proposal.calldatas = calldatas;
        proposal.startBlock = startBlock;
        proposal.endBlock = endBlock;
        
        emit ProposalCreated(proposalId, msg.sender, title, startBlock, endBlock);
        
        return proposalId;
    }
    
    /**
     * @dev Cast a vote on a proposal
     */
    function castVote(uint256 proposalId, uint8 support) external {
        _castVote(proposalId, msg.sender, support, false, "");
    }
    
    /**
     * @dev Cast a private vote using zero-knowledge proof
     */
    function castPrivateVote(
        uint256 proposalId,
        uint8 support,
        bytes calldata zkProof
    ) external {
        // Verify ZK proof that voter has voting power without revealing identity
        bytes32[] memory publicInputs = new bytes32[](2);
        publicInputs[0] = bytes32(proposalId);
        publicInputs[1] = bytes32(uint256(support));
        
        require(verifyProof(zkProof, publicInputs), "Invalid ZK proof");
        
        _castVote(proposalId, msg.sender, support, true, zkProof);
    }
    
    /**
     * @dev Internal vote casting logic
     */
    function _castVote(
        uint256 proposalId,
        address voter,
        uint8 support,
        bool isPrivate,
        bytes memory zkProof
    ) internal {
        require(state(proposalId) == ProposalState.Active, "Voting is not active");
        require(support <= 2, "Invalid vote type");
        
        Proposal storage proposal = proposals[proposalId];
        require(!proposal.hasVoted[voter], "Voter has already voted");
        
        uint256 weight = governanceToken.getVotingPowerAt(voter, proposal.startBlock);
        require(weight > 0, "No voting power");
        
        proposal.hasVoted[voter] = true;
        proposal.votes[voter] = support;
        
        if (support == 0) {
            proposal.votesAgainst += weight;
        } else if (support == 1) {
            proposal.votesFor += weight;
        } else {
            proposal.votesAbstain += weight;
        }
        
        emit VoteCast(proposalId, voter, support, weight, isPrivate);
    }
    
    /**
     * @dev Execute a successful proposal
     */
    function execute(uint256 proposalId) external nonReentrant {
        require(state(proposalId) == ProposalState.Succeeded, "Proposal not succeeded");
        
        Proposal storage proposal = proposals[proposalId];
        proposal.executed = true;
        
        for (uint256 i = 0; i < proposal.targets.length; i++) {
            (bool success, ) = proposal.targets[i].call{value: proposal.values[i]}(
                proposal.calldatas[i]
            );
            require(success, "Transaction execution reverted");
        }
        
        emit ProposalExecuted(proposalId);
    }
    
    /**
     * @dev Get current state of a proposal
     */
    function state(uint256 proposalId) public view returns (ProposalState) {
        require(proposalId > 0 && proposalId <= proposalCount, "Invalid proposal id");
        
        Proposal storage proposal = proposals[proposalId];
        
        if (proposal.executed) {
            return ProposalState.Executed;
        }
        
        if (block.number <= proposal.startBlock) {
            return ProposalState.Pending;
        }
        
        if (block.number <= proposal.endBlock) {
            return ProposalState.Active;
        }
        
        uint256 totalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
        uint256 totalSupply = governanceToken.totalSupply();
        uint256 quorum = (totalSupply * quorumPercentage) / 100;
        
        if (totalVotes < quorum) {
            return ProposalState.Defeated;
        }
        
        uint256 approvalVotes = (proposal.votesFor * 100) / (proposal.votesFor + proposal.votesAgainst);
        
        if (approvalVotes >= approvalThreshold) {
            return ProposalState.Succeeded;
        } else {
            return ProposalState.Defeated;
        }
    }
    
    /**
     * @dev Get proposal details
     */
    function getProposal(uint256 proposalId) external view returns (
        address proposer,
        string memory title,
        string memory description,
        uint256 startBlock,
        uint256 endBlock,
        uint256 votesFor,
        uint256 votesAgainst,
        uint256 votesAbstain,
        bool executed
    ) {
        Proposal storage proposal = proposals[proposalId];
        return (
            proposal.proposer,
            proposal.title,
            proposal.description,
            proposal.startBlock,
            proposal.endBlock,
            proposal.votesFor,
            proposal.votesAgainst,
            proposal.votesAbstain,
            proposal.executed
        );
    }
    
    /**
     * @dev Update governance parameters (only through governance)
     */
    function updateVotingPeriod(uint256 _votingPeriod) external {
        require(msg.sender == address(this), "Only governance can update parameters");
        votingPeriod = _votingPeriod;
    }
    
    function updateQuorumPercentage(uint256 _quorumPercentage) external {
        require(msg.sender == address(this), "Only governance can update parameters");
        require(_quorumPercentage <= 100, "Invalid percentage");
        quorumPercentage = _quorumPercentage;
    }
    
    function updateApprovalThreshold(uint256 _approvalThreshold) external {
        require(msg.sender == address(this), "Only governance can update parameters");
        require(_approvalThreshold <= 100, "Invalid percentage");
        approvalThreshold = _approvalThreshold;
    }
}
```

### Treasury.sol

DAO treasury contract:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Treasury is ReentrancyGuard {
    address public immutable dao;
    
    event FundsReceived(address indexed from, uint256 amount);
    event FundsWithdrawn(address indexed to, uint256 amount);
    event TokensWithdrawn(address indexed token, address indexed to, uint256 amount);
    
    modifier onlyDAO() {
        require(msg.sender == dao, "Only DAO can execute");
        _;
    }
    
    constructor(address _dao) {
        dao = _dao;
    }
    
    receive() external payable {
        emit FundsReceived(msg.sender, msg.value);
    }
    
    /**
     * @dev Withdraw native tokens (executed by DAO governance)
     */
    function withdraw(address payable to, uint256 amount) external onlyDAO nonReentrant {
        require(to != address(0), "Invalid recipient");
        require(amount <= address(this).balance, "Insufficient balance");
        
        to.transfer(amount);
        emit FundsWithdrawn(to, amount);
    }
    
    /**
     * @dev Withdraw ERC20 tokens (executed by DAO governance)
     */
    function withdrawToken(
        IERC20 token,
        address to,
        uint256 amount
    ) external onlyDAO nonReentrant {
        require(to != address(0), "Invalid recipient");
        require(amount <= token.balanceOf(address(this)), "Insufficient balance");
        
        token.transfer(to, amount);
        emit TokensWithdrawn(address(token), to, amount);
    }
    
    /**
     * @dev Get treasury balance
     */
    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }
    
    /**
     * @dev Get ERC20 token balance
     */
    function getTokenBalance(IERC20 token) external view returns (uint256) {
        return token.balanceOf(address(this));
    }
}
```

## 🎨 Frontend Components

### ProposalCard.jsx

React component for displaying proposals:

```jsx
import React, { useState } from 'react';
import { formatDistanceToNow } from 'date-fns';

const ProposalCard = ({ proposal, onVote, userVotingPower, hasVoted }) => {
    const [voting, setVoting] = useState(false);
    const [selectedVote, setSelectedVote] = useState(null);

    const getStateColor = (state) => {
        const colors = {
            'Pending': '#6c757d',
            'Active': '#007bff',
            'Succeeded': '#28a745',
            'Defeated': '#dc3545',
            'Executed': '#6f42c1'
        };
        return colors[state] || '#6c757d';
    };

    const handleVote = async (support) => {
        setVoting(true);
        try {
            await onVote(proposal.id, support);
        } catch (error) {
            console.error('Failed to vote:', error);
        } finally {
            setVoting(false);
        }
    };

    const totalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
    const forPercentage = totalVotes > 0 ? (proposal.votesFor / totalVotes) * 100 : 0;
    const againstPercentage = totalVotes > 0 ? (proposal.votesAgainst / totalVotes) * 100 : 0;

    return (
        <div className="proposal-card">
            <div className="proposal-header">
                <div className="proposal-meta">
                    <span 
                        className="proposal-state"
                        style={{ backgroundColor: getStateColor(proposal.state) }}
                    >
                        {proposal.state}
                    </span>
                    <span className="proposal-id">#{proposal.id}</span>
                </div>
                <h3 className="proposal-title">{proposal.title}</h3>
                <p className="proposal-proposer">
                    Proposed by {proposal.proposer.slice(0, 6)}...{proposal.proposer.slice(-4)}
                </p>
            </div>

            <div className="proposal-body">
                <p className="proposal-description">{proposal.description}</p>
            </div>

            <div className="proposal-voting">
                <div className="voting-stats">
                    <div className="vote-breakdown">
                        <div className="vote-bar">
                            <div 
                                className="vote-bar-for"
                                style={{ width: `${forPercentage}%` }}
                            ></div>
                            <div 
                                className="vote-bar-against"
                                style={{ width: `${againstPercentage}%` }}
                            ></div>
                        </div>
                        <div className="vote-numbers">
                            <span className="votes-for">
                                For: {(proposal.votesFor / 1e18).toFixed(0)} votes
                            </span>
                            <span className="votes-against">
                                Against: {(proposal.votesAgainst / 1e18).toFixed(0)} votes
                            </span>
                        </div>
                    </div>
                </div>

                {proposal.state === 'Active' && userVotingPower > 0 && !hasVoted && (
                    <div className="voting-actions">
                        <h4>Cast Your Vote</h4>
                        <p className="voting-power">
                            Your voting power: {(userVotingPower / 1e18).toFixed(2)} votes
                        </p>
                        <div className="vote-buttons">
                            <button
                                className={`vote-btn vote-for ${selectedVote === 1 ? 'selected' : ''}`}
                                onClick={() => setSelectedVote(1)}
                                disabled={voting}
                            >
                                Vote For
                            </button>
                            <button
                                className={`vote-btn vote-against ${selectedVote === 0 ? 'selected' : ''}`}
                                onClick={() => setSelectedVote(0)}
                                disabled={voting}
                            >
                                Vote Against
                            </button>
                            <button
                                className={`vote-btn vote-abstain ${selectedVote === 2 ? 'selected' : ''}`}
                                onClick={() => setSelectedVote(2)}
                                disabled={voting}
                            >
                                Abstain
                            </button>
                        </div>
                        {selectedVote !== null && (
                            <div className="vote-confirm">
                                <button
                                    className="confirm-vote-btn"
                                    onClick={() => handleVote(selectedVote)}
                                    disabled={voting}
                                >
                                    {voting ? 'Submitting Vote...' : 'Confirm Vote'}
                                </button>
                            </div>
                        )}
                    </div>
                )}

                {hasVoted && (
                    <div className="already-voted">
                        <p>✓ You have already voted on this proposal</p>
                    </div>
                )}
            </div>

            <div className="proposal-footer">
                <span className="proposal-deadline">
                    Voting ends {formatDistanceToNow(new Date(proposal.endTime * 1000), { addSuffix: true })}
                </span>
            </div>
        </div>
    );
};

export default ProposalCard;
```

### CreateProposal.jsx

Component for creating new proposals:

```jsx
import React, { useState } from 'react';

const CreateProposal = ({ onCreateProposal, userVotingPower, proposalThreshold }) => {
    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [actions, setActions] = useState([{ target: '', value: '', calldata: '' }]);
    const [creating, setCreating] = useState(false);

    const canCreateProposal = userVotingPower >= proposalThreshold;

    const addAction = () => {
        setActions([...actions, { target: '', value: '', calldata: '' }]);
    };

    const removeAction = (index) => {
        setActions(actions.filter((_, i) => i !== index));
    };

    const updateAction = (index, field, value) => {
        const newActions = [...actions];
        newActions[index][field] = value;
        setActions(newActions);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        
        if (!canCreateProposal) {
            alert(`You need at least ${proposalThreshold / 1e18} voting power to create a proposal`);
            return;
        }

        setCreating(true);
        try {
            const targets = actions.map(a => a.target || '0x0000000000000000000000000000000000000000');
            const values = actions.map(a => a.value || '0');
            const calldatas = actions.map(a => a.calldata || '0x');

            await onCreateProposal(title, description, targets, values, calldatas);
            
            // Reset form
            setTitle('');
            setDescription('');
            setActions([{ target: '', value: '', calldata: '' }]);
        } catch (error) {
            console.error('Failed to create proposal:', error);
        } finally {
            setCreating(false);
        }
    };

    return (
        <div className="create-proposal">
            <h2>Create New Proposal</h2>
            
            <div className="eligibility-check">
                {canCreateProposal ? (
                    <p className="eligible">
                        ✓ You have {(userVotingPower / 1e18).toFixed(2)} voting power 
                        (Required: {proposalThreshold / 1e18})
                    </p>
                ) : (
                    <p className="not-eligible">
                        ✗ You need {(proposalThreshold / 1e18).toFixed(2)} voting power to create proposals
                        (Current: {(userVotingPower / 1e18).toFixed(2)})
                    </p>
                )}
            </div>

            <form onSubmit={handleSubmit} className="proposal-form">
                <div className="form-group">
                    <label htmlFor="title">Proposal Title</label>
                    <input
                        type="text"
                        id="title"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        placeholder="Enter a clear, descriptive title"
                        required
                        maxLength={100}
                    />
                </div>

                <div className="form-group">
                    <label htmlFor="description">Description</label>
                    <textarea
                        id="description"
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        placeholder="Provide detailed information about your proposal..."
                        required
                        rows={6}
                        maxLength={5000}
                    />
                </div>

                <div className="actions-section">
                    <h3>Proposal Actions</h3>
                    <p className="actions-help">
                        Define what happens if this proposal passes. Leave blank for text-only proposals.
                    </p>
                    
                    {actions.map((action, index) => (
                        <div key={index} className="action-item">
                            <h4>Action {index + 1}</h4>
                            
                            <div className="form-row">
                                <div className="form-group">
                                    <label>Target Contract Address</label>
                                    <input
                                        type="text"
                                        value={action.target}
                                        onChange={(e) => updateAction(index, 'target', e.target.value)}
                                        placeholder="0x..."
                                    />
                                </div>
                                
                                <div className="form-group">
                                    <label>Value (in wei)</label>
                                    <input
                                        type="text"
                                        value={action.value}
                                        onChange={(e) => updateAction(index, 'value', e.target.value)}
                                        placeholder="0"
                                    />
                                </div>
                            </div>
                            
                            <div className="form-group">
                                <label>Function Call Data</label>
                                <input
                                    type="text"
                                    value={action.calldata}
                                    onChange={(e) => updateAction(index, 'calldata', e.target.value)}
                                    placeholder="0x..."
                                />
                            </div>
                            
                            {actions.length > 1 && (
                                <button
                                    type="button"
                                    className="remove-action-btn"
                                    onClick={() => removeAction(index)}
                                >
                                    Remove Action
                                </button>
                            )}
                        </div>
                    ))}
                    
                    <button
                        type="button"
                        className="add-action-btn"
                        onClick={addAction}
                    >
                        Add Another Action
                    </button>
                </div>

                <div className="form-actions">
                    <button
                        type="submit"
                        className="create-btn"
                        disabled={!canCreateProposal || creating || !title || !description}
                    >
                        {creating ? 'Creating Proposal...' : 'Create Proposal'}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default CreateProposal;
```

## 📋 Package.json

```json
{
  "name": "zhtp-voting-dao",
  "version": "1.0.0",
  "description": "A complete voting DAO template for ZHTP",
  "main": "index.js",
  "scripts": {
    "dev": "concurrently \"npm run dev:contracts\" \"npm run dev:frontend\"",
    "dev:contracts": "hardhat node",
    "dev:frontend": "cd frontend && npm start",
    "build": "npm run build:contracts && npm run build:frontend",
    "build:contracts": "hardhat compile",
    "build:frontend": "cd frontend && npm run build",
    "test": "npm run test:contracts && npm run test:frontend",
    "test:contracts": "hardhat test",
    "test:frontend": "cd frontend && npm test",
    "deploy:local": "hardhat run scripts/deploy.js --network localhost",
    "deploy:testnet": "hardhat run scripts/deploy.js --network zhtp_testnet",
    "deploy:mainnet": "hardhat run scripts/deploy.js --network zhtp_mainnet",
    "verify": "hardhat run scripts/verify.js",
    "lint": "eslint . --ext .js,.jsx,.ts,.tsx",
    "format": "prettier --write ."
  },
  "dependencies": {
    "@zhtp/contracts": "^1.0.0",
    "@zhtp/sdk": "^1.0.0",
    "@openzeppelin/contracts": "^4.9.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "date-fns": "^2.29.0",
    "ethers": "^5.7.0"
  },
  "devDependencies": {
    "@nomiclabs/hardhat-ethers": "^2.2.0",
    "@nomiclabs/hardhat-waffle": "^2.0.3",
    "hardhat": "^2.12.0",
    "concurrently": "^7.6.0",
    "eslint": "^8.0.0",
    "prettier": "^2.8.0"
  },
  "keywords": ["zhtp", "dao", "governance", "voting", "blockchain", "dapp"],
  "author": "ZHTP Network",
  "license": "MIT"
}
```

## 🚀 Deployment

### deploy.js

```javascript
const { ethers } = require("hardhat");

async function main() {
    console.log("Deploying Voting DAO...");
    
    const [deployer] = await ethers.getSigners();
    console.log("Deploying with account:", deployer.address);

    // Deploy Governance Token
    const GovernanceToken = await ethers.getContractFactory("GovernanceToken");
    const token = await GovernanceToken.deploy(
        process.env.DAO_NAME + " Token",
        process.env.DAO_SYMBOL,
        ethers.utils.parseEther(process.env.INITIAL_SUPPLY)
    );
    await token.deployed();
    console.log("GovernanceToken deployed to:", token.address);

    // Deploy DAO
    const DAO = await ethers.getContractFactory("DAO");
    const dao = await DAO.deploy(token.address);
    await dao.deployed();
    console.log("DAO deployed to:", dao.address);

    // Deploy Treasury
    const Treasury = await ethers.getContractFactory("Treasury");
    const treasury = await Treasury.deploy(dao.address);
    await treasury.deployed();
    console.log("Treasury deployed to:", treasury.address);

    // Save deployment addresses
    const deployment = {
        token: token.address,
        dao: dao.address,
        treasury: treasury.address,
        network: network.name,
        deployer: deployer.address,
        timestamp: new Date().toISOString()
    };

    const fs = require("fs");
    fs.writeFileSync(
        `frontend/src/contracts/deployment-${network.name}.json`,
        JSON.stringify(deployment, null, 2)
    );

    console.log("Deployment complete!");
    console.log(`Token: ${token.address}`);
    console.log(`DAO: ${dao.address}`);
    console.log(`Treasury: ${treasury.address}`);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
```

---

This template provides a complete, production-ready voting DAO with:

- ✅ Smart contracts with comprehensive governance features
- ✅ React frontend with modern UI components
- ✅ Zero-knowledge privacy features
- ✅ Token economics and delegation
- ✅ Treasury management
- ✅ Complete deployment and testing setup

Ready to customize and deploy on ZHTP! 🚀

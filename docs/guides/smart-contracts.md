# 📜 Smart Contract Development Guide

**Complete guide to developing smart contracts on the ZHTP blockchain**

## Overview

ZHTP smart contracts are Ethereum-compatible, supporting Solidity and Vyper. They benefit from ZHTP's enhanced privacy features, zero-knowledge proof integration, and high-performance consensus mechanism.

## 🏗️ Development Environment Setup

### Required Tools

```bash
# Install Solidity compiler
npm install -g solc

# Install development framework
npm install -g hardhat
# or
npm install -g truffle

# Install ZHTP development tools
npm install -g @zhtp/cli
```

### Project Initialization

```bash
# Create new ZHTP smart contract project
zhtp init my-contract-project
cd my-contract-project

# Install dependencies
npm install @zhtp/contracts @openzeppelin/contracts
```

### Hardhat Configuration

```javascript
// hardhat.config.js
require("@zhtp/hardhat-plugin");

module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    zhtp_mainnet: {
      url: "https://mainnet.zhtp.network",
      accounts: [process.env.PRIVATE_KEY],
      chainId: 1337
    },
    zhtp_testnet: {
      url: "https://testnet.zhtp.network", 
      accounts: [process.env.PRIVATE_KEY],
      chainId: 31337
    },
    zhtp_local: {
      url: "http://localhost:8080",
      accounts: ["0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"],
      chainId: 1337
    }
  },
  zhtp: {
    apiKey: process.env.ZHTP_API_KEY,
    verify: true
  }
};
```

## 🔧 Basic Smart Contract

### Simple Storage Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpContract.sol";

contract SimpleStorage is ZhtpContract {
    uint256 private storedData;
    
    event ValueChanged(uint256 indexed oldValue, uint256 indexed newValue, address indexed changer);
    
    constructor(uint256 _initialValue) {
        storedData = _initialValue;
        emit ValueChanged(0, _initialValue, msg.sender);
    }
    
    function setValue(uint256 _value) public {
        uint256 oldValue = storedData;
        storedData = _value;
        emit ValueChanged(oldValue, _value, msg.sender);
    }
    
    function getValue() public view returns (uint256) {
        return storedData;
    }
    
    // ZHTP-specific: Use zero-knowledge proof for value verification
    function setValueWithProof(
        uint256 _value,
        bytes calldata _proof
    ) public {
        require(verifyZkProof(_proof, _value), "Invalid proof");
        setValue(_value);
    }
}
```

### ERC-20 Token with Privacy Features

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpERC20.sol";
import "@zhtp/contracts/ZkProofVerifier.sol";

contract PrivateToken is ZhtpERC20, ZkProofVerifier {
    using SafeMath for uint256;
    
    // Private balances using zero-knowledge proofs
    mapping(address => bytes32) private zkBalanceCommitments;
    mapping(address => bool) private isPrivateMode;
    
    event PrivateTransfer(
        address indexed from,
        address indexed to,
        bytes32 amountCommitment,
        bytes zkProof
    );
    
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _totalSupply
    ) ZhtpERC20(_name, _symbol) {
        _mint(msg.sender, _totalSupply);
    }
    
    // Enable privacy mode for an account
    function enablePrivacyMode(bytes32 _balanceCommitment) public {
        require(!isPrivateMode[msg.sender], "Already in private mode");
        
        uint256 balance = balanceOf(msg.sender);
        require(balance > 0, "No balance to make private");
        
        // Burn public tokens
        _burn(msg.sender, balance);
        
        // Store private balance commitment
        zkBalanceCommitments[msg.sender] = _balanceCommitment;
        isPrivateMode[msg.sender] = true;
    }
    
    // Private transfer using zero-knowledge proofs
    function privateTransfer(
        address _to,
        bytes32 _amountCommitment,
        bytes32 _newFromCommitment,
        bytes32 _newToCommitment,
        bytes calldata _proof
    ) public {
        require(isPrivateMode[msg.sender], "Sender not in private mode");
        
        // Verify the zero-knowledge proof
        bytes32[] memory publicInputs = new bytes32[](4);
        publicInputs[0] = zkBalanceCommitments[msg.sender]; // from_balance
        publicInputs[1] = zkBalanceCommitments[_to];        // to_balance
        publicInputs[2] = _newFromCommitment;               // new_from_balance
        publicInputs[3] = _newToCommitment;                 // new_to_balance
        
        require(
            verifyProof(_proof, publicInputs),
            "Invalid transfer proof"
        );
        
        // Update commitments
        zkBalanceCommitments[msg.sender] = _newFromCommitment;
        zkBalanceCommitments[_to] = _newToCommitment;
        
        if (!isPrivateMode[_to]) {
            isPrivateMode[_to] = true;
        }
        
        emit PrivateTransfer(msg.sender, _to, _amountCommitment, _proof);
    }
    
    // Get private balance commitment
    function getBalanceCommitment(address _account) public view returns (bytes32) {
        require(isPrivateMode[_account], "Account not in private mode");
        return zkBalanceCommitments[_account];
    }
}
```

## 🔐 Zero-Knowledge Integration

### ZK Proof Verification

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZkProofVerifier.sol";

contract AgeVerification is ZkProofVerifier {
    struct VerificationRecord {
        bool isVerified;
        uint256 timestamp;
        uint256 minAge;
    }
    
    mapping(address => VerificationRecord) public verifications;
    
    event AgeVerified(
        address indexed user,
        uint256 minAge,
        uint256 timestamp
    );
    
    function verifyAge(
        uint256 _minAge,
        bytes calldata _proof
    ) public {
        // Proof verifies: age >= _minAge without revealing actual age
        bytes32[] memory publicInputs = new bytes32[](2);
        publicInputs[0] = bytes32(_minAge);
        publicInputs[1] = bytes32(uint256(uint160(msg.sender)));
        
        require(
            verifyProof(_proof, publicInputs),
            "Invalid age proof"
        );
        
        verifications[msg.sender] = VerificationRecord({
            isVerified: true,
            timestamp: block.timestamp,
            minAge: _minAge
        });
        
        emit AgeVerified(msg.sender, _minAge, block.timestamp);
    }
    
    function isAgeVerified(address _user, uint256 _minAge) public view returns (bool) {
        VerificationRecord memory record = verifications[_user];
        return record.isVerified && 
               record.minAge <= _minAge &&
               block.timestamp - record.timestamp <= 86400; // 24 hours validity
    }
}
```

### Identity Verification Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZkProofVerifier.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract IdentityVerification is ZkProofVerifier, Ownable {
    struct Identity {
        bool isVerified;
        bytes32 attributeHash;
        uint256 verificationTime;
        uint256 expiryTime;
    }
    
    mapping(address => Identity) public identities;
    mapping(bytes32 => bool) public usedProofs;
    
    uint256 public constant VERIFICATION_VALIDITY = 30 days;
    
    event IdentityVerified(
        address indexed user,
        bytes32 attributeHash,
        uint256 expiryTime
    );
    
    function verifyIdentity(
        bytes32 _attributeHash,
        bytes calldata _proof
    ) public {
        // Prevent proof replay
        bytes32 proofHash = keccak256(_proof);
        require(!usedProofs[proofHash], "Proof already used");
        
        // Verify zero-knowledge proof for identity attributes
        bytes32[] memory publicInputs = new bytes32[](2);
        publicInputs[0] = _attributeHash;
        publicInputs[1] = bytes32(uint256(uint160(msg.sender)));
        
        require(
            verifyProof(_proof, publicInputs),
            "Invalid identity proof"
        );
        
        // Mark proof as used
        usedProofs[proofHash] = true;
        
        // Store verification
        uint256 expiryTime = block.timestamp + VERIFICATION_VALIDITY;
        identities[msg.sender] = Identity({
            isVerified: true,
            attributeHash: _attributeHash,
            verificationTime: block.timestamp,
            expiryTime: expiryTime
        });
        
        emit IdentityVerified(msg.sender, _attributeHash, expiryTime);
    }
    
    function isIdentityValid(address _user) public view returns (bool) {
        Identity memory identity = identities[_user];
        return identity.isVerified && block.timestamp <= identity.expiryTime;
    }
    
    function getIdentityInfo(address _user) public view returns (
        bool isVerified,
        bytes32 attributeHash,
        uint256 verificationTime,
        uint256 expiryTime
    ) {
        Identity memory identity = identities[_user];
        return (
            identity.isVerified,
            identity.attributeHash,
            identity.verificationTime,
            identity.expiryTime
        );
    }
}
```

## 🏛️ DAO and Governance

### Governance Token

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpERC20.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract GovernanceToken is ZhtpERC20, Pausable, Ownable {
    mapping(address => uint256) public lockedBalances;
    mapping(address => uint256) public lockUntil;
    
    event TokensLocked(address indexed user, uint256 amount, uint256 unlockTime);
    event TokensUnlocked(address indexed user, uint256 amount);
    
    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _totalSupply
    ) ZhtpERC20(_name, _symbol) {
        _mint(msg.sender, _totalSupply);
    }
    
    function lockTokens(uint256 _amount, uint256 _lockDuration) public {
        require(_amount > 0, "Amount must be positive");
        require(_amount <= balanceOf(msg.sender), "Insufficient balance");
        require(_lockDuration >= 7 days, "Minimum lock period is 7 days");
        
        _transfer(msg.sender, address(this), _amount);
        
        lockedBalances[msg.sender] += _amount;
        lockUntil[msg.sender] = block.timestamp + _lockDuration;
        
        emit TokensLocked(msg.sender, _amount, lockUntil[msg.sender]);
    }
    
    function unlockTokens() public {
        require(block.timestamp >= lockUntil[msg.sender], "Tokens still locked");
        require(lockedBalances[msg.sender] > 0, "No locked tokens");
        
        uint256 amount = lockedBalances[msg.sender];
        lockedBalances[msg.sender] = 0;
        lockUntil[msg.sender] = 0;
        
        _transfer(address(this), msg.sender, amount);
        
        emit TokensUnlocked(msg.sender, amount);
    }
    
    function getVotingPower(address _user) public view returns (uint256) {
        return balanceOf(_user) + lockedBalances[_user];
    }
    
    function pause() public onlyOwner {
        _pause();
    }
    
    function unpause() public onlyOwner {
        _unpause();
    }
    
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
```

### DAO Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./GovernanceToken.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract DAO is ReentrancyGuard {
    GovernanceToken public immutable governanceToken;
    
    struct Proposal {
        uint256 id;
        string title;
        string description;
        address proposer;
        uint256 startTime;
        uint256 endTime;
        uint256 votesFor;
        uint256 votesAgainst;
        bool executed;
        bytes callData;
        address target;
        uint256 value;
    }
    
    mapping(uint256 => Proposal) public proposals;
    mapping(uint256 => mapping(address => bool)) public hasVoted;
    mapping(uint256 => mapping(address => bool)) public voteChoice; // true = for, false = against
    
    uint256 public proposalCount;
    uint256 public constant VOTING_DURATION = 7 days;
    uint256 public constant PROPOSAL_THRESHOLD = 1000 ether; // Tokens needed to propose
    uint256 public constant QUORUM = 10000 ether; // Minimum votes for proposal to pass
    
    event ProposalCreated(
        uint256 indexed proposalId,
        address indexed proposer,
        string title,
        uint256 startTime,
        uint256 endTime
    );
    
    event VoteCast(
        uint256 indexed proposalId,
        address indexed voter,
        bool support,
        uint256 votingPower
    );
    
    event ProposalExecuted(uint256 indexed proposalId, bool success);
    
    constructor(address _governanceToken) {
        governanceToken = GovernanceToken(_governanceToken);
    }
    
    function createProposal(
        string memory _title,
        string memory _description,
        address _target,
        uint256 _value,
        bytes memory _callData
    ) public returns (uint256) {
        require(
            governanceToken.getVotingPower(msg.sender) >= PROPOSAL_THRESHOLD,
            "Insufficient tokens to create proposal"
        );
        
        uint256 proposalId = ++proposalCount;
        uint256 startTime = block.timestamp;
        uint256 endTime = startTime + VOTING_DURATION;
        
        proposals[proposalId] = Proposal({
            id: proposalId,
            title: _title,
            description: _description,
            proposer: msg.sender,
            startTime: startTime,
            endTime: endTime,
            votesFor: 0,
            votesAgainst: 0,
            executed: false,
            callData: _callData,
            target: _target,
            value: _value
        });
        
        emit ProposalCreated(proposalId, msg.sender, _title, startTime, endTime);
        
        return proposalId;
    }
    
    function vote(uint256 _proposalId, bool _support) public {
        Proposal storage proposal = proposals[_proposalId];
        require(proposal.id != 0, "Proposal does not exist");
        require(block.timestamp >= proposal.startTime, "Voting not started");
        require(block.timestamp <= proposal.endTime, "Voting ended");
        require(!hasVoted[_proposalId][msg.sender], "Already voted");
        
        uint256 votingPower = governanceToken.getVotingPower(msg.sender);
        require(votingPower > 0, "No voting power");
        
        hasVoted[_proposalId][msg.sender] = true;
        voteChoice[_proposalId][msg.sender] = _support;
        
        if (_support) {
            proposal.votesFor += votingPower;
        } else {
            proposal.votesAgainst += votingPower;
        }
        
        emit VoteCast(_proposalId, msg.sender, _support, votingPower);
    }
    
    function executeProposal(uint256 _proposalId) public nonReentrant {
        Proposal storage proposal = proposals[_proposalId];
        require(proposal.id != 0, "Proposal does not exist");
        require(block.timestamp > proposal.endTime, "Voting still active");
        require(!proposal.executed, "Proposal already executed");
        
        uint256 totalVotes = proposal.votesFor + proposal.votesAgainst;
        require(totalVotes >= QUORUM, "Quorum not reached");
        require(proposal.votesFor > proposal.votesAgainst, "Proposal rejected");
        
        proposal.executed = true;
        
        bool success = false;
        if (proposal.target != address(0)) {
            (success, ) = proposal.target.call{value: proposal.value}(proposal.callData);
        } else {
            success = true; // Text-only proposal
        }
        
        emit ProposalExecuted(_proposalId, success);
    }
    
    function getProposal(uint256 _proposalId) public view returns (
        string memory title,
        string memory description,
        address proposer,
        uint256 startTime,
        uint256 endTime,
        uint256 votesFor,
        uint256 votesAgainst,
        bool executed
    ) {
        Proposal memory proposal = proposals[_proposalId];
        return (
            proposal.title,
            proposal.description,
            proposal.proposer,
            proposal.startTime,
            proposal.endTime,
            proposal.votesFor,
            proposal.votesAgainst,
            proposal.executed
        );
    }
}
```

## 🎮 DeFi and Gaming Contracts

### Staking Contract

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract StakingPool is ReentrancyGuard, Ownable {
    ZhtpERC20 public immutable stakingToken;
    ZhtpERC20 public immutable rewardToken;
    
    struct StakeInfo {
        uint256 amount;
        uint256 rewardDebt;
        uint256 lastStakeTime;
    }
    
    mapping(address => StakeInfo) public stakes;
    
    uint256 public totalStaked;
    uint256 public rewardPerTokenStored;
    uint256 public lastUpdateTime;
    uint256 public rewardRate = 100; // Rewards per second
    uint256 public constant MINIMUM_STAKE_DURATION = 1 days;
    
    event Staked(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);
    event RewardPaid(address indexed user, uint256 reward);
    
    constructor(address _stakingToken, address _rewardToken) {
        stakingToken = ZhtpERC20(_stakingToken);
        rewardToken = ZhtpERC20(_rewardToken);
        lastUpdateTime = block.timestamp;
    }
    
    modifier updateReward(address _account) {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = block.timestamp;
        
        if (_account != address(0)) {
            stakes[_account].rewardDebt = earned(_account);
        }
        _;
    }
    
    function rewardPerToken() public view returns (uint256) {
        if (totalStaked == 0) {
            return rewardPerTokenStored;
        }
        
        return rewardPerTokenStored + 
               ((block.timestamp - lastUpdateTime) * rewardRate * 1e18) / totalStaked;
    }
    
    function earned(address _account) public view returns (uint256) {
        return (stakes[_account].amount * 
                (rewardPerToken() - rewardPerTokenStored)) / 1e18 + 
               stakes[_account].rewardDebt;
    }
    
    function stake(uint256 _amount) public nonReentrant updateReward(msg.sender) {
        require(_amount > 0, "Cannot stake 0");
        
        stakingToken.transferFrom(msg.sender, address(this), _amount);
        
        stakes[msg.sender].amount += _amount;
        stakes[msg.sender].lastStakeTime = block.timestamp;
        totalStaked += _amount;
        
        emit Staked(msg.sender, _amount);
    }
    
    function withdraw(uint256 _amount) public nonReentrant updateReward(msg.sender) {
        require(_amount > 0, "Cannot withdraw 0");
        require(stakes[msg.sender].amount >= _amount, "Insufficient stake");
        require(
            block.timestamp >= stakes[msg.sender].lastStakeTime + MINIMUM_STAKE_DURATION,
            "Minimum stake duration not met"
        );
        
        stakes[msg.sender].amount -= _amount;
        totalStaked -= _amount;
        
        stakingToken.transfer(msg.sender, _amount);
        
        emit Withdrawn(msg.sender, _amount);
    }
    
    function claimReward() public nonReentrant updateReward(msg.sender) {
        uint256 reward = stakes[msg.sender].rewardDebt;
        if (reward > 0) {
            stakes[msg.sender].rewardDebt = 0;
            rewardToken.transfer(msg.sender, reward);
            emit RewardPaid(msg.sender, reward);
        }
    }
    
    function exit() external {
        withdraw(stakes[msg.sender].amount);
        claimReward();
    }
    
    function setRewardRate(uint256 _rewardRate) external onlyOwner updateReward(address(0)) {
        rewardRate = _rewardRate;
    }
}
```

### NFT Marketplace

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zhtp/contracts/ZhtpERC721.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract NFTMarketplace is ReentrancyGuard {
    struct Listing {
        address seller;
        address nftContract;
        uint256 tokenId;
        uint256 price;
        bool active;
        uint256 listingTime;
    }
    
    struct Offer {
        address buyer;
        uint256 price;
        uint256 expiry;
        bool active;
    }
    
    mapping(bytes32 => Listing) public listings;
    mapping(bytes32 => mapping(address => Offer)) public offers;
    mapping(address => bool) public approvedTokens;
    
    IERC20 public immutable paymentToken;
    address public feeRecipient;
    uint256 public feePercent = 250; // 2.5%
    
    event ItemListed(
        bytes32 indexed listingId,
        address indexed seller,
        address indexed nftContract,
        uint256 tokenId,
        uint256 price
    );
    
    event ItemSold(
        bytes32 indexed listingId,
        address indexed buyer,
        address indexed seller,
        uint256 price
    );
    
    event OfferMade(
        bytes32 indexed listingId,
        address indexed buyer,
        uint256 price,
        uint256 expiry
    );
    
    constructor(address _paymentToken, address _feeRecipient) {
        paymentToken = IERC20(_paymentToken);
        feeRecipient = _feeRecipient;
    }
    
    function listItem(
        address _nftContract,
        uint256 _tokenId,
        uint256 _price
    ) public returns (bytes32) {
        require(_price > 0, "Price must be positive");
        
        ZhtpERC721 nft = ZhtpERC721(_nftContract);
        require(nft.ownerOf(_tokenId) == msg.sender, "Not token owner");
        require(
            nft.getApproved(_tokenId) == address(this) || 
            nft.isApprovedForAll(msg.sender, address(this)),
            "Marketplace not approved"
        );
        
        bytes32 listingId = keccak256(
            abi.encodePacked(_nftContract, _tokenId, msg.sender, block.timestamp)
        );
        
        listings[listingId] = Listing({
            seller: msg.sender,
            nftContract: _nftContract,
            tokenId: _tokenId,
            price: _price,
            active: true,
            listingTime: block.timestamp
        });
        
        emit ItemListed(listingId, msg.sender, _nftContract, _tokenId, _price);
        
        return listingId;
    }
    
    function buyItem(bytes32 _listingId) public nonReentrant {
        Listing storage listing = listings[_listingId];
        require(listing.active, "Listing not active");
        require(listing.seller != msg.sender, "Cannot buy own item");
        
        uint256 price = listing.price;
        uint256 fee = (price * feePercent) / 10000;
        uint256 sellerAmount = price - fee;
        
        listing.active = false;
        
        // Transfer payment
        paymentToken.transferFrom(msg.sender, listing.seller, sellerAmount);
        paymentToken.transferFrom(msg.sender, feeRecipient, fee);
        
        // Transfer NFT
        ZhtpERC721(listing.nftContract).transferFrom(
            listing.seller,
            msg.sender,
            listing.tokenId
        );
        
        emit ItemSold(_listingId, msg.sender, listing.seller, price);
    }
    
    function makeOffer(
        bytes32 _listingId,
        uint256 _price,
        uint256 _duration
    ) public {
        require(_price > 0, "Price must be positive");
        require(_duration > 0, "Duration must be positive");
        
        Listing memory listing = listings[_listingId];
        require(listing.active, "Listing not active");
        require(listing.seller != msg.sender, "Cannot offer on own item");
        
        // Cancel previous offer if exists
        if (offers[_listingId][msg.sender].active) {
            offers[_listingId][msg.sender].active = false;
        }
        
        offers[_listingId][msg.sender] = Offer({
            buyer: msg.sender,
            price: _price,
            expiry: block.timestamp + _duration,
            active: true
        });
        
        emit OfferMade(_listingId, msg.sender, _price, block.timestamp + _duration);
    }
    
    function acceptOffer(bytes32 _listingId, address _buyer) public nonReentrant {
        Listing storage listing = listings[_listingId];
        require(listing.active, "Listing not active");
        require(listing.seller == msg.sender, "Not listing owner");
        
        Offer storage offer = offers[_listingId][_buyer];
        require(offer.active, "Offer not active");
        require(block.timestamp <= offer.expiry, "Offer expired");
        
        uint256 price = offer.price;
        uint256 fee = (price * feePercent) / 10000;
        uint256 sellerAmount = price - fee;
        
        listing.active = false;
        offer.active = false;
        
        // Transfer payment
        paymentToken.transferFrom(_buyer, msg.sender, sellerAmount);
        paymentToken.transferFrom(_buyer, feeRecipient, fee);
        
        // Transfer NFT
        ZhtpERC721(listing.nftContract).transferFrom(
            msg.sender,
            _buyer,
            listing.tokenId
        );
        
        emit ItemSold(_listingId, _buyer, msg.sender, price);
    }
    
    function cancelListing(bytes32 _listingId) public {
        Listing storage listing = listings[_listingId];
        require(listing.seller == msg.sender, "Not listing owner");
        require(listing.active, "Listing not active");
        
        listing.active = false;
    }
    
    function setFeePercent(uint256 _feePercent) public {
        require(msg.sender == feeRecipient, "Not authorized");
        require(_feePercent <= 1000, "Fee too high"); // Max 10%
        feePercent = _feePercent;
    }
}
```

## 🧪 Testing Contracts

### Test Setup

```javascript
// test/SimpleStorage.test.js
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SimpleStorage", function () {
  let SimpleStorage;
  let simpleStorage;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    SimpleStorage = await ethers.getContractFactory("SimpleStorage");
    [owner, addr1, addr2] = await ethers.getSigners();
    
    simpleStorage = await SimpleStorage.deploy(42);
    await simpleStorage.deployed();
  });

  describe("Deployment", function () {
    it("Should set the right initial value", async function () {
      expect(await simpleStorage.getValue()).to.equal(42);
    });

    it("Should emit ValueChanged event on deployment", async function () {
      // Check event was emitted during deployment
      const deployTx = simpleStorage.deployTransaction;
      const receipt = await deployTx.wait();
      
      const event = receipt.events.find(e => e.event === "ValueChanged");
      expect(event.args.oldValue).to.equal(0);
      expect(event.args.newValue).to.equal(42);
      expect(event.args.changer).to.equal(owner.address);
    });
  });

  describe("setValue", function () {
    it("Should update the stored value", async function () {
      await simpleStorage.setValue(100);
      expect(await simpleStorage.getValue()).to.equal(100);
    });

    it("Should emit ValueChanged event", async function () {
      await expect(simpleStorage.setValue(100))
        .to.emit(simpleStorage, "ValueChanged")
        .withArgs(42, 100, owner.address);
    });

    it("Should work with different addresses", async function () {
      await simpleStorage.connect(addr1).setValue(200);
      expect(await simpleStorage.getValue()).to.equal(200);
    });
  });

  describe("Zero-Knowledge Proof Integration", function () {
    it("Should accept valid proof", async function () {
      // Mock proof data - in real tests, generate actual proofs
      const mockProof = "0x1234567890abcdef";
      
      // This would require actual ZK proof generation in real tests
      await expect(simpleStorage.setValueWithProof(150, mockProof))
        .to.not.be.reverted;
    });
  });
});
```

### Integration Tests

```javascript
// test/NFTMarketplace.integration.test.js
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("NFT Marketplace Integration", function () {
  let marketplace;
  let nft;
  let token;
  let owner, seller, buyer, feeRecipient;

  beforeEach(async function () {
    [owner, seller, buyer, feeRecipient] = await ethers.getSigners();

    // Deploy ERC20 token for payments
    const Token = await ethers.getContractFactory("ZhtpERC20");
    token = await Token.deploy("TestToken", "TEST");
    await token.deployed();

    // Deploy NFT contract
    const NFT = await ethers.getContractFactory("ZhtpERC721");
    nft = await NFT.deploy("TestNFT", "TNFT");
    await nft.deployed();

    // Deploy marketplace
    const Marketplace = await ethers.getContractFactory("NFTMarketplace");
    marketplace = await Marketplace.deploy(token.address, feeRecipient.address);
    await marketplace.deployed();

    // Setup: mint tokens and NFT
    await token.mint(buyer.address, ethers.utils.parseEther("1000"));
    await nft.mint(seller.address, 1);
    
    // Approve marketplace
    await nft.connect(seller).approve(marketplace.address, 1);
    await token.connect(buyer).approve(marketplace.address, ethers.utils.parseEther("1000"));
  });

  it("Should complete full listing and purchase flow", async function () {
    const price = ethers.utils.parseEther("100");
    
    // List item
    const tx = await marketplace.connect(seller).listItem(nft.address, 1, price);
    const receipt = await tx.wait();
    const event = receipt.events.find(e => e.event === "ItemListed");
    const listingId = event.args.listingId;

    // Buy item
    await marketplace.connect(buyer).buyItem(listingId);

    // Check NFT ownership
    expect(await nft.ownerOf(1)).to.equal(buyer.address);

    // Check payment
    const fee = price.mul(250).div(10000); // 2.5%
    const sellerAmount = price.sub(fee);
    
    expect(await token.balanceOf(seller.address)).to.equal(sellerAmount);
    expect(await token.balanceOf(feeRecipient.address)).to.equal(fee);
  });

  it("Should handle offer and acceptance flow", async function () {
    const listingPrice = ethers.utils.parseEther("100");
    const offerPrice = ethers.utils.parseEther("80");
    
    // List item
    const tx = await marketplace.connect(seller).listItem(nft.address, 1, listingPrice);
    const receipt = await tx.wait();
    const event = receipt.events.find(e => e.event === "ItemListed");
    const listingId = event.args.listingId;

    // Make offer
    await marketplace.connect(buyer).makeOffer(listingId, offerPrice, 86400); // 1 day

    // Accept offer
    await marketplace.connect(seller).acceptOffer(listingId, buyer.address);

    // Check NFT ownership
    expect(await nft.ownerOf(1)).to.equal(buyer.address);

    // Check payment based on offer price
    const fee = offerPrice.mul(250).div(10000);
    const sellerAmount = offerPrice.sub(fee);
    
    expect(await token.balanceOf(seller.address)).to.equal(sellerAmount);
  });
});
```

## 🚀 Deployment Scripts

### Deploy Script

```javascript
// scripts/deploy.js
const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", (await deployer.getBalance()).toString());

  // Deploy GovernanceToken
  const GovernanceToken = await ethers.getContractFactory("GovernanceToken");
  const governanceToken = await GovernanceToken.deploy(
    "ZHTP Governance",
    "ZHTPGOV",
    ethers.utils.parseEther("1000000") // 1M tokens
  );
  await governanceToken.deployed();
  console.log("GovernanceToken deployed to:", governanceToken.address);

  // Deploy DAO
  const DAO = await ethers.getContractFactory("DAO");
  const dao = await DAO.deploy(governanceToken.address);
  await dao.deployed();
  console.log("DAO deployed to:", dao.address);

  // Deploy StakingPool
  const StakingPool = await ethers.getContractFactory("StakingPool");
  const stakingPool = await StakingPool.deploy(
    governanceToken.address, // staking token
    governanceToken.address  // reward token (same for simplicity)
  );
  await stakingPool.deployed();
  console.log("StakingPool deployed to:", stakingPool.address);

  // Deploy NFT
  const NFT = await ethers.getContractFactory("ZhtpERC721");
  const nft = await NFT.deploy("ZHTP NFT", "ZNFT");
  await nft.deployed();
  console.log("NFT deployed to:", nft.address);

  // Deploy NFTMarketplace
  const NFTMarketplace = await ethers.getContractFactory("NFTMarketplace");
  const marketplace = await NFTMarketplace.deploy(
    governanceToken.address, // payment token
    deployer.address         // fee recipient
  );
  await marketplace.deployed();
  console.log("NFTMarketplace deployed to:", marketplace.address);

  // Verify contracts on ZHTP explorer
  if (network.name !== "hardhat" && network.name !== "localhost") {
    console.log("Waiting for block confirmations...");
    await governanceToken.deployTransaction.wait(5);
    
    await hre.run("verify:verify", {
      address: governanceToken.address,
      constructorArguments: [
        "ZHTP Governance",
        "ZHTPGOV",
        ethers.utils.parseEther("1000000")
      ],
    });

    await hre.run("verify:verify", {
      address: dao.address,
      constructorArguments: [governanceToken.address],
    });

    // Verify other contracts...
  }

  // Save deployment addresses
  const deployments = {
    governanceToken: governanceToken.address,
    dao: dao.address,
    stakingPool: stakingPool.address,
    nft: nft.address,
    marketplace: marketplace.address,
    network: network.name,
    deployer: deployer.address,
    timestamp: new Date().toISOString()
  };

  const fs = require("fs");
  fs.writeFileSync(
    `deployments/${network.name}.json`,
    JSON.stringify(deployments, null, 2)
  );

  console.log("Deployment completed!");
  console.log("Deployment info saved to:", `deployments/${network.name}.json`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### Upgrade Script

```javascript
// scripts/upgrade.js
const { ethers, upgrades } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Upgrading contracts with account:", deployer.address);

  // Load existing deployment
  const fs = require("fs");
  const deployments = JSON.parse(fs.readFileSync(`deployments/${network.name}.json`));

  // Upgrade StakingPool (assuming it's upgradeable)
  const StakingPoolV2 = await ethers.getContractFactory("StakingPoolV2");
  const upgraded = await upgrades.upgradeProxy(deployments.stakingPool, StakingPoolV2);
  await upgraded.deployed();

  console.log("StakingPool upgraded to V2");
  
  // Update deployment record
  deployments.stakingPoolV2 = upgraded.address;
  deployments.lastUpgrade = new Date().toISOString();
  
  fs.writeFileSync(
    `deployments/${network.name}.json`,
    JSON.stringify(deployments, null, 2)
  );
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

## 🔧 Best Practices

### Gas Optimization

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract GasOptimized {
    // Pack structs to save storage slots
    struct User {
        uint128 balance;     // 16 bytes
        uint64 lastActivity; // 8 bytes
        uint32 level;        // 4 bytes
        bool isActive;       // 1 byte
        // Total: 29 bytes = 1 storage slot (32 bytes)
    }
    
    // Use mappings instead of arrays for lookups
    mapping(address => User) public users;
    
    // Cache storage reads
    function updateUser(address _user, uint128 _newBalance) external {
        User storage user = users[_user]; // Single storage read
        user.balance = _newBalance;
        user.lastActivity = uint64(block.timestamp);
        // Multiple writes to same storage slot are optimized
    }
    
    // Use unchecked for safe arithmetic
    function safeLoop(uint256 _count) external pure returns (uint256) {
        uint256 sum = 0;
        for (uint256 i = 0; i < _count;) {
            sum += i;
            unchecked { ++i; } // Save gas on overflow checks
        }
        return sum;
    }
    
    // Use custom errors instead of require strings
    error InsufficientBalance(uint256 requested, uint256 available);
    error NotAuthorized(address caller);
    
    function transfer(address _to, uint256 _amount) external {
        User storage sender = users[msg.sender];
        if (sender.balance < _amount) {
            revert InsufficientBalance(_amount, sender.balance);
        }
        
        sender.balance -= uint128(_amount);
        users[_to].balance += uint128(_amount);
    }
}
```

### Security Patterns

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SecureContract is ReentrancyGuard, Pausable, Ownable {
    mapping(address => uint256) private balances;
    
    // Use pull over push pattern
    mapping(address => uint256) public pendingWithdrawals;
    
    event Deposit(address indexed user, uint256 amount);
    event WithdrawalRequested(address indexed user, uint256 amount);
    event WithdrawalCompleted(address indexed user, uint256 amount);
    
    // Check-Effects-Interactions pattern
    function deposit() external payable whenNotPaused {
        require(msg.value > 0, "Must deposit something");
        
        // Effects
        balances[msg.sender] += msg.value;
        
        emit Deposit(msg.sender, msg.value);
    }
    
    // Two-step withdrawal for safety
    function requestWithdrawal(uint256 _amount) external whenNotPaused {
        require(balances[msg.sender] >= _amount, "Insufficient balance");
        
        // Effects
        balances[msg.sender] -= _amount;
        pendingWithdrawals[msg.sender] += _amount;
        
        emit WithdrawalRequested(msg.sender, _amount);
    }
    
    function completeWithdrawal() external nonReentrant whenNotPaused {
        uint256 amount = pendingWithdrawals[msg.sender];
        require(amount > 0, "No pending withdrawal");
        
        // Effects
        pendingWithdrawals[msg.sender] = 0;
        
        // Interactions
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        
        emit WithdrawalCompleted(msg.sender, amount);
    }
    
    // Emergency functions
    function pause() external onlyOwner {
        _pause();
    }
    
    function unpause() external onlyOwner {
        _unpause();
    }
    
    function emergencyWithdraw() external onlyOwner {
        require(paused(), "Contract must be paused");
        payable(owner()).transfer(address(this).balance);
    }
}
```

---

## 📚 Next Steps

- **[DApp Development Guide](dapp-development.md)** - Building complete DApps
- **[Zero-Knowledge Guide](zero-knowledge.md)** - Advanced ZK integration
- **[Security Audit Checklist](security-audit.md)** - Contract security best practices
- **[DApp Templates](../templates/)** - Ready-to-use smart contract templates
- **[API Documentation](../api/)** - SDK references for all languages

For more resources and community support, visit our **[Developer Portal](../README.md)**.

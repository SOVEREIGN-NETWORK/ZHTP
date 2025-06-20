"""
ZHTP Client - Main interface for decentralized internet operations

Complete replacement for HTTP clients (requests, urllib, etc.)
with quantum-resistant, privacy-preserving, decentralized alternatives.
"""

import asyncio
import json
import logging
from typing import Dict, Any, Optional, Union, List, Tuple
from dataclasses import dataclass, field

from .exceptions import ZhtpError, NetworkError, SecurityError
from .dns import BlockchainDNS
from .certificates import ZkCertificateAuthority
from .routing import AnonymousRouting
from .storage import DecentralizedStorage
from .contracts import SmartContracts
from .crypto import ZkCrypto
from .network import NetworkManager

logger = logging.getLogger(__name__)

@dataclass
class ZhtpConfig:
    """Configuration for ZHTP client"""
    network: str = "mainnet"  # mainnet, testnet, localnet
    privacy_level: str = "high"  # maximum, high, medium, low
    anonymous_routing: bool = True
    quantum_resistant: bool = True
    bootstrap_nodes: List[str] = field(default_factory=list)
    encryption_level: str = "aes256"
    consensus_algorithm: str = "zk_consensus"
    storage_nodes: int = 5
    routing_hops: int = 3
    enable_dht: bool = True
    enable_tor_like_routing: bool = True
    certificate_validation: bool = True
    smart_contract_execution: bool = True
    
    def __post_init__(self):
        """Set default bootstrap nodes if none provided"""
        if not self.bootstrap_nodes:
            if self.network == "mainnet":
                self.bootstrap_nodes = [
                    "zhtp://node1.zhtp.network:8333",
                    "zhtp://node2.zhtp.network:8333",
                    "zhtp://node3.zhtp.network:8333"
                ]
            elif self.network == "testnet":
                self.bootstrap_nodes = [
                    "zhtp://testnet1.zhtp.network:18333",
                    "zhtp://testnet2.zhtp.network:18333"
                ]
            else:  # localnet
                self.bootstrap_nodes = [
                    "zhtp://localhost:8333"
                ]

class ZhtpClient:
    """
    Main ZHTP client - Complete replacement for HTTP clients
    
    Provides quantum-resistant, privacy-preserving, decentralized
    alternatives to all traditional internet operations.
    
    Example:
        client = ZhtpClient()
        await client.connect()
        content = await client.fetch_content("news.zhtp", "/latest")
    """
    
    def __init__(self, config: Optional[ZhtpConfig] = None):
        """Initialize ZHTP client with configuration"""
        self.config = config or ZhtpConfig()
        self.connected = False
        self.session_id = None        
        # Initialize core components
        self.dns = BlockchainDNS(self)
        self.certificates = ZkCertificateAuthority(self)
        self.routing = AnonymousRouting(self)
        self.storage = DecentralizedStorage(self)
        self.contracts = SmartContracts(self)
        self.crypto = ZkCrypto(self)
        self.network = NetworkManager(self)
        
        # Connection state
        self._peer_connections = {}
        self._active_routes = {}
        
        logger.info(f"ZHTP Client initialized - Network: {self.config.network}")
    
    async def connect(self) -> bool:
        """
        Connect to ZHTP network - Replaces HTTP session establishment
        
        Returns:
            bool: True if connection successful
            
        Raises:
            NetworkError: If connection fails
        """
        try:
            logger.info("Connecting to ZHTP decentralized network...")
            
            # Initialize cryptographic components
            await self.crypto.initialize()
            
            # Establish anonymous routing
            if self.config.anonymous_routing:
                await self.routing.establish_anonymous_circuit()
            
            # Initialize network manager
            await self.network.initialize()
            
            # Connect to blockchain DNS
            await self.dns.connect()
            
            # Initialize certificate authority
            await self.certificates.initialize()
            
            # Connect to storage network
            await self.storage.connect()
            
            self.connected = True
            self.session_id = await self.crypto.generate_session_id()
            
            logger.info(f"Connected to ZHTP network - Session: {self.session_id[:8]}...")
            return True
            
        except Exception as e:
            logger.error(f"Failed to connect to ZHTP network: {e}")
            raise NetworkError(f"Connection failed: {e}")
    
    async def disconnect(self):
        """Disconnect from ZHTP network"""
        logger.info("Disconnecting from ZHTP network...")
        
        if self.routing:
            await self.routing.close_circuits()
        if self.storage:
            await self.storage.disconnect()
        if self.dns:
            await self.dns.disconnect()
        if self.network:
            await self.network.shutdown()
            
        self.connected = False
        self.session_id = None
        
        logger.info("Disconnected from ZHTP network")
    
    async def fetch_content(
        self, 
        domain: str, 
        path: str = "/", 
        anonymous: bool = True,
        verify_integrity: bool = True
    ) -> bytes:
        """
        Fetch content from decentralized web - Replaces HTTP GET
        
        Args:
            domain: ZHTP domain (e.g., "news.zhtp")
            path: Content path
            anonymous: Use anonymous routing
            verify_integrity: Verify content integrity
            
        Returns:
            bytes: Content data
            
        Raises:
            NetworkError: If fetch fails
        """
        if not self.connected:
            await self.connect()
        
        try:
            # Resolve domain via blockchain DNS
            domain_info = await self.dns.resolve(domain)
            
            # Fetch content via decentralized storage
            content = await self.storage.fetch_content(
                content_hash=domain_info.content_hash,
                path=path,
                anonymous=anonymous
            )
            
            # Verify integrity
            if verify_integrity:
                await self._verify_content_integrity(content, domain_info)
            
            logger.info(f"Fetched content from {domain}{path}")
            return content
            
        except Exception as e:
            logger.error(f"Failed to fetch content from {domain}{path}: {e}")
            raise NetworkError(f"Content fetch failed: {e}")
    
    async def fetch_json(
        self, 
        domain: str, 
        path: str = "/",
        anonymous: bool = True
    ) -> Dict[str, Any]:
        """
        Fetch JSON data - Replaces HTTP GET with JSON response
        
        Args:
            domain: ZHTP domain
            path: API path
            anonymous: Use anonymous routing
            
        Returns:
            Dict: Parsed JSON data
        """
        content = await self.fetch_content(domain, path, anonymous)
        return json.loads(content.decode('utf-8'))
    
    async def fetch_text(
        self, 
        domain: str, 
        path: str = "/",
        encoding: str = "utf-8",
        anonymous: bool = True
    ) -> str:
        """
        Fetch text content - Replaces HTTP GET with text response
        
        Args:
            domain: ZHTP domain
            path: Content path
            encoding: Text encoding
            anonymous: Use anonymous routing
            
        Returns:
            str: Text content
        """
        content = await self.fetch_content(domain, path, anonymous)
        return content.decode(encoding)
    
    async def publish_content(
        self,
        domain: str,
        path: str,
        content: Union[str, bytes, Dict[str, Any]],
        anonymous: bool = True
    ) -> str:
        """
        Publish content to decentralized web - Replaces HTTP POST/PUT
        
        Args:
            domain: ZHTP domain to publish to
            path: Content path
            content: Content to publish
            anonymous: Use anonymous routing
            
        Returns:
            str: Content hash
        """
        if not self.connected:
            await self.connect()
        
        # Convert content to bytes
        if isinstance(content, str):
            content_bytes = content.encode('utf-8')
        elif isinstance(content, dict):
            content_bytes = json.dumps(content).encode('utf-8')
        else:
            content_bytes = content
        
        # Store content via decentralized storage
        content_hash = await self.storage.store_content(
            content=content_bytes,
            path=path,
            anonymous=anonymous
        )
        
        # Update domain DNS record
        await self.dns.update_domain(domain, content_hash, path)
        
        logger.info(f"Published content to {domain}{path} - Hash: {content_hash[:16]}...")
        return content_hash
    
    async def call_contract(
        self,
        contract_address: str,
        method: str,
        params: Optional[Dict[str, Any]] = None,
        anonymous: bool = True
    ) -> Any:
        """
        Call smart contract method - Replaces HTTP API calls
        
        Args:
            contract_address: Contract address
            method: Method name
            params: Method parameters
            anonymous: Use anonymous routing
            
        Returns:
            Any: Contract method result
        """
        return await self.contracts.call_method(
            contract_address, method, params, anonymous
        )
    
    async def deploy_dapp(
        self,
        name: str,
        domain: str,
        source_code: Union[str, bytes],
        language: str = "javascript",
        resources: Optional[Dict[str, bytes]] = None
    ) -> str:
        """
        Deploy decentralized application - Replaces traditional hosting
        
        Args:
            name: DApp name
            domain: ZHTP domain
            source_code: Application source code
            language: Programming language
            resources: Additional resources (CSS, images, etc.)
            
        Returns:
            str: Deployment hash
        """
        return await self.contracts.deploy_dapp(
            name=name,
            domain=domain,
            source_code=source_code,
            language=language,
            resources=resources
        )
    
    async def _verify_content_integrity(self, content: bytes, domain_info) -> bool:
        """Verify content integrity using cryptographic proofs"""
        expected_hash = domain_info.content_hash
        actual_hash = await self.crypto.hash_content(content)
        
        if expected_hash != actual_hash:
            raise SecurityError("Content integrity verification failed")
        
        return True
    
    async def __aenter__(self):
        """Async context manager entry"""
        await self.connect()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit"""
        await self.disconnect()

# Convenience functions for quick access
async def fetch_content(domain: str, path: str = "/") -> bytes:
    """Quick content fetch without client management"""
    async with ZhtpClient() as client:
        return await client.fetch_content(domain, path)

async def fetch_json(domain: str, path: str = "/") -> Dict[str, Any]:
    """Quick JSON fetch without client management"""
    async with ZhtpClient() as client:
        return await client.fetch_json(domain, path)

"""
ZHTP Python SDK - Complete Decentralized Internet Replacement

This SDK provides a complete replacement for traditional HTTP, DNS, and SSL/TLS
infrastructure with quantum-resistant, privacy-preserving, decentralized alternatives.

Key Components:
- ZhtpClient: Main client for all ZHTP operations
- BlockchainDNS: Decentralized domain name resolution  
- ZkCertificateAuthority: Quantum-resistant certificate management
- AnonymousRouting: Privacy-preserving communication routing
- DecentralizedStorage: Distributed content storage and retrieval
- SmartContracts: Programmable network rules and DApp deployment

Usage:
    from zhtp import ZhtpClient, BlockchainDNS
    
    client = ZhtpClient()
    await client.connect()
    content = await client.fetch_content("example.zhtp", "/path")
"""

__version__ = "1.0.0"
__author__ = "ZHTP Network"
__license__ = "MIT"

# Core client exports
from .client import ZhtpClient, ZhtpConfig
from .dns import BlockchainDNS
from .certificates import ZkCertificateAuthority
from .routing import AnonymousRouting
from .storage import DecentralizedStorage
from .contracts import SmartContracts
from .dapps import DAppManager
from .network import NetworkManager
from .crypto import ZkCrypto
from .exceptions import (
    ZhtpError,
    NetworkError,
    SecurityError,
    ValidationError,
    RoutingError,
    StorageError,
    ContractError
)

# Convenience imports for quick access
from .client import ZhtpClient as Client
from .dns import resolve_domain
from .storage import store_content, fetch_content
from .contracts import deploy_contract, call_contract

__all__ = [
    # Core classes    "ZhtpClient",
    "ZhtpConfig",
    "Client", 
    "BlockchainDNS",
    "ZkCertificateAuthority", 
    "AnonymousRouting",
    "DecentralizedStorage",
    "SmartContracts",
    "DAppManager",
    "NetworkManager",
    "ZkCrypto",
    
    # Convenience functions
    "resolve_domain",
    "store_content", 
    "fetch_content",
    "deploy_contract",
    "call_contract",
    
    # Exceptions
    "ZhtpError",
    "NetworkError",
    "SecurityError", 
    "ValidationError",
    "RoutingError",
    "StorageError",
    "ContractError",
    
    # Metadata
    "__version__",
    "__author__",
    "__license__"
]

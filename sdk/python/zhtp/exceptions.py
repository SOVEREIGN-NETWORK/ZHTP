"""
ZHTP Exception Classes - Specialized error handling for decentralized operations
"""

class ZhtpError(Exception):
    """Base exception for all ZHTP operations"""
    pass

class NetworkError(ZhtpError):
    """Raised when network operations fail"""
    pass

class SecurityError(ZhtpError):
    """Raised when security validation fails"""
    pass

class ValidationError(ZhtpError):
    """Raised when data validation fails"""
    pass

class RoutingError(ZhtpError):
    """Raised when anonymous routing fails"""
    pass

class StorageError(ZhtpError):
    """Raised when decentralized storage operations fail"""
    pass

class ContractError(ZhtpError):
    """Raised when smart contract operations fail"""
    pass

class DnsError(ZhtpError):
    """Raised when blockchain DNS operations fail"""
    pass

class CertificateError(ZhtpError):
    """Raised when certificate operations fail"""
    pass

class ZkProofError(ZhtpError):
    """Raised when zero-knowledge proof operations fail"""
    pass

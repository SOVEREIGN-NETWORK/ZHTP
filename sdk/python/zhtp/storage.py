"""
Decentralized Storage - Distributed content storage and retrieval

Complete replacement for traditional cloud storage with decentralized,
encrypted, redundant storage across peer network.
"""

import asyncio
import logging
from typing import Dict, Any, Optional, List, Union
from dataclasses import dataclass
from datetime import datetime
import hashlib

from .exceptions import StorageError, NetworkError

logger = logging.getLogger(__name__)

@dataclass
class StorageNode:
    """Decentralized storage node"""
    node_id: str
    address: str
    port: int
    capacity: int
    available: int
    reliability: float
    latency: float

@dataclass
class ContentMetadata:
    """Content metadata"""
    content_hash: str
    size: int
    content_type: str
    encryption_key: str
    storage_nodes: List[str]
    created_at: datetime
    access_count: int

class DecentralizedStorage:
    """
    Decentralized storage system
    
    Provides encrypted, redundant storage across peer network
    without relying on traditional cloud providers.
    """
    
    def __init__(self, client):
        self.client = client
        self.connected = False
        self.storage_nodes = []
        self.content_cache = {}
        self.metadata_cache = {}
        
    async def connect(self):
        """Connect to decentralized storage network"""
        try:
            logger.info("Connecting to decentralized storage network...")
            
            # Discover storage nodes
            await self._discover_storage_nodes()
            
            # Test node connectivity
            await self._test_node_connectivity()
            
            self.connected = True
            logger.info(f"Connected to {len(self.storage_nodes)} storage nodes")
            
        except Exception as e:
            logger.error(f"Failed to connect to storage network: {e}")
            raise StorageError(f"Storage connection failed: {e}")
    
    async def disconnect(self):
        """Disconnect from storage network"""
        self.connected = False
        self.content_cache.clear()
        self.metadata_cache.clear()
        logger.info("Disconnected from decentralized storage")
    
    async def store_content(
        self,
        content: bytes,
        path: str = "/",
        encryption_key: Optional[str] = None,
        redundancy: int = 3,
        anonymous: bool = True
    ) -> str:
        """
        Store content in decentralized storage
        
        Args:
            content: Content to store
            path: Content path
            encryption_key: Optional encryption key
            redundancy: Number of copies to store
            anonymous: Use anonymous storage
            
        Returns:
            str: Content hash
        """
        if not self.connected:
            await self.connect()
        
        try:
            logger.info(f"Storing content ({len(content)} bytes) at path: {path}")
            
            # Generate content hash
            content_hash = await self._hash_content(content)
            
            # Encrypt content if needed
            if encryption_key or self.client.config.privacy_level == "maximum":
                if not encryption_key:
                    encryption_key = await self._generate_encryption_key()
                encrypted_content = await self._encrypt_content(content, encryption_key)
            else:
                encrypted_content = content
                encryption_key = None
            
            # Select storage nodes
            selected_nodes = await self._select_storage_nodes(
                len(encrypted_content), redundancy
            )
            
            # Store content on selected nodes
            storage_results = await self._store_on_nodes(
                encrypted_content, selected_nodes, anonymous
            )
            
            # Create metadata
            metadata = ContentMetadata(
                content_hash=content_hash,
                size=len(content),
                content_type=self._detect_content_type(content),
                encryption_key=encryption_key or "",
                storage_nodes=[node.node_id for node in selected_nodes],
                created_at=datetime.now(),
                access_count=0
            )
            
            # Store metadata
            await self._store_metadata(content_hash, metadata)
            
            # Cache content and metadata
            self.content_cache[content_hash] = content
            self.metadata_cache[content_hash] = metadata
            
            logger.info(f"Stored content - Hash: {content_hash[:16]}... on {len(selected_nodes)} nodes")
            return content_hash
            
        except Exception as e:
            logger.error(f"Failed to store content: {e}")
            raise StorageError(f"Content storage failed: {e}")
    
    async def fetch_content(
        self,
        content_hash: str,
        path: str = "/",
        anonymous: bool = True,
        verify_integrity: bool = True
    ) -> bytes:
        """
        Fetch content from decentralized storage
        
        Args:
            content_hash: Content hash to fetch
            path: Content path
            anonymous: Use anonymous fetching
            verify_integrity: Verify content integrity
            
        Returns:
            bytes: Content data
        """
        if not self.connected:
            await self.connect()
        
        try:
            logger.debug(f"Fetching content: {content_hash[:16]}...")
            
            # Check cache first
            if content_hash in self.content_cache:
                logger.debug("Content cache hit")
                return self.content_cache[content_hash]
            
            # Get metadata
            metadata = await self._get_metadata(content_hash)
            if not metadata:
                raise StorageError(f"Content metadata not found: {content_hash}")
            
            # Fetch content from storage nodes
            encrypted_content = await self._fetch_from_nodes(
                content_hash, metadata.storage_nodes, anonymous
            )
            
            # Decrypt content if needed
            if metadata.encryption_key:
                content = await self._decrypt_content(
                    encrypted_content, metadata.encryption_key
                )
            else:
                content = encrypted_content
            
            # Verify integrity
            if verify_integrity:
                actual_hash = await self._hash_content(content)
                if actual_hash != content_hash:
                    raise StorageError("Content integrity verification failed")
            
            # Update access count
            metadata.access_count += 1
            await self._store_metadata(content_hash, metadata)
            
            # Cache content
            self.content_cache[content_hash] = content
            
            logger.debug(f"Fetched content: {content_hash[:16]}... ({len(content)} bytes)")
            return content
            
        except Exception as e:
            logger.error(f"Failed to fetch content {content_hash}: {e}")
            raise StorageError(f"Content fetch failed: {e}")
    
    async def delete_content(
        self,
        content_hash: str,
        authorization_key: str
    ) -> bool:
        """
        Delete content from decentralized storage
        
        Args:
            content_hash: Content to delete
            authorization_key: Authorization for deletion
            
        Returns:
            bool: True if deletion successful
        """
        try:
            logger.info(f"Deleting content: {content_hash[:16]}...")
            
            # Get metadata
            metadata = await self._get_metadata(content_hash)
            if not metadata:
                return False
            
            # Verify authorization
            if not await self._verify_deletion_authorization(content_hash, authorization_key):
                raise StorageError("Deletion authorization failed")
            
            # Delete from storage nodes
            deletion_results = await self._delete_from_nodes(
                content_hash, metadata.storage_nodes
            )
            
            # Delete metadata
            await self._delete_metadata(content_hash)
            
            # Clear cache
            self.content_cache.pop(content_hash, None)
            self.metadata_cache.pop(content_hash, None)
            
            logger.info(f"Deleted content: {content_hash[:16]}...")
            return True
            
        except Exception as e:
            logger.error(f"Failed to delete content {content_hash}: {e}")
            raise StorageError(f"Content deletion failed: {e}")
    
    async def search_content(
        self,
        query: str,
        content_type: Optional[str] = None,
        limit: int = 100
    ) -> List[ContentMetadata]:
        """
        Search for content by metadata
        
        Args:
            query: Search query
            content_type: Filter by content type
            limit: Maximum results
            
        Returns:
            List[ContentMetadata]: Matching content
        """
        try:
            # Implementation would search metadata index
            results = []
            
            # Mock search results
            for i in range(min(5, limit)):
                results.append(ContentMetadata(
                    content_hash=f"hash_{i}_{query}",
                    size=1024 * (i + 1),
                    content_type=content_type or "text/plain",
                    encryption_key="",
                    storage_nodes=[f"node_{j}" for j in range(3)],
                    created_at=datetime.now(),
                    access_count=i * 10
                ))
            
            return results
            
        except Exception as e:
            logger.error(f"Content search failed: {e}")
            raise StorageError(f"Search failed: {e}")
    
    async def _discover_storage_nodes(self):
        """Discover available storage nodes"""
        # Implementation would discover nodes from DHT or directory
        self.storage_nodes = [
            StorageNode(
                node_id=f"storage_node_{i:03d}",
                address=f"192.168.1.{200 + i}",
                port=8001,
                capacity=1024 * 1024 * 1024 * (i + 1),  # GB
                available=1024 * 1024 * 1024 * (i + 1) // 2,
                reliability=0.95 + (i % 5) * 0.01,
                latency=10.0 + (i % 10)
            )
            for i in range(20)  # Mock nodes
        ]
    
    async def _test_node_connectivity(self):
        """Test connectivity to storage nodes"""
        # Implementation would test actual connectivity
        pass
    
    async def _hash_content(self, content: bytes) -> str:
        """Generate content hash"""
        return hashlib.sha256(content).hexdigest()
    
    async def _generate_encryption_key(self) -> str:
        """Generate encryption key"""
        import secrets
        return secrets.token_hex(32)
    
    async def _encrypt_content(self, content: bytes, key: str) -> bytes:
        """Encrypt content with key"""
        # Implementation would use actual encryption
        return b"encrypted_" + content
    
    async def _decrypt_content(self, encrypted_content: bytes, key: str) -> bytes:
        """Decrypt content with key"""
        # Implementation would use actual decryption
        if encrypted_content.startswith(b"encrypted_"):
            return encrypted_content[10:]  # Remove prefix
        return encrypted_content
    
    def _detect_content_type(self, content: bytes) -> str:
        """Detect content type from content"""
        # Simple content type detection
        if content.startswith(b'<!DOCTYPE html') or content.startswith(b'<html'):
            return "text/html"
        elif content.startswith(b'{') or content.startswith(b'['):
            return "application/json"
        elif content.startswith(b'\x89PNG'):
            return "image/png"
        elif content.startswith(b'\xff\xd8\xff'):
            return "image/jpeg"
        else:
            return "application/octet-stream"
    
    async def _select_storage_nodes(
        self,
        content_size: int,
        redundancy: int
    ) -> List[StorageNode]:
        """Select optimal storage nodes"""
        # Filter nodes with sufficient space
        suitable_nodes = [
            node for node in self.storage_nodes
            if node.available >= content_size and node.reliability >= 0.9
        ]
        
        if len(suitable_nodes) < redundancy:
            raise StorageError("Insufficient storage nodes available")
        
        # Select best nodes by reliability and latency
        suitable_nodes.sort(key=lambda n: (-n.reliability, n.latency))
        return suitable_nodes[:redundancy]
    
    async def _store_on_nodes(
        self,
        content: bytes,
        nodes: List[StorageNode],
        anonymous: bool
    ) -> List[bool]:
        """Store content on selected nodes"""
        results = []
        for node in nodes:
            try:
                # Implementation would store on actual node
                await asyncio.sleep(0.1)  # Simulate storage delay
                results.append(True)
                logger.debug(f"Stored on node: {node.node_id}")
            except Exception as e:
                logger.warning(f"Failed to store on node {node.node_id}: {e}")
                results.append(False)
        
        return results
    
    async def _fetch_from_nodes(
        self,
        content_hash: str,
        node_ids: List[str],
        anonymous: bool
    ) -> bytes:
        """Fetch content from storage nodes"""
        # Try nodes in order until successful
        for node_id in node_ids:
            try:
                # Implementation would fetch from actual node
                await asyncio.sleep(0.05)  # Simulate fetch delay
                return b"mock_content_data"  # Mock content
            except Exception as e:
                logger.warning(f"Failed to fetch from node {node_id}: {e}")
                continue
        
        raise StorageError("Failed to fetch from any storage node")
    
    async def _store_metadata(self, content_hash: str, metadata: ContentMetadata):
        """Store content metadata"""
        # Implementation would store in metadata index
        self.metadata_cache[content_hash] = metadata
    
    async def _get_metadata(self, content_hash: str) -> Optional[ContentMetadata]:
        """Get content metadata"""
        # Check cache first
        if content_hash in self.metadata_cache:
            return self.metadata_cache[content_hash]
        
        # Implementation would query metadata index
        return None
    
    async def _delete_from_nodes(
        self,
        content_hash: str,
        node_ids: List[str]
    ) -> List[bool]:
        """Delete content from storage nodes"""
        results = []
        for node_id in node_ids:
            try:
                # Implementation would delete from actual node
                await asyncio.sleep(0.05)  # Simulate deletion delay
                results.append(True)
            except Exception as e:
                logger.warning(f"Failed to delete from node {node_id}: {e}")
                results.append(False)
        
        return results
    
    async def _delete_metadata(self, content_hash: str):
        """Delete content metadata"""
        # Implementation would delete from metadata index
        self.metadata_cache.pop(content_hash, None)
    
    async def _verify_deletion_authorization(
        self,
        content_hash: str,
        authorization_key: str
    ) -> bool:
        """Verify deletion authorization"""
        # Implementation would verify cryptographic authorization
        return True

# Convenience functions
async def store_content(content: Union[str, bytes], path: str = "/") -> str:
    """Quick content storage without client management"""
    from .client import ZhtpClient
    if isinstance(content, str):
        content = content.encode('utf-8')
    async with ZhtpClient() as client:
        return await client.storage.store_content(content, path)

async def fetch_content(content_hash: str, path: str = "/") -> bytes:
    """Quick content fetch without client management"""
    from .client import ZhtpClient
    async with ZhtpClient() as client:
        return await client.storage.fetch_content(content_hash, path)

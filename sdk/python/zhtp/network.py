"""
Network Manager - Core networking and peer management

Manages connections to the ZHTP decentralized network including
peer discovery, connection management, and network health monitoring.
"""

import asyncio
import logging
from typing import Dict, Any, Optional, List, Set
from dataclasses import dataclass
from datetime import datetime
from enum import Enum

from .exceptions import NetworkError

logger = logging.getLogger(__name__)

class NodeType(Enum):
    """ZHTP node types"""
    FULL_NODE = "full_node"
    LIGHT_NODE = "light_node"
    STORAGE_NODE = "storage_node"
    ROUTING_NODE = "routing_node"
    DNS_NODE = "dns_node"

class ConnectionStatus(Enum):
    """Connection status"""
    CONNECTING = "connecting"
    CONNECTED = "connected"
    DISCONNECTING = "disconnecting"
    DISCONNECTED = "disconnected"
    FAILED = "failed"

@dataclass
class NetworkPeer:
    """Network peer information"""
    peer_id: str
    address: str
    port: int
    node_type: NodeType
    version: str
    last_seen: datetime
    latency: float
    reliability: float
    capabilities: List[str]

@dataclass
class NetworkStats:
    """Network statistics"""
    total_peers: int
    connected_peers: int
    bandwidth_in: int
    bandwidth_out: int
    messages_sent: int
    messages_received: int
    uptime: float

class NetworkManager:
    """
    Network Manager for ZHTP protocol
    
    Handles all aspects of network connectivity including peer discovery,
    connection management, and network health monitoring.
    """
    
    def __init__(self, client):
        self.client = client
        self.connected = False
        self.peers = {}
        self.connections = {}
        self.network_stats = NetworkStats(0, 0, 0, 0, 0, 0, 0.0)
        self.start_time = None
        
    async def initialize(self):
        """Initialize the network manager"""
        logger.info("Initializing network manager...")
        self.start_time = datetime.now()
        # Any additional initialization can go here
        logger.info("Network manager initialized")
        
    async def shutdown(self):
        """Shutdown the network manager and close all connections"""
        logger.info("Shutting down network manager...")
        
        # Close all peer connections
        for peer_id, connection in self.connections.items():
            try:
                await connection.close()
            except Exception as e:
                logger.warning(f"Error closing connection to {peer_id}: {e}")
        
        self.connections.clear()
        self.peers.clear()
        self.connected = False
        logger.info("Network manager shutdown complete")
    
    async def connect_to_network(
        self,
        bootstrap_nodes: Optional[List[str]] = None,
        max_peers: int = 50
    ) -> bool:
        """
        Connect to ZHTP network
        
        Args:
            bootstrap_nodes: List of bootstrap node addresses
            max_peers: Maximum number of peer connections
            
        Returns:
            bool: True if connection successful
        """
        try:
            logger.info("Connecting to ZHTP network...")
            self.start_time = datetime.now()
            
            # Use default bootstrap nodes if none provided
            if not bootstrap_nodes:
                bootstrap_nodes = await self._get_default_bootstrap_nodes()
            
            # Connect to bootstrap nodes
            connected_count = 0
            for node_address in bootstrap_nodes:
                try:
                    if await self._connect_to_peer(node_address):
                        connected_count += 1
                        if connected_count >= 3:  # Minimum connections
                            break
                except Exception as e:
                    logger.warning(f"Failed to connect to bootstrap node {node_address}: {e}")
            
            if connected_count == 0:
                raise NetworkError("Failed to connect to any bootstrap nodes")
            
            # Start peer discovery
            await self._start_peer_discovery()
            
            # Start network maintenance tasks
            await self._start_network_maintenance()
            
            self.connected = True
            logger.info(f"Connected to ZHTP network with {connected_count} initial peers")
            return True
            
        except Exception as e:
            logger.error(f"Failed to connect to network: {e}")
            raise NetworkError(f"Network connection failed: {e}")
    
    async def disconnect_from_network(self):
        """Disconnect from ZHTP network"""
        try:
            logger.info("Disconnecting from ZHTP network...")
            
            # Close all peer connections
            disconnect_tasks = []
            for peer_id in list(self.connections.keys()):
                disconnect_tasks.append(self._disconnect_from_peer(peer_id))
            
            if disconnect_tasks:
                await asyncio.gather(*disconnect_tasks, return_exceptions=True)
            
            self.connected = False
            self.peers.clear()
            self.connections.clear()
            
            logger.info("Disconnected from ZHTP network")
            
        except Exception as e:
            logger.error(f"Error during network disconnection: {e}")
    
    async def send_message(
        self,
        peer_id: str,
        message_type: str,
        data: Dict[str, Any],
        timeout: float = 30.0
    ) -> Optional[Dict[str, Any]]:
        """
        Send message to peer
        
        Args:
            peer_id: Target peer ID
            message_type: Type of message
            data: Message data
            timeout: Response timeout
            
        Returns:
            Dict: Response data or None
        """
        try:
            if peer_id not in self.connections:
                raise NetworkError(f"Not connected to peer: {peer_id}")
            
            logger.debug(f"Sending {message_type} message to {peer_id[:16]}...")
            
            # Prepare message
            message = {
                "type": message_type,
                "data": data,
                "timestamp": datetime.now().isoformat(),
                "sender": await self._get_our_peer_id()
            }
            
            # Send message
            response = await self._send_message_to_peer(peer_id, message, timeout)
            
            # Update statistics
            self.network_stats.messages_sent += 1
            
            logger.debug(f"Message sent to {peer_id[:16]}...")
            return response
            
        except Exception as e:
            logger.error(f"Failed to send message to {peer_id}: {e}")
            return None
    
    async def broadcast_message(
        self,
        message_type: str,
        data: Dict[str, Any],
        node_types: Optional[List[NodeType]] = None
    ) -> List[str]:
        """
        Broadcast message to multiple peers
        
        Args:
            message_type: Type of message
            data: Message data
            node_types: Filter by node types (optional)
            
        Returns:
            List[str]: List of peer IDs that received the message
        """
        try:
            logger.debug(f"Broadcasting {message_type} message...")
            
            # Select target peers
            target_peers = []
            for peer_id, peer in self.peers.items():
                if peer_id in self.connections:
                    if not node_types or peer.node_type in node_types:
                        target_peers.append(peer_id)
            
            # Send to all target peers
            send_tasks = []
            for peer_id in target_peers:
                send_tasks.append(self.send_message(peer_id, message_type, data))
            
            # Wait for all sends to complete
            results = await asyncio.gather(*send_tasks, return_exceptions=True)
            
            # Count successful sends
            successful_peers = []
            for i, result in enumerate(results):
                if not isinstance(result, Exception):
                    successful_peers.append(target_peers[i])
            
            logger.debug(f"Broadcast completed: {len(successful_peers)}/{len(target_peers)} peers")
            return successful_peers
            
        except Exception as e:
            logger.error(f"Broadcast failed: {e}")
            return []
    
    async def discover_peers(
        self,
        node_type: Optional[NodeType] = None,
        max_peers: int = 10
    ) -> List[NetworkPeer]:
        """
        Discover new peers on the network
        
        Args:
            node_type: Filter by node type
            max_peers: Maximum peers to discover
            
        Returns:
            List[NetworkPeer]: Discovered peers
        """
        try:
            logger.debug(f"Discovering peers (type: {node_type}, max: {max_peers})...")
            
            # Query connected peers for peer lists
            discovered_peers = []
            
            for peer_id in self.connections:
                try:
                    response = await self.send_message(
                        peer_id,
                        "peer_discovery_request",
                        {"node_type": node_type.value if node_type else None}
                    )
                    
                    if response and "peers" in response:
                        for peer_data in response["peers"]:
                            peer = await self._parse_peer_data(peer_data)
                            if peer and peer.peer_id not in self.peers:
                                discovered_peers.append(peer)
                                
                                if len(discovered_peers) >= max_peers:
                                    break
                    
                    if len(discovered_peers) >= max_peers:
                        break
                        
                except Exception as e:
                    logger.warning(f"Peer discovery failed for {peer_id}: {e}")
            
            logger.debug(f"Discovered {len(discovered_peers)} new peers")
            return discovered_peers
            
        except Exception as e:
            logger.error(f"Peer discovery failed: {e}")
            return []
    
    async def get_network_stats(self) -> NetworkStats:
        """
        Get current network statistics
        
        Returns:
            NetworkStats: Current network statistics
        """
        # Update uptime
        if self.start_time:
            uptime = (datetime.now() - self.start_time).total_seconds()
            self.network_stats.uptime = uptime
        
        # Update peer counts
        self.network_stats.total_peers = len(self.peers)
        self.network_stats.connected_peers = len(self.connections)
        
        return self.network_stats
    
    async def get_peer_info(self, peer_id: str) -> Optional[NetworkPeer]:
        """
        Get information about a specific peer
        
        Args:
            peer_id: Peer ID
            
        Returns:
            NetworkPeer: Peer information or None
        """
        return self.peers.get(peer_id)
    
    async def get_connected_peers(self) -> List[NetworkPeer]:
        """
        Get list of currently connected peers
        
        Returns:
            List[NetworkPeer]: Connected peers
        """
        return [self.peers[peer_id] for peer_id in self.connections if peer_id in self.peers]
    
    async def _get_default_bootstrap_nodes(self) -> List[str]:
        """Get default bootstrap node addresses"""
        # In production, these would be well-known bootstrap nodes
        return [
            "bootstrap1.zhtp.network:9001",
            "bootstrap2.zhtp.network:9001",
            "bootstrap3.zhtp.network:9001"
        ]
    
    async def _connect_to_peer(self, node_address: str) -> bool:
        """Connect to a specific peer"""
        try:
            # Parse address
            host, port = node_address.split(':')
            port = int(port)
            
            # Establish connection
            # Implementation would create actual network connection
            peer_id = f"peer_{hash(node_address):x}"
            
            # Create peer info
            peer = NetworkPeer(
                peer_id=peer_id,
                address=host,
                port=port,
                node_type=NodeType.FULL_NODE,
                version="1.0.0",
                last_seen=datetime.now(),
                latency=50.0,  # Mock latency
                reliability=0.95,
                capabilities=["storage", "routing", "dns"]
            )
            
            # Store peer and connection
            self.peers[peer_id] = peer
            self.connections[peer_id] = {
                "status": ConnectionStatus.CONNECTED,
                "connected_at": datetime.now()
            }
            
            logger.debug(f"Connected to peer: {peer_id[:16]}...")
            return True
            
        except Exception as e:
            logger.warning(f"Failed to connect to {node_address}: {e}")
            return False
    
    async def _disconnect_from_peer(self, peer_id: str):
        """Disconnect from a specific peer"""
        try:
            if peer_id in self.connections:
                # Close connection
                # Implementation would close actual network connection
                del self.connections[peer_id]
                logger.debug(f"Disconnected from peer: {peer_id[:16]}...")
                
        except Exception as e:
            logger.warning(f"Error disconnecting from {peer_id}: {e}")
    
    async def _start_peer_discovery(self):
        """Start periodic peer discovery"""
        asyncio.create_task(self._peer_discovery_loop())
    
    async def _start_network_maintenance(self):
        """Start network maintenance tasks"""
        asyncio.create_task(self._maintenance_loop())
    
    async def _peer_discovery_loop(self):
        """Periodic peer discovery loop"""
        while self.connected:
            try:
                await asyncio.sleep(60)  # Run every minute
                if len(self.connections) < 10:  # Maintain minimum connections
                    await self.discover_peers(max_peers=5)
            except Exception as e:
                logger.error(f"Peer discovery loop error: {e}")
    
    async def _maintenance_loop(self):
        """Network maintenance loop"""
        while self.connected:
            try:
                await asyncio.sleep(30)  # Run every 30 seconds
                
                # Check peer health
                dead_peers = []
                for peer_id, peer in self.peers.items():
                    if peer_id in self.connections:
                        # Ping peer
                        try:
                            response = await self.send_message(peer_id, "ping", {})
                            if response:
                                peer.last_seen = datetime.now()
                            else:
                                dead_peers.append(peer_id)
                        except Exception:
                            dead_peers.append(peer_id)
                
                # Remove dead peers
                for peer_id in dead_peers:
                    await self._disconnect_from_peer(peer_id)
                    if peer_id in self.peers:
                        del self.peers[peer_id]
                
            except Exception as e:
                logger.error(f"Maintenance loop error: {e}")
    
    async def _send_message_to_peer(
        self,
        peer_id: str,
        message: Dict[str, Any],
        timeout: float
    ) -> Optional[Dict[str, Any]]:
        """Send message to peer and wait for response"""
        # Implementation would send actual message
        await asyncio.sleep(0.1)  # Simulate network delay
        
        # Mock response
        return {
            "status": "success",
            "echo": message["type"],
            "timestamp": datetime.now().isoformat()
        }
    
    async def _get_our_peer_id(self) -> str:
        """Get our peer ID"""
        # Implementation would return actual peer ID
        return f"our_peer_{id(self):x}"
    
    async def _parse_peer_data(self, peer_data: Dict[str, Any]) -> Optional[NetworkPeer]:
        """Parse peer data from network message"""
        try:
            return NetworkPeer(
                peer_id=peer_data["peer_id"],
                address=peer_data["address"],
                port=peer_data["port"],
                node_type=NodeType(peer_data["node_type"]),
                version=peer_data["version"],
                last_seen=datetime.now(),
                latency=peer_data.get("latency", 100.0),
                reliability=peer_data.get("reliability", 0.9),
                capabilities=peer_data.get("capabilities", [])
            )
        except Exception as e:
            logger.warning(f"Failed to parse peer data: {e}")
            return None

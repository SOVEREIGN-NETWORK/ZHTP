"""
DApp Manager - Decentralized application management and deployment

Provides tools for managing, deploying, and interacting with
decentralized applications on the ZHTP network.
"""

import asyncio
import json
import logging
from typing import Dict, Any, Optional, List
from dataclasses import dataclass
from datetime import datetime
from enum import Enum

from .exceptions import ContractError, StorageError

logger = logging.getLogger(__name__)

class DAppStatus(Enum):
    """DApp status enumeration"""
    DEPLOYING = "deploying"
    ACTIVE = "active"
    PAUSED = "paused"
    UPDATING = "updating"
    TERMINATED = "terminated"

@dataclass
class DAppInfo:
    """Decentralized application information"""
    name: str
    domain: str
    contract_address: str
    app_hash: str
    version: str
    status: DAppStatus
    owner: str
    deployed_at: datetime
    resource_hashes: Dict[str, str]
    metadata: Dict[str, Any]

class DAppManager:
    """
    Decentralized Application Manager
    
    Provides comprehensive tools for DApp lifecycle management
    including deployment, updates, monitoring, and analytics.
    """
    
    def __init__(self, client):
        self.client = client
        self.deployed_dapps = {}
        
    async def create_dapp(
        self,
        name: str,
        domain: str,
        template: str = "basic",
        language: str = "javascript"
    ) -> str:
        """
        Create new DApp from template
        
        Args:
            name: DApp name
            domain: ZHTP domain
            template: DApp template (basic, marketplace, social, etc.)
            language: Programming language
            
        Returns:
            str: DApp creation hash
        """
        try:
            logger.info(f"Creating DApp: {name}")
            
            # Generate DApp source from template
            source_code = await self._generate_from_template(template, language, name)
            
            # Create initial resources
            resources = await self._create_initial_resources(template, name)
            
            # Deploy DApp
            deployment_hash = await self.client.contracts.deploy_dapp(
                name=name,
                domain=domain,
                source_code=source_code,
                language=language,
                resources=resources
            )
            
            logger.info(f"Created DApp {name} - Hash: {deployment_hash[:16]}...")
            return deployment_hash
            
        except Exception as e:
            logger.error(f"Failed to create DApp {name}: {e}")
            raise ContractError(f"DApp creation failed: {e}")
    
    async def deploy_existing_dapp(
        self,
        name: str,
        domain: str,
        source_path: str,
        language: str = "javascript",
        config: Optional[Dict[str, Any]] = None
    ) -> str:
        """
        Deploy existing DApp from source
        
        Args:
            name: DApp name
            domain: ZHTP domain
            source_path: Path to source code
            language: Programming language
            config: Optional configuration
            
        Returns:
            str: Deployment hash
        """
        try:
            logger.info(f"Deploying existing DApp: {name}")
            
            # Read source code
            source_code = await self._read_source_code(source_path)
            
            # Read resources
            resources = await self._read_resources(source_path)
            
            # Apply configuration
            if config:
                source_code = await self._apply_config(source_code, config)
            
            # Deploy DApp
            deployment_hash = await self.client.contracts.deploy_dapp(
                name=name,
                domain=domain,
                source_code=source_code,
                language=language,
                resources=resources
            )
            
            logger.info(f"Deployed DApp {name} - Hash: {deployment_hash[:16]}...")
            return deployment_hash
            
        except Exception as e:
            logger.error(f"Failed to deploy DApp {name}: {e}")
            raise ContractError(f"DApp deployment failed: {e}")
    
    async def update_dapp(
        self,
        domain: str,
        new_version: str,
        source_code: Optional[str] = None,
        resources: Optional[Dict[str, bytes]] = None
    ) -> str:
        """
        Update existing DApp
        
        Args:
            domain: DApp domain
            new_version: New version number
            source_code: Updated source code
            resources: Updated resources
            
        Returns:
            str: Update transaction hash
        """
        try:
            logger.info(f"Updating DApp: {domain} to version {new_version}")
            
            # Get current DApp info
            dapp_info = await self.get_dapp_info(domain)
            if not dapp_info:
                raise ContractError(f"DApp not found: {domain}")
            
            # Prepare update data
            update_data = {"version": new_version}
            
            if source_code:
                # Store new source code
                app_hash = await self.client.storage.store_content(
                    source_code.encode('utf-8'), "/"
                )
                update_data["app_hash"] = app_hash
                
                # Update DNS
                await self.client.dns.update_domain(domain, app_hash)
            
            if resources:
                # Store new resources
                resource_hashes = {}
                for path, data in resources.items():
                    resource_hash = await self.client.storage.store_content(data, path)
                    resource_hashes[path] = resource_hash
                update_data["resource_hashes"] = resource_hashes
            
            # Update contract
            tx_hash = await self.client.contracts.call_method(
                dapp_info.contract_address,
                "update",
                update_data
            )
            
            logger.info(f"Updated DApp {domain} - TX: {tx_hash[:16]}...")
            return tx_hash
            
        except Exception as e:
            logger.error(f"Failed to update DApp {domain}: {e}")
            raise ContractError(f"DApp update failed: {e}")
    
    async def get_dapp_info(self, domain: str) -> Optional[DAppInfo]:
        """
        Get DApp information
        
        Args:
            domain: DApp domain
            
        Returns:
            DAppInfo: DApp information or None
        """
        try:
            # Resolve domain to get contract
            domain_record = await self.client.dns.resolve(domain)
            
            # Get contract info
            # Implementation would query contract for DApp info
            # For now, return mock data
            return DAppInfo(
                name="Example DApp",
                domain=domain,
                contract_address="0xabcd1234...",
                app_hash=domain_record.content_hash,
                version="1.0.0",
                status=DAppStatus.ACTIVE,
                owner="0x1234...",
                deployed_at=datetime.now(),
                resource_hashes={},
                metadata={}
            )
            
        except Exception as e:
            logger.error(f"Failed to get DApp info for {domain}: {e}")
            return None
    
    async def pause_dapp(self, domain: str) -> bool:
        """
        Pause DApp execution
        
        Args:
            domain: DApp domain
            
        Returns:
            bool: True if paused successfully
        """
        try:
            dapp_info = await self.get_dapp_info(domain)
            if not dapp_info:
                return False
            
            # Pause contract
            await self.client.contracts.call_method(
                dapp_info.contract_address,
                "pause"
            )
            
            logger.info(f"Paused DApp: {domain}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to pause DApp {domain}: {e}")
            return False
    
    async def resume_dapp(self, domain: str) -> bool:
        """
        Resume paused DApp
        
        Args:
            domain: DApp domain
            
        Returns:
            bool: True if resumed successfully
        """
        try:
            dapp_info = await self.get_dapp_info(domain)
            if not dapp_info:
                return False
            
            # Resume contract
            await self.client.contracts.call_method(
                dapp_info.contract_address,
                "resume"
            )
            
            logger.info(f"Resumed DApp: {domain}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to resume DApp {domain}: {e}")
            return False
    
    async def get_dapp_analytics(self, domain: str) -> Dict[str, Any]:
        """
        Get DApp usage analytics
        
        Args:
            domain: DApp domain
            
        Returns:
            Dict: Analytics data
        """
        try:
            # Implementation would gather actual analytics
            return {
                "domain": domain,
                "total_users": 1234,
                "daily_active_users": 156,
                "total_transactions": 5678,
                "data_transferred": "12.5 GB",
                "uptime": "99.8%",
                "last_updated": datetime.now().isoformat()
            }
            
        except Exception as e:
            logger.error(f"Failed to get analytics for {domain}: {e}")
            return {}
    
    async def list_dapps(self, owner: Optional[str] = None) -> List[DAppInfo]:
        """
        List deployed DApps
        
        Args:
            owner: Filter by owner (optional)
            
        Returns:
            List[DAppInfo]: List of DApps
        """
        try:
            # Implementation would query blockchain for DApps
            # For now, return mock data
            mock_dapps = [
                DAppInfo(
                    name=f"DApp {i}",
                    domain=f"dapp{i}.zhtp",
                    contract_address=f"0xdapp{i}",
                    app_hash=f"hash{i}",
                    version="1.0.0",
                    status=DAppStatus.ACTIVE,
                    owner=owner or "0x1234",
                    deployed_at=datetime.now(),
                    resource_hashes={},
                    metadata={}
                )
                for i in range(3)
            ]
            
            return mock_dapps
            
        except Exception as e:
            logger.error(f"Failed to list DApps: {e}")
            return []
    
    async def _generate_from_template(
        self,
        template: str,
        language: str,
        name: str
    ) -> str:
        """Generate DApp source code from template"""
        templates = {
            "basic": {
                "javascript": f"""
// Basic ZHTP DApp - {name}
class {name.replace(' ', '')}DApp {{
    constructor() {{
        this.name = "{name}";
        this.version = "1.0.0";
    }}
    
    async init() {{
        console.log(`Initializing ${{this.name}} DApp`);
        await this.setupUI();
    }}
    
    async setupUI() {{
        document.body.innerHTML = `
            <h1>{name}</h1>
            <p>Welcome to your decentralized application!</p>
            <button onclick="app.handleClick()">Click Me</button>
        `;
    }}
    
    handleClick() {{
        alert("Hello from ZHTP DApp!");
    }}
}}

const app = new {name.replace(' ', '')}DApp();
app.init();
"""
            },
            "marketplace": {
                "javascript": f"""
// Marketplace DApp - {name}
class {name.replace(' ', '')}Marketplace {{
    constructor() {{
        this.name = "{name}";
        this.products = [];
    }}
    
    async init() {{
        await this.loadProducts();
        await this.setupUI();
    }}
    
    async loadProducts() {{
        // Load products from ZHTP storage
        this.products = [
            {{id: 1, name: "Product 1", price: 10}},
            {{id: 2, name: "Product 2", price: 20}}
        ];
    }}
    
    async setupUI() {{
        const productList = this.products.map(p => 
            `<div class="product">
                <h3>${{p.name}}</h3>
                <p>Price: ${{p.price}} ZHTP</p>
                <button onclick="app.buyProduct(${{p.id}})">Buy</button>
            </div>`
        ).join('');
        
        document.body.innerHTML = `
            <h1>{name}</h1>
            <div class="products">${{productList}}</div>
        `;
    }}
    
    async buyProduct(productId) {{
        // Implement purchase logic
        alert(`Purchasing product ${{productId}}`);
    }}
}}

const app = new {name.replace(' ', '')}Marketplace();
app.init();
"""
            }
        }
        
        template_code = templates.get(template, {}).get(language)
        if not template_code:
            # Return basic template
            return templates["basic"]["javascript"]
        
        return template_code
    
    async def _create_initial_resources(self, template: str, name: str) -> Dict[str, bytes]:
        """Create initial resources for DApp"""
        resources = {
            "style.css": f"""
/* {name} DApp Styles */
body {{
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 20px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
}}

h1 {{
    text-align: center;
    margin-bottom: 30px;
}}

.products {{
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
}}

.product {{
    background: rgba(255, 255, 255, 0.1);
    padding: 20px;
    border-radius: 10px;
    text-align: center;
}}

button {{
    background: #4CAF50;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    cursor: pointer;
}}

button:hover {{
    background: #45a049;
}}
""".encode('utf-8'),
            
            "manifest.json": json.dumps({
                "name": name,
                "version": "1.0.0",
                "description": f"Decentralized {name} application",
                "author": "ZHTP Developer",
                "permissions": ["storage", "contracts", "networking"],
                "entry_point": "app.js"
            }).encode('utf-8')
        }
        
        return resources
    
    async def _read_source_code(self, source_path: str) -> str:
        """Read source code from file"""
        try:
            with open(source_path, 'r', encoding='utf-8') as f:
                return f.read()
        except Exception as e:
            logger.error(f"Failed to read source code: {e}")
            return ""
    
    async def _read_resources(self, source_path: str) -> Dict[str, bytes]:
        """Read resources from directory"""
        import os
        resources = {}
        
        try:
            base_dir = os.path.dirname(source_path)
            for root, dirs, files in os.walk(base_dir):
                for file in files:
                    if file.endswith(('.css', '.json', '.png', '.jpg', '.gif')):
                        file_path = os.path.join(root, file)
                        relative_path = os.path.relpath(file_path, base_dir)
                        
                        with open(file_path, 'rb') as f:
                            resources[relative_path] = f.read()
        
        except Exception as e:
            logger.error(f"Failed to read resources: {e}")
        
        return resources
    
    async def _apply_config(self, source_code: str, config: Dict[str, Any]) -> str:
        """Apply configuration to source code"""
        # Simple configuration replacement
        for key, value in config.items():
            placeholder = f"{{{{CONFIG_{key.upper()}}}}}"
            source_code = source_code.replace(placeholder, str(value))
        
        return source_code

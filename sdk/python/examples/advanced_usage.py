"""
ZHTP Python SDK - Advanced Examples

Advanced usage patterns for the ZHTP Python SDK including
real-world applications, migration examples, and complex scenarios.
"""

import asyncio
import json
import hashlib
from datetime import datetime
from zhtp import ZhtpClient, ZhtpConfig

async def migrate_from_http():
    """Example: Migrate existing HTTP application to ZHTP"""
    print("=== Migration from HTTP to ZHTP ===")
    
    # BEFORE: Traditional HTTP code
    # import requests
    # response = requests.get("https://api.example.com/users")
    # data = response.json()
    
    # AFTER: ZHTP replacement
    async with ZhtpClient() as client:
        # Direct replacement for HTTP GET
        data = await client.fetch_json("api.zhtp", "/users")
        print(f"Migrated API call - Fetched {len(data)} user records")
        
        # No more SSL/TLS concerns - quantum-resistant by default
        # No more DNS dependencies - blockchain-based resolution
        # No more HTTP status codes - direct content or exceptions

async def build_decentralized_marketplace():
    """Example: Build a complete decentralized marketplace"""
    print("\n=== Decentralized Marketplace DApp ===")
    
    async with ZhtpClient() as client:
        # 1. Deploy marketplace smart contract
        marketplace_contract = """
        class DecentralizedMarketplace {
            constructor() {
                this.products = new Map();
                this.orders = new Map();
                this.nextProductId = 1;
                this.nextOrderId = 1;
            }
            
            addProduct(name, description, price, seller) {
                const productId = this.nextProductId++;
                this.products.set(productId, {
                    id: productId,
                    name: name,
                    description: description,
                    price: price,
                    seller: seller,
                    available: true,
                    createdAt: new Date().toISOString()
                });
                return productId;
            }
            
            getProducts() {
                return Array.from(this.products.values())
                    .filter(p => p.available);
            }
            
            purchaseProduct(productId, buyer, paymentHash) {
                const product = this.products.get(productId);
                if (!product || !product.available) {
                    throw new Error("Product not available");
                }
                
                const orderId = this.nextOrderId++;
                this.orders.set(orderId, {
                    id: orderId,
                    productId: productId,
                    buyer: buyer,
                    seller: product.seller,
                    amount: product.price,
                    paymentHash: paymentHash,
                    status: "pending",
                    createdAt: new Date().toISOString()
                });
                
                product.available = false;
                return orderId;
            }
            
            getOrder(orderId) {
                return this.orders.get(orderId);
            }
        }
        """
        
        contract_address = await client.contracts.deploy_contract(
            name="DecentralizedMarketplace",
            source_code=marketplace_contract,
            language="javascript"
        )
        print(f"Deployed marketplace contract: {contract_address}")
        
        # 2. Add some products
        product_id = await client.call_contract(
            contract_address,
            "addProduct",
            {
                "name": "Quantum-Safe Laptop",
                "description": "Laptop with quantum-resistant encryption",
                "price": 1500,
                "seller": "0xseller123"
            }
        )
        print(f"Added product ID: {product_id}")
        
        # 3. Deploy marketplace frontend
        marketplace_frontend = """
        class MarketplaceDApp {
            constructor(contractAddress) {
                this.contractAddress = contractAddress;
                this.zhtp = new ZHTTPClient();
            }
            
            async init() {
                await this.zhtp.connect();
                await this.loadProducts();
                this.setupUI();
            }
            
            async loadProducts() {
                this.products = await this.zhtp.callContract(
                    this.contractAddress, 
                    "getProducts"
                );
            }
            
            setupUI() {
                document.body.innerHTML = `
                    <div class="marketplace">
                        <h1>Decentralized Marketplace</h1>
                        <div class="products" id="products"></div>
                    </div>
                `;
                
                this.renderProducts();
            }
            
            renderProducts() {
                const container = document.getElementById('products');
                container.innerHTML = this.products.map(product => `
                    <div class="product-card">
                        <h3>${product.name}</h3>
                        <p>${product.description}</p>
                        <p class="price">$${product.price} ZHTP</p>
                        <button onclick="marketplace.purchaseProduct(${product.id})">
                            Buy Now
                        </button>
                    </div>
                `).join('');
            }
            
            async purchaseProduct(productId) {
                try {
                    const paymentHash = await this.processPayment(productId);
                    const orderId = await this.zhtp.callContract(
                        this.contractAddress,
                        "purchaseProduct",
                        {
                            productId: productId,
                            buyer: await this.zhtp.getAddress(),
                            paymentHash: paymentHash
                        }
                    );
                    
                    alert(`Purchase successful! Order ID: ${orderId}`);
                    await this.loadProducts();
                    this.renderProducts();
                } catch (error) {
                    alert(`Purchase failed: ${error.message}`);
                }
            }
            
            async processPayment(productId) {
                // Mock payment processing
                return "payment_hash_" + Date.now();
            }
        }
        
        const marketplace = new MarketplaceDApp("${contract_address}");
        marketplace.init();
        """
        
        # Deploy DApp
        dapp_hash = await client.deploy_dapp(
            name="Decentralized Marketplace",
            domain="marketplace.zhtp",
            source_code=marketplace_frontend.replace("${contract_address}", contract_address),
            language="javascript",
            resources={
                "style.css": """
                .marketplace { max-width: 1200px; margin: 0 auto; padding: 20px; }
                .products { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
                .product-card { border: 1px solid #ddd; padding: 20px; border-radius: 8px; }
                .price { font-size: 1.2em; font-weight: bold; color: #2196F3; }
                button { background: #4CAF50; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; }
                button:hover { background: #45a049; }
                """.encode()
            }
        )
        print(f"Deployed marketplace DApp: {dapp_hash}")
        print("Access at: marketplace.zhtp")

async def file_sharing_system():
    """Example: Decentralized file sharing system"""
    print("\n=== Decentralized File Sharing ===")
    
    async with ZhtpClient() as client:
        # 1. Upload a file with encryption
        file_content = b"This is a secret document that needs to be shared securely."
        
        file_hash = await client.storage.store_content(
            content=file_content,
            path="/secret-document.txt",
            encryption_key=None,  # Auto-generate encryption key
            redundancy=5,  # Store on 5 nodes for reliability
            anonymous=True
        )
        print(f"Uploaded encrypted file: {file_hash}")
        
        # 2. Share file with specific users (access control contract)
        access_contract = """
        class FileAccessControl {
            constructor(fileHash, owner) {
                this.fileHash = fileHash;
                this.owner = owner;
                this.authorizedUsers = new Set([owner]);
                this.accessLog = [];
            }
            
            grantAccess(userAddress) {
                if (this.getCaller() !== this.owner) {
                    throw new Error("Only owner can grant access");
                }
                this.authorizedUsers.add(userAddress);
                this.accessLog.push({
                    action: "grant",
                    user: userAddress,
                    timestamp: new Date().toISOString()
                });
                return true;
            }
            
            revokeAccess(userAddress) {
                if (this.getCaller() !== this.owner) {
                    throw new Error("Only owner can revoke access");
                }
                this.authorizedUsers.delete(userAddress);
                this.accessLog.push({
                    action: "revoke",
                    user: userAddress,
                    timestamp: new Date().toISOString()
                });
                return true;
            }
            
            checkAccess(userAddress) {
                return this.authorizedUsers.has(userAddress);
            }
            
            getAccessLog() {
                if (this.getCaller() !== this.owner) {
                    throw new Error("Only owner can view access log");
                }
                return this.accessLog;
            }
        }
        """
        
        access_contract_address = await client.contracts.deploy_contract(
            name="FileAccessControl",
            source_code=access_contract,
            language="javascript",
            initial_state={
                "fileHash": file_hash,
                "owner": "0xowner123"
            }
        )
        print(f"Deployed access control contract: {access_contract_address}")
        
        # 3. Grant access to specific users
        await client.call_contract(
            access_contract_address,
            "grantAccess",
            {"userAddress": "0xuser456"}
        )
        print("Granted access to user: 0xuser456")

async def streaming_media_platform():
    """Example: Decentralized streaming media platform"""
    print("\n=== Decentralized Streaming Platform ===")
    
    async with ZhtpClient() as client:
        # 1. Upload video content in chunks
        video_chunks = [
            b"video_chunk_001_data...",
            b"video_chunk_002_data...",
            b"video_chunk_003_data..."
        ]
        
        chunk_hashes = []
        for i, chunk in enumerate(video_chunks):
            chunk_hash = await client.storage.store_content(
                content=chunk,
                path=f"/video/chunk_{i:03d}.dat",
                redundancy=3
            )
            chunk_hashes.append(chunk_hash)
            print(f"Uploaded video chunk {i+1}: {chunk_hash[:16]}...")
        
        # 2. Create video manifest
        video_manifest = {
            "title": "My Decentralized Video",
            "duration": 180,  # seconds
            "resolution": "1080p",
            "chunks": chunk_hashes,
            "created_at": datetime.now().isoformat()
        }
        
        manifest_hash = await client.storage.store_content(
            content=json.dumps(video_manifest).encode(),
            path="/video/manifest.json"
        )
        print(f"Uploaded video manifest: {manifest_hash}")
        
        # 3. Deploy streaming contract
        streaming_contract = """
        class StreamingPlatform {
            constructor() {
                this.videos = new Map();
                this.subscriptions = new Map();
                this.nextVideoId = 1;
            }
            
            publishVideo(title, manifestHash, creator, price) {
                const videoId = this.nextVideoId++;
                this.videos.set(videoId, {
                    id: videoId,
                    title: title,
                    manifestHash: manifestHash,
                    creator: creator,
                    price: price,
                    views: 0,
                    earnings: 0,
                    publishedAt: new Date().toISOString()
                });
                return videoId;
            }
            
            purchaseAccess(videoId, viewer, paymentHash) {
                const video = this.videos.get(videoId);
                if (!video) throw new Error("Video not found");
                
                this.subscriptions.set(`${viewer}_${videoId}`, {
                    videoId: videoId,
                    viewer: viewer,
                    purchasedAt: new Date().toISOString(),
                    paymentHash: paymentHash
                });
                
                video.views += 1;
                video.earnings += video.price;
                return true;
            }
            
            hasAccess(viewer, videoId) {
                return this.subscriptions.has(`${viewer}_${videoId}`);
            }
            
            getVideo(videoId) {
                return this.videos.get(videoId);
            }
        }
        """
        
        streaming_address = await client.contracts.deploy_contract(
            name="StreamingPlatform",
            source_code=streaming_contract,
            language="javascript"
        )
        print(f"Deployed streaming contract: {streaming_address}")
        
        # 4. Publish video
        video_id = await client.call_contract(
            streaming_address,
            "publishVideo",
            {
                "title": "My Decentralized Video",
                "manifestHash": manifest_hash,
                "creator": "0xcreator789",
                "price": 5
            }
        )
        print(f"Published video with ID: {video_id}")

async def database_replacement():
    """Example: Replace traditional database with ZHTP storage"""
    print("\n=== Database Replacement ===")
    
    async with ZhtpClient() as client:
        # 1. Create database schema contract
        database_contract = """
        class DecentralizedDatabase {
            constructor() {
                this.tables = new Map();
                this.indexes = new Map();
                this.nextId = 1;
            }
            
            createTable(name, schema) {
                this.tables.set(name, {
                    name: name,
                    schema: schema,
                    records: new Map(),
                    createdAt: new Date().toISOString()
                });
                return true;
            }
            
            insert(tableName, data) {
                const table = this.tables.get(tableName);
                if (!table) throw new Error("Table not found");
                
                const recordId = this.nextId++;
                const record = {
                    id: recordId,
                    data: data,
                    createdAt: new Date().toISOString(),
                    updatedAt: new Date().toISOString()
                };
                
                table.records.set(recordId, record);
                this.updateIndexes(tableName, recordId, data);
                return recordId;
            }
            
            select(tableName, where = null) {
                const table = this.tables.get(tableName);
                if (!table) throw new Error("Table not found");
                
                let records = Array.from(table.records.values());
                
                if (where) {
                    records = records.filter(record => {
                        return Object.keys(where).every(key =>
                            record.data[key] === where[key]
                        );
                    });
                }
                
                return records;
            }
            
            update(tableName, recordId, data) {
                const table = this.tables.get(tableName);
                if (!table) throw new Error("Table not found");
                
                const record = table.records.get(recordId);
                if (!record) throw new Error("Record not found");
                
                record.data = { ...record.data, ...data };
                record.updatedAt = new Date().toISOString();
                
                this.updateIndexes(tableName, recordId, record.data);
                return true;
            }
            
            updateIndexes(tableName, recordId, data) {
                // Simple indexing implementation
                const indexKey = `${tableName}_${JSON.stringify(data)}`;
                if (!this.indexes.has(indexKey)) {
                    this.indexes.set(indexKey, new Set());
                }
                this.indexes.get(indexKey).add(recordId);
            }
        }
        """
        
        db_address = await client.contracts.deploy_contract(
            name="DecentralizedDatabase",
            source_code=database_contract,
            language="javascript"
        )
        print(f"Deployed database contract: {db_address}")
        
        # 2. Create tables and insert data
        await client.call_contract(
            db_address,
            "createTable",
            {
                "name": "users",
                "schema": {
                    "id": "number",
                    "name": "string",
                    "email": "string",
                    "created_at": "datetime"
                }
            }
        )
        print("Created 'users' table")
        
        # Insert records
        user_ids = []
        for i in range(3):
            user_id = await client.call_contract(
                db_address,
                "insert",
                {
                    "tableName": "users",
                    "data": {
                        "name": f"User {i+1}",
                        "email": f"user{i+1}@zhtp.network",
                        "created_at": datetime.now().isoformat()
                    }
                }
            )
            user_ids.append(user_id)
            print(f"Inserted user {i+1} with ID: {user_id}")
        
        # Query records
        users = await client.call_contract(
            db_address,
            "select",
            {"tableName": "users"}
        )
        print(f"Retrieved {len(users)} users from database")

async def main():
    """Run all advanced examples"""
    print("ZHTP Python SDK - Advanced Examples")
    print("=" * 50)
    
    try:
        await migrate_from_http()
        await build_decentralized_marketplace()
        await file_sharing_system()
        await streaming_media_platform()
        await database_replacement()
        
        print("\n" + "=" * 50)
        print("All advanced examples completed successfully!")
        print("\nThese examples demonstrate:")
        print("• Complete migration from traditional HTTP/DNS")
        print("• Real-world decentralized applications")
        print("• Advanced smart contract patterns")
        print("• Decentralized storage strategies")
        print("• Privacy-preserving architectures")
        
    except Exception as e:
        print(f"\nError running examples: {e}")

if __name__ == "__main__":
    asyncio.run(main())

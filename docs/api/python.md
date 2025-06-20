# 🐍 ZHTP Python SDK - Complete Decentralized Internet Replacement

**The ZHTP Python SDK provides native access to the decentralized internet, completely replacing traditional HTTP, DNS, and SSL/TLS infrastructure.**

## 🚀 Installation

```bash
# Install from PyPI (recommended)
pip install zhtp-sdk

# Install from source for latest features
pip install git+https://github.com/zhtp-network/zhtp-python-sdk.git

# Install with all optional dependencies
pip install zhtp-sdk[full]

# For development
pip install zhtp-sdk[dev]
```

## ⚡ Quick Start - Complete Traditional Internet Replacement

```python
import asyncio
from zhtp import ZhtpClient, BlockchainDNS, ZkCertificateAuthority

async def main():
    # Initialize ZHTP client (replaces HTTP clients entirely)
    client = ZhtpClient(
        network="mainnet",  # or "testnet", "local"
        privacy_level="maximum",  # Built-in anonymity (no VPN needed)
        security_level="quantum_resistant"  # Post-quantum cryptography
    )
    
    # Connect to decentralized network (no ISP dependencies)
    await client.connect()
    
    # Resolve domain via blockchain DNS (no traditional DNS servers)
    domain_info = await client.blockchain_dns.resolve("news.zhtp")
    print(f"Domain resolved: {domain_info.content_hash}")
    
    # Fetch content via decentralized storage (no HTTP/HTTPS)
    content = await client.fetch_content(
        domain="news.zhtp",
        path="/latest-news",
        anonymous=True  # Anonymous access by default
    )
    
    print(f"Content fetched: {content[:100]}...")

if __name__ == "__main__":
    asyncio.run(main())
```

## 🏗️ Core API Reference

### ZhtpClient - Main Interface

The `ZhtpClient` replaces all traditional internet infrastructure:

```python
class ZhtpClient:
    """
    Main ZHTP client that replaces:
    - HTTP/HTTPS clients (requests, aiohttp, etc.)
    - DNS resolution (socket.getaddrinfo, etc.)
    - TLS/SSL libraries (ssl, cryptography, etc.)
    - VPN/proxy services (complete anonymity built-in)
    """
    
    def __init__(
        self,
        network: str = "mainnet",  # "mainnet", "testnet", "local"
        privacy_level: str = "high",  # "standard", "high", "maximum"
        security_level: str = "quantum_resistant",
        node_endpoints: Optional[List[str]] = None,
        wallet_private_key: Optional[str] = None,
        storage_cache_size: int = 100_000_000,  # 100MB default cache
    ):
        """
        Initialize ZHTP client with decentralized internet capabilities.
        
        Args:
            network: Which ZHTP network to connect to
            privacy_level: Anonymity level (higher = more hops, mixing)
            security_level: Cryptographic security level
            node_endpoints: Custom bootstrap nodes (optional)
            wallet_private_key: For transactions and DApp deployment
            storage_cache_size: Local cache for content
        """
```

#### Core Connection Methods

```python
# Establish connection to decentralized network
async def connect(self) -> bool:
    """
    Connect to ZHTP network without any traditional infrastructure.
    Establishes peer connections, syncs blockchain state, sets up routing.
    """
    client = ZhtpClient()
    success = await client.connect()
    if success:
        print("✅ Connected to decentralized internet")
    return success

# Get network status (replaces ping, traceroute, etc.)
async def get_network_status(self) -> NetworkStatus:
    """Get comprehensive network health and statistics"""
    status = await client.get_network_status()
    print(f"Network: {status.name}")
    print(f"Active nodes: {status.node_count}")
    print(f"Block height: {status.block_height}")
    print(f"Your anonymity set: {status.anonymity_set_size}")
    return status

# Disconnect from network
async def disconnect(self) -> None:
    """Cleanly disconnect from ZHTP network"""
    await client.disconnect()
```

### BlockchainDNS - Decentralized Domain Resolution

Completely replaces traditional DNS infrastructure:

```python
class BlockchainDNS:
    """
    Blockchain-based DNS that replaces traditional DNS entirely.
    Provides censorship-resistant, cryptographically verified domain resolution.
    """
    
    async def resolve(self, domain: str) -> DomainInfo:
        """
        Resolve domain using blockchain DNS (no traditional DNS servers).
        
        Returns cryptographically verified domain information:
        - Content hash (IPFS-style addressing)
        - ZK certificate for secure communication
        - Owner verification
        - Routing proofs for anonymous access
        """
        domain_info = await client.blockchain_dns.resolve("my-app.zhtp")
        
        # Returns DomainInfo with:
        print(f"Content hash: {domain_info.content_hash}")
        print(f"Owner: {domain_info.owner_address}")
        print(f"Certificate: {domain_info.zk_certificate}")
        print(f"Last updated: {domain_info.last_update}")
        
        return domain_info
    
    async def register_domain(
        self,
        domain: str,
        content_hash: str,
        owner_keypair: Keypair,
        stake_amount: int = 1000  # ZHTP tokens
    ) -> RegistrationResult:
        """
        Register a new domain on the blockchain DNS.
        One-time payment replaces annual DNS fees.
        """
        result = await client.blockchain_dns.register_domain(
            domain="my-new-app.zhtp",
            content_hash="QmX7B8a...",  # IPFS-style hash
            owner_keypair=my_keypair,
            stake_amount=1000  # One-time cost vs annual DNS fees
        )
        
        if result.success:
            print(f"✅ Domain registered: {result.domain}")
            print(f"Transaction hash: {result.tx_hash}")
        
        return result
    
    async def update_domain(
        self,
        domain: str,
        new_content_hash: str,
        owner_keypair: Keypair
    ) -> UpdateResult:
        """Update domain content hash (like DNS record update)"""
        result = await client.blockchain_dns.update_domain(
            domain="my-app.zhtp",
            new_content_hash="QmY8C9b...",
            owner_keypair=my_keypair
        )
        return result
    
    async def query_domain_history(self, domain: str) -> List[DomainEvent]:
        """Get complete history of domain changes (immutable audit trail)"""
        history = await client.blockchain_dns.query_domain_history("news.zhtp")
        for event in history:
            print(f"{event.timestamp}: {event.event_type} - {event.details}")
        return history
```

### ZkCertificateAuthority - Post-Quantum Certificate Issuance

Completely replaces traditional certificate authorities (DigiCert, Let's Encrypt, etc.):

```python
class ZkCertificateAuthority:
    """
    Zero-knowledge certificate authority that replaces traditional SSL/TLS CAs.
    Issues quantum-resistant certificates without trusted third parties.
    """
    
    async def issue_certificate(
        self,
        domain: str,
        security_level: SecurityLevel = SecurityLevel.QUANTUM_RESISTANT,
        validity_period: int = 365 * 24 * 60 * 60,  # 1 year in seconds
        privacy_level: PrivacyLevel = PrivacyLevel.HIGH
    ) -> ZkCertificate:
        """
        Issue a zero-knowledge certificate for domain.
        Costs ~$1-10 vs $100-1000 for traditional SSL certificates.
        """
        certificate = await client.zk_certificate_authority.issue_certificate(
            domain="my-app.zhtp",
            security_level=SecurityLevel.QUANTUM_RESISTANT,
            validity_period=365 * 24 * 60 * 60,  # 1 year
            privacy_level=PrivacyLevel.MAXIMUM
        )
        
        print(f"✅ ZK Certificate issued for {certificate.domain}")
        print(f"Certificate ID: {certificate.cert_id}")
        print(f"Expires: {certificate.expiry_date}")
        print(f"Security: {certificate.security_level}")
        print(f"Cost: {certificate.cost_zhtp_tokens} ZHTP tokens (~${certificate.cost_usd})")
        
        return certificate
    
    async def verify_certificate(
        self,
        certificate: ZkCertificate,
        domain: str
    ) -> VerificationResult:
        """
        Verify certificate using zero-knowledge proofs.
        No need to contact certificate authority servers.
        """
        verification = await client.zk_certificate_authority.verify_certificate(
            certificate=certificate,
            domain="secure-app.zhtp"
        )
        
        return VerificationResult(
            valid=verification.is_valid,
            expired=verification.is_expired,
            revoked=verification.is_revoked,
            quantum_resistant=verification.is_quantum_safe,
            trust_score=verification.trust_score
        )
    
    async def revoke_certificate(
        self,
        certificate_id: str,
        owner_keypair: Keypair,
        reason: str = "Owner request"
    ) -> RevocationResult:
        """Revoke certificate with cryptographic proof"""
        result = await client.zk_certificate_authority.revoke_certificate(
            certificate_id=certificate_id,
            owner_keypair=owner_keypair,
            reason=reason
        )
        return result
    
    async def list_certificates(
        self,
        owner_address: str
    ) -> List[ZkCertificate]:
        """List all certificates owned by an address"""
        certificates = await client.zk_certificate_authority.list_certificates(
            owner_address="0x1234..."
        )
        
        for cert in certificates:
            print(f"Domain: {cert.domain} | Status: {cert.status} | Expires: {cert.expiry}")
        
        return certificates
```

### DecentralizedStorage - Content Distribution Network

Replaces traditional cloud storage and CDNs:

```python
class DecentralizedStorage:
    """
    Decentralized content storage and distribution that replaces
    traditional cloud storage (AWS S3, Google Cloud) and CDNs.
    """
    
    async def store_content(
        self,
        content: bytes,
        content_type: str,
        redundancy_level: int = 5,
        encryption: bool = True,
        public: bool = False
    ) -> StorageResult:
        """
        Store content across decentralized network.
        Earn tokens for hosting others' content.
        """
        result = await client.decentralized_storage.store_content(
            content=my_website_files,
            content_type="application/zip",
            redundancy_level=7,  # Store across 7 nodes for high availability
            encryption=True,     # End-to-end encryption
            public=True         # Publicly accessible content
        )
        
        print(f"✅ Content stored across {result.node_count} nodes")
        print(f"Content hash: {result.content_hash}")
        print(f"Access URL: zhtp://{result.content_hash}")
        print(f"Monthly hosting cost: {result.cost_zhtp_tokens} ZHTP (~${result.cost_usd})")
        
        return result
    
    async def retrieve_content(
        self,
        content_hash: str,
        verify_integrity: bool = True,
        preferred_nodes: Optional[List[str]] = None
    ) -> Content:
        """
        Retrieve content from decentralized storage.
        Automatically finds fastest/closest nodes.
        """
        content = await client.decentralized_storage.retrieve_content(
            content_hash="QmX7B8a...",
            verify_integrity=True,
            preferred_regions=[Region.NORTH_AMERICA, Region.EUROPE]
        )
        
        print(f"✅ Content retrieved from {content.source_node_count} nodes")
        print(f"Download speed: {content.download_speed_mbps} Mbps")
        print(f"Integrity verified: {content.integrity_verified}")
        
        return content
    
    async def pin_content(
        self,
        content_hash: str,
        pin_duration: int = 30 * 24 * 60 * 60  # 30 days
    ) -> PinResult:
        """
        Pin content to ensure it stays available.
        Pay small fee to guarantee hosting.
        """
        result = await client.decentralized_storage.pin_content(
            content_hash="QmY8C9b...",
            pin_duration=90 * 24 * 60 * 60,  # 90 days
            redundancy_level=5
        )
        
        return result
    
    async def host_content_for_rewards(
        self,
        storage_capacity: int,  # Bytes
        bandwidth_capacity: int,  # Mbps
        uptime_commitment: float = 0.99  # 99% uptime
    ) -> HostingResult:
        """
        Host content for other users and earn ZHTP tokens.
        Turn your spare storage/bandwidth into income.
        """
        hosting = await client.decentralized_storage.start_hosting(
            storage_capacity=100 * 1024**3,  # 100 GB
            bandwidth_capacity=100,          # 100 Mbps
            uptime_commitment=0.995         # 99.5% uptime
        )
        
        print(f"✅ Started hosting with {hosting.storage_gb} GB capacity")
        print(f"Estimated monthly earnings: {hosting.estimated_monthly_earnings} ZHTP")
        
        return hosting
    
    async def get_storage_economics(self) -> StorageEconomics:
        """Get current storage pricing and reward rates"""
        economics = await client.decentralized_storage.get_economics()
        
        print(f"Storage cost: {economics.cost_per_gb_month} ZHTP/GB/month")
        print(f"Hosting rewards: {economics.hosting_reward_per_gb} ZHTP/GB/month")
        print(f"Network capacity: {economics.total_network_capacity_tb} TB")
        
        return economics
```

### AnonymousRouting - Built-in Privacy Layer

Replaces VPNs and provides untraceable communication:

```python
class AnonymousRouting:
    """
    Zero-knowledge anonymous routing that replaces VPNs and provides
    untraceable communication without revealing user identity or destination.
    """
    
    async def create_anonymous_route(
        self,
        destination: str,
        hops: int = 5,
        privacy_level: PrivacyLevel = PrivacyLevel.HIGH,
        geographic_constraints: Optional[List[Region]] = None
    ) -> AnonymousRoute:
        """
        Create anonymous route to destination with specified privacy level.
        More hops = more anonymity but slightly higher latency.
        """
        route = await client.anonymous_routing.create_route(
            destination="sensitive-docs.zhtp",
            hops=7,  # High anonymity
            privacy_level=PrivacyLevel.MAXIMUM,
            geographic_constraints=[Region.EUROPE, Region.ASIA]  # Route through specific regions
        )
        
        print(f"✅ Anonymous route created")
        print(f"Route ID: {route.route_id}")
        print(f"Hops: {route.hop_count}")
        print(f"Estimated latency: {route.estimated_latency_ms}ms")
        print(f"Anonymity score: {route.anonymity_score}/100")
        
        return route
    
    async def send_anonymous_message(
        self,
        route: AnonymousRoute,
        message: bytes,
        recipient_public_key: str
    ) -> MessageResult:
        """Send message through anonymous route with end-to-end encryption"""
        result = await client.anonymous_routing.send_message(
            route=route,
            message=message,
            recipient_public_key=recipient_public_key,
            encrypt=True,
            sign=True
        )
        
        return MessageResult(
            sent=result.success,
            message_id=result.message_id,
            delivery_confirmation=result.confirmed
        )
    
    async def fetch_content_anonymously(
        self,
        domain: str,
        path: str = "/",
        route: Optional[AnonymousRoute] = None
    ) -> Content:
        """
        Fetch content anonymously without revealing your IP or identity.
        Automatically creates anonymous route if not provided.
        """
        content = await client.anonymous_routing.fetch_content(
            domain="private-docs.zhtp",
            path="/confidential-report.pdf",
            anonymity_level=PrivacyLevel.MAXIMUM,
            verify_content=True  # Cryptographically verify content integrity
        )
        
        print(f"✅ Content fetched anonymously")
        print(f"Content type: {content.content_type}")
        print(f"Size: {content.size_bytes} bytes")
        print(f"Verified: {content.verified}")
        
        return content
    
    async def get_anonymity_metrics(self) -> AnonymityMetrics:
        """Get current anonymity metrics and network mixing status"""
        metrics = await client.anonymous_routing.get_anonymity_metrics()
        
        print(f"Your anonymity set size: {metrics.anonymity_set_size}")
        print(f"Network mixing strength: {metrics.mixing_strength}")
        print(f"Traffic analysis resistance: {metrics.traffic_analysis_resistance}")
        
        return metrics
```

## 🎯 Complete Examples

### Example 1: Replace Traditional Web Scraping

```python
import asyncio
from zhtp import ZhtpClient

async def scrape_decentralized_news():
    """
    Scrape news from decentralized news sites.
    Completely anonymous, no IP blocking, no geo-restrictions.
    """
    client = ZhtpClient(privacy_level="maximum")
    await client.connect()
    
    # List of decentralized news sites
    news_sites = [
        "news.zhtp",
        "crypto-news.zhtp", 
        "world-events.zhtp"
    ]
    
    articles = []
    
    for site in news_sites:
        print(f"Fetching from {site}...")
        
        # Anonymous access to content
        content = await client.fetch_content(
            domain=site,
            path="/api/latest",
            anonymous=True,  # Completely untraceable
            headers={"Accept": "application/json"}
        )
        
        # Parse articles
        site_articles = json.loads(content)
        articles.extend(site_articles)
        
        print(f"✅ Got {len(site_articles)} articles from {site}")
    
    print(f"Total articles collected: {len(articles)}")
    return articles

# Run the scraper
asyncio.run(scrape_decentralized_news())
```

### Example 2: Deploy a Social Media DApp

```python
import asyncio
from zhtp import ZhtpClient

async def deploy_social_dapp():
    """
    Deploy a decentralized social media application.
    Users own their data, no censorship, earn from content.
    """
    client = ZhtpClient(network="mainnet")
    await client.connect()
    
    # Load DApp files
    with open("social_app.html", "rb") as f:
        html_content = f.read()
    with open("app.js", "rb") as f:
        js_content = f.read()
    with open("smart_contract.wasm", "rb") as f:
        contract_bytecode = f.read()
    
    # Deploy the DApp
    result = await client.dapp_deployment.deploy_dapp(
        name="Decentralized Social Network",
        domain="my-social.zhtp",
        content={
            "/": html_content,
            "/app.js": js_content,
            "/style.css": b"/* CSS content */",
        },
        smart_contract=contract_bytecode,
        access_control=AccessControl(
            public_read=True,
            content_monetization=True,  # Earn from user engagement
            moderation_dao=True  # Community-based moderation
        )
    )
    
    if result.success:
        print("🎉 Social DApp deployed successfully!")
        print(f"Domain: {result.domain}")
        print(f"Users can access at: zhtp://{result.domain}")
        print(f"Expected monthly earnings: {result.estimated_earnings} ZHTP")
        
        # The DApp is now live on the decentralized internet
        # No servers to maintain, no hosting costs
        # Users access it anonymously through ZHTP browser
        # You earn tokens when users interact with content
    
    return result

# Deploy the DApp
asyncio.run(deploy_social_dapp())
```

### Example 3: Anonymous File Sharing

```python
import asyncio
from zhtp import ZhtpClient, AccessControl

async def share_files_anonymously():
    """
    Share files anonymously with end-to-end encryption.
    No file size limits, no server storage, earn rewards.
    """
    client = ZhtpClient(privacy_level="maximum")
    await client.connect()
    
    # Share large file (video, dataset, etc.)
    with open("large_dataset.zip", "rb") as f:
        file_content = f.read()
    
    # Store with high privacy and redundancy
    storage_result = await client.decentralized_storage.store_content(
        content=file_content,
        content_type="application/zip",
        encryption=True,           # End-to-end encrypted
        redundancy_level=10,       # High availability
        access_control=AccessControl(
            anonymous_access=True,    # No login required
            expiry_time=7 * 24 * 60 * 60,  # 7 days
            download_limit=100       # Max 100 downloads
        )
    )
    
    print("🔒 File shared anonymously!")
    print(f"Share URL: zhtp://{storage_result.content_hash}")
    print(f"Access expires: {storage_result.expiry_date}")
    print(f"Estimated bandwidth earnings: {storage_result.bandwidth_rewards} ZHTP/GB")
    
    return storage_result.content_hash

# Share files
share_url = asyncio.run(share_files_anonymously())
print(f"Anyone can download anonymously from: {share_url}")
```

### Example 4: Decentralized E-commerce Store

```python
import asyncio
from zhtp import ZhtpClient, SmartContract, PaymentProcessor

async def deploy_ecommerce_store():
    """
    Deploy decentralized e-commerce store.
    No payment processors, no chargebacks, global access.
    """
    client = ZhtpClient(network="mainnet")
    await client.connect()
    
    # Create store smart contract
    store_contract = SmartContract(
        name="DecentralizedStore",
        functions=[
            "add_product",
            "purchase_product", 
            "process_payment",
            "handle_dispute",
            "calculate_rewards"
        ]
    )
    
    # Deploy store
    store_result = await client.dapp_deployment.deploy_dapp(
        name="My Decentralized Store",
        domain="my-store.zhtp",
        smart_contract=store_contract,
        payment_processor=PaymentProcessor(
            accepted_tokens=["ZHTP", "BTC", "ETH"],
            escrow_enabled=True,      # Automatic escrow for disputes
            instant_settlements=True   # No waiting for payments
        ),
        global_shipping=True,         # Worldwide shipping
        no_kyc_required=True         # Anonymous purchases allowed
    )
    
    # Add products to store
    products = [
        {
            "name": "Digital Course",
            "price": 50,  # ZHTP tokens
            "type": "digital",
            "instant_delivery": True
        },
        {
            "name": "Physical Product", 
            "price": 100,
            "type": "physical",
            "shipping_required": True
        }
    ]
    
    for product in products:
        await client.smart_contracts.call_function(
            contract_address=store_result.contract_address,
            function="add_product",
            params=product
        )
    
    print("🛍️ Decentralized store deployed!")
    print(f"Store URL: zhtp://{store_result.domain}")
    print(f"Contract: {store_result.contract_address}")
    print(f"Payment methods: ZHTP, BTC, ETH")
    print(f"Transaction fees: 0.1% (vs 3-5% traditional)")
    
    return store_result

# Deploy store
store = asyncio.run(deploy_ecommerce_store())
```

### Example 5: Decentralized Streaming Platform

```python
import asyncio
from zhtp import ZhtpClient, StreamingProtocol, ContentMonetization

async def deploy_streaming_platform():
    """
    Deploy decentralized video streaming platform.
    Creators earn directly, no platform fees, global access.
    """
    client = ZhtpClient(network="mainnet")
    await client.connect()
    
    # Create streaming platform
    platform_result = await client.dapp_deployment.deploy_dapp(
        name="Decentralized Streaming",
        domain="stream.zhtp",
        features=[
            StreamingProtocol.ADAPTIVE_BITRATE,  # Automatic quality adjustment
            StreamingProtocol.P2P_DELIVERY,     # Peer-to-peer content delivery
            ContentMonetization.PAY_PER_VIEW,   # Direct creator payments
            ContentMonetization.SUBSCRIPTION,   # Monthly/yearly subscriptions
            ContentMonetization.TIP_SYSTEM      # Viewer tips to creators
        ],
        content_storage=ContentStorage(
            redundancy_level=15,      # High availability for streaming
            geographic_distribution=True,  # Global content nodes
            automatic_transcoding=True     # Multiple quality levels
        )
    )
    
    print("📺 Streaming platform deployed!")
    print(f"Platform URL: zhtp://{platform_result.domain}")
    print(f"Creator earnings: 95% (vs 50-70% traditional)")
    print(f"Global access: No geo-blocking")
    print(f"Censorship resistance: Built-in")
    
    # Example: Upload content as creator
    await upload_creator_content(client, platform_result.domain)
    
    return platform_result

async def upload_creator_content(client, platform_domain):
    """Upload content as a creator and set monetization"""
    
    # Upload video content
    with open("my_video.mp4", "rb") as f:
        video_content = f.read()
    
    upload_result = await client.content_platform.upload_content(
        platform_domain=platform_domain,
        content=video_content,
        metadata={
            "title": "My Awesome Video",
            "description": "Educational content about ZHTP",
            "tags": ["technology", "blockchain", "privacy"],
            "duration": 1800,  # 30 minutes
        },
        monetization=ContentMonetization(
            pay_per_view=5,      # 5 ZHTP tokens per view
            subscription_tier="premium",
            tips_enabled=True,
            revenue_split=0.95   # Creator gets 95%, platform gets 5%
        ),
        accessibility=AccessibilityOptions(
            subtitles=True,
            audio_descriptions=True,
            multiple_languages=["en", "es", "fr"]
        )
    )
    
    print(f"✅ Video uploaded: {upload_result.content_url}")
    print(f"Estimated monthly earnings: {upload_result.estimated_earnings} ZHTP")

# Deploy streaming platform
platform = asyncio.run(deploy_streaming_platform())
```

### Example 6: Decentralized Database & API

```python
import asyncio
from zhtp import ZhtpClient, DatabaseService, APIGateway

async def create_decentralized_database():
    """
    Create decentralized database with API access.
    No server maintenance, automatic scaling, earn from API usage.
    """
    client = ZhtpClient(network="mainnet")
    await client.connect()
    
    # Create database schema
    database_schema = {
        "users": {
            "fields": ["id", "username", "email_hash", "created_at"],
            "indexes": ["username", "created_at"],
            "privacy": "encrypted"  # Encrypted at rest
        },
        "posts": {
            "fields": ["id", "user_id", "content_hash", "timestamp", "likes"],
            "indexes": ["user_id", "timestamp"],
            "privacy": "public"
        },
        "analytics": {
            "fields": ["event", "user_hash", "timestamp", "metadata"],
            "indexes": ["event", "timestamp"],
            "privacy": "anonymous"  # Zero-knowledge analytics
        }
    }
    
    # Deploy database
    db_result = await client.database_service.create_database(
        name="MyDecentralizedDB",
        schema=database_schema,
        replication_factor=7,      # Data replicated across 7 nodes
        consistency_level="strong", # Strong consistency guarantees
        backup_frequency="hourly"   # Automatic backups
    )
    
    # Create API gateway
    api_result = await client.api_gateway.create_api(
        database_id=db_result.database_id,
        domain="api.my-service.zhtp",
        endpoints=[
            {
                "path": "/users",
                "methods": ["GET", "POST"],
                "rate_limit": 1000,  # Requests per minute
                "authentication": "wallet_signature"
            },
            {
                "path": "/posts",
                "methods": ["GET", "POST", "PUT", "DELETE"],
                "rate_limit": 500,
                "authentication": "optional"
            },
            {
                "path": "/analytics",
                "methods": ["POST"],
                "rate_limit": 10000,
                "authentication": "none"  # Public analytics endpoint
            }
        ],
        monetization=APIMonetization(
            pay_per_request=0.01,    # 0.01 ZHTP per API call
            subscription_tiers={
                "free": 1000,        # 1000 requests/month
                "pro": 100000,       # 100k requests/month for 50 ZHTP
                "enterprise": "unlimited"  # Unlimited for 500 ZHTP/month
            }
        )
    )
    
    print("🗄️ Decentralized database & API deployed!")
    print(f"Database ID: {db_result.database_id}")
    print(f"API URL: zhtp://{api_result.domain}")
    print(f"Revenue per 1M requests: {1000000 * 0.01} ZHTP")
    print(f"Monthly server costs: $0 (vs $500-5000 traditional)")
    
    return db_result, api_result

# Create database and API
db, api = asyncio.run(create_decentralized_database())
```

### Example 7: Migrate from Traditional Web Service

```python
import asyncio
from zhtp import ZhtpClient, MigrationTools

async def migrate_from_traditional_web():
    """
    Migrate existing web service from traditional infrastructure to ZHTP.
    Eliminate server costs, gain privacy, reduce complexity.
    """
    client = ZhtpClient(network="mainnet")
    await client.connect()
    
    # Migrate traditional web app
    migration_result = await client.migration_tools.migrate_webapp(
        source_type="traditional_web",
        source_config={
            "domain": "my-old-site.com",
            "server_ips": ["192.168.1.100", "192.168.1.101"],
            "database_url": "postgresql://...",
            "cdn_urls": ["cdn1.example.com", "cdn2.example.com"]
        },
        target_config={
            "new_domain": "my-site.zhtp",
            "preserve_functionality": True,
            "improve_privacy": True,
            "reduce_costs": True
        },
        migration_strategy=MigrationStrategy(
            database_migration="decentralized_storage",
            content_migration="ipfs_pinning", 
            api_migration="smart_contracts",
            dns_migration="blockchain_dns",
            ssl_migration="zk_certificates",
            user_migration="wallet_based_auth"
        )
    )
    
    print("🔄 Migration completed!")
    print(f"New URL: zhtp://{migration_result.new_domain}")
    
    # Cost comparison
    old_costs = migration_result.cost_analysis.traditional_monthly
    new_costs = migration_result.cost_analysis.zhtp_monthly
    savings = old_costs - new_costs
    
    print(f"Old monthly costs: ${old_costs}")
    print(f"New monthly costs: ${new_costs}")
    print(f"Monthly savings: ${savings} ({(savings/old_costs)*100:.1f}%)")
    
    # Performance comparison
    perf = migration_result.performance_analysis
    print(f"Latency improvement: {perf.latency_improvement}%")
    print(f"Uptime improvement: {perf.uptime_improvement}%")
    print(f"Global accessibility: {perf.global_reach_improvement}%")
    
    return migration_result

# Migrate existing service
migration = asyncio.run(migrate_from_traditional_web())
```

## 🚀 Advanced Features

### Smart Contract Integration

```python
from zhtp import SmartContract, ContractInterface

# Deploy custom smart contract
contract = SmartContract(
    source_code=open("my_contract.py").read(),  # Python smart contracts
    language="python",
    optimization_level="high"
)

deployment = await client.smart_contracts.deploy(
    contract=contract,
    initial_state={"owner": wallet_address, "balance": 1000000}
)

# Call contract functions
result = await client.smart_contracts.call_function(
    contract_address=deployment.address,
    function="transfer",
    params={"to": recipient_address, "amount": 1000},
    sender_keypair=my_keypair
)
```

### Zero-Knowledge Proofs

```python
from zhtp import ZkProofSystem, PrivacyProof

# Generate privacy proof
proof = await client.zk_proofs.generate_proof(
    proof_type="identity_verification",
    private_inputs={
        "age": 25,
        "country": "US",
        "verified": True
    },
    public_inputs={
        "minimum_age": 18,
        "allowed_countries": ["US", "CA", "EU"]
    }
)

# Verify without revealing private data
verification = await client.zk_proofs.verify_proof(proof)
print(f"Age verification: {verification.age_valid}")
print(f"Country verification: {verification.country_valid}")
# Private data (actual age, country) never revealed
```

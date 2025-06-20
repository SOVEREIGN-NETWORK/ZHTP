"""
ZHTP Python SDK - Basic Usage Examples

This file demonstrates basic usage of the ZHTP Python SDK
for common decentralized internet operations.
"""

import asyncio
import json
from zhtp import ZhtpClient, ZhtpConfig

async def basic_content_fetch():
    """Example: Fetch content from decentralized web"""
    print("=== Basic Content Fetch ===")
    
    # Create client with maximum privacy
    config = ZhtpConfig(
        network="testnet",
        privacy_level="maximum",
        anonymous_routing=True
    )
    
    async with ZhtpClient(config) as client:
        # Fetch content from decentralized domain
        content = await client.fetch_content("news.zhtp", "/latest-article")
        print(f"Fetched {len(content)} bytes from news.zhtp")
        
        # Fetch JSON data
        data = await client.fetch_json("api.zhtp", "/user-count")
        print(f"API Response: {data}")

async def publish_content():
    """Example: Publish content to decentralized web"""
    print("\n=== Publish Content ===")
    
    async with ZhtpClient() as client:
        # Publish text content
        text_content = "Hello, decentralized world!"
        content_hash = await client.publish_content(
            domain="my-site.zhtp",
            path="/hello.txt",
            content=text_content
        )
        print(f"Published text content: {content_hash}")
        
        # Publish JSON data
        json_data = {"message": "Hello from ZHTP", "timestamp": "2024-01-01"}
        json_hash = await client.publish_content(
            domain="my-api.zhtp",
            path="/message",
            content=json_data
        )
        print(f"Published JSON data: {json_hash}")

async def domain_operations():
    """Example: Domain registration and resolution"""
    print("\n=== Domain Operations ===")
    
    async with ZhtpClient() as client:
        # Resolve existing domain
        domain_record = await client.dns.resolve("example.zhtp")
        print(f"Resolved domain: {domain_record.domain}")
        print(f"Content hash: {domain_record.content_hash}")
        print(f"Owner: {domain_record.owner}")

async def smart_contract_interaction():
    """Example: Smart contract deployment and interaction"""
    print("\n=== Smart Contract Interaction ===")
    
    async with ZhtpClient() as client:
        # Deploy a simple contract
        contract_code = """
        class SimpleCounter {
            constructor() {
                this.count = 0;
            }
            
            increment() {
                this.count += 1;
                return this.count;
            }
            
            getCount() {
                return this.count;
            }
        }
        """
        
        contract_address = await client.contracts.deploy_contract(
            name="SimpleCounter",
            source_code=contract_code,
            language="javascript"
        )
        print(f"Deployed contract: {contract_address}")
        
        # Call contract methods
        result = await client.call_contract(
            contract_address,
            "increment"
        )
        print(f"Increment result: {result}")
        
        count = await client.call_contract(
            contract_address,
            "getCount"
        )
        print(f"Current count: {count}")

async def dapp_deployment():
    """Example: Deploy a simple DApp"""
    print("\n=== DApp Deployment ===")
    
    async with ZhtpClient() as client:
        # Create a simple DApp
        dapp_code = """
        class MyDApp {
            constructor() {
                this.title = "My First ZHTP DApp";
            }
            
            init() {
                document.body.innerHTML = `
                    <h1>${this.title}</h1>
                    <p>Welcome to the decentralized web!</p>
                    <button onclick="this.handleClick()">Click Me</button>
                `;
            }
            
            handleClick() {
                alert("Hello from ZHTP DApp!");
            }
        }
        
        const app = new MyDApp();
        app.init();
        """
        
        # Deploy DApp
        deployment_hash = await client.deploy_dapp(
            name="My First DApp",
            domain="my-dapp.zhtp",
            source_code=dapp_code,
            language="javascript"
        )
        print(f"Deployed DApp: {deployment_hash}")
        print("Visit your DApp at: my-dapp.zhtp")

async def anonymous_communication():
    """Example: Anonymous communication"""
    print("\n=== Anonymous Communication ===")
    
    config = ZhtpConfig(
        privacy_level="maximum",
        anonymous_routing=True
    )
    
    async with ZhtpClient(config) as client:
        # All operations are automatically anonymous
        content = await client.fetch_content(
            domain="private-docs.zhtp",
            path="/sensitive-info",
            anonymous=True
        )
        print(f"Anonymously fetched {len(content)} bytes")
        
        # Anonymous content publishing
        hash_result = await client.publish_content(
            domain="anonymous.zhtp",
            path="/whistle-blow",
            content="Sensitive information...",
            anonymous=True
        )
        print(f"Anonymously published content: {hash_result}")

async def network_status():
    """Example: Check network status"""
    print("\n=== Network Status ===")
    
    async with ZhtpClient() as client:
        stats = await client.network.get_network_stats()
        print(f"Connected peers: {stats.connected_peers}")
        print(f"Total peers: {stats.total_peers}")
        print(f"Network uptime: {stats.uptime:.1f}s")
        print(f"Messages sent: {stats.messages_sent}")

async def main():
    """Run all examples"""
    print("ZHTP Python SDK - Basic Examples")
    print("=" * 40)
    
    try:
        # Run all example functions
        await basic_content_fetch()
        await publish_content()
        await domain_operations()
        await smart_contract_interaction()
        await dapp_deployment()
        await anonymous_communication()
        await network_status()
        
        print("\n" + "=" * 40)
        print("All examples completed successfully!")
        
    except Exception as e:
        print(f"\nError running examples: {e}")

if __name__ == "__main__":
    # Run examples
    asyncio.run(main())

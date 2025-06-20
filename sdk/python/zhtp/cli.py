"""
ZHTP CLI - Command line interface for ZHTP operations

Provides command-line tools for interacting with the ZHTP network,
deploying DApps, managing content, and network operations.
"""

import asyncio
import click
import json
import logging
from pathlib import Path
from typing import Optional

from .client import ZhtpClient
from .exceptions import ZhtpError

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@click.group()
@click.option('--network', default='mainnet', help='Network to connect to (mainnet/testnet/local)')
@click.option('--privacy', default='maximum', help='Privacy level (maximum/high/medium/low)')
@click.option('--verbose', '-v', is_flag=True, help='Enable verbose logging')
@click.pass_context
def cli(ctx, network, privacy, verbose):
    """ZHTP Command Line Interface - Decentralized Internet Tools"""
    if verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    ctx.ensure_object(dict)
    ctx.obj['network'] = network
    ctx.obj['privacy'] = privacy

@cli.command()
@click.argument('domain')
@click.option('--path', default='/', help='Content path')
@click.option('--output', '-o', help='Output file path')
@click.pass_context
def fetch(ctx, domain, path, output):
    """Fetch content from ZHTP domain"""
    async def _fetch():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            async with ZhtpClient(config) as client:
                content = await client.fetch_content(domain, path)
                
                if output:
                    with open(output, 'wb') as f:
                        f.write(content)
                    click.echo(f"Content saved to {output}")
                else:
                    try:
                        click.echo(content.decode('utf-8'))
                    except UnicodeDecodeError:
                        click.echo(f"Binary content ({len(content)} bytes)")
                        
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_fetch())
    if not success:
        exit(1)

@cli.command()
@click.argument('domain')
@click.argument('content_file')
@click.option('--path', default='/', help='Content path')
@click.pass_context
def publish(ctx, domain, content_file, path):
    """Publish content to ZHTP domain"""
    async def _publish():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            # Read content
            content_path = Path(content_file)
            if not content_path.exists():
                click.echo(f"File not found: {content_file}", err=True)
                return False
            
            with open(content_path, 'rb') as f:
                content = f.read()
            
            async with ZhtpClient(config) as client:
                content_hash = await client.publish_content(domain, path, content)
                click.echo(f"Published to {domain}{path}")
                click.echo(f"Content hash: {content_hash}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_publish())
    if not success:
        exit(1)

@cli.command()
@click.argument('domain')
@click.pass_context
def resolve(ctx, domain):
    """Resolve ZHTP domain to content hash"""
    async def _resolve():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            async with ZhtpClient(config) as client:
                record = await client.dns.resolve(domain)
                
                click.echo(f"Domain: {record.domain}")
                click.echo(f"Content Hash: {record.content_hash}")
                click.echo(f"Owner: {record.owner}")
                click.echo(f"Expires: {record.expires}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_resolve())
    if not success:
        exit(1)

@cli.group()
def dapp():
    """DApp management commands"""
    pass

@dapp.command()
@click.argument('name')
@click.argument('domain')
@click.option('--template', default='basic', help='DApp template (basic/marketplace/social)')
@click.option('--language', default='javascript', help='Programming language')
@click.pass_context
def create(ctx, name, domain, template, language):
    """Create new DApp from template"""
    async def _create():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            async with ZhtpClient(config) as client:
                deployment_hash = await client.dapps.create_dapp(
                    name=name,
                    domain=domain,
                    template=template,
                    language=language
                )
                
                click.echo(f"Created DApp: {name}")
                click.echo(f"Domain: {domain}")
                click.echo(f"Deployment Hash: {deployment_hash}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_create())
    if not success:
        exit(1)

@dapp.command()
@click.argument('name')
@click.argument('domain')
@click.argument('source_file')
@click.option('--language', default='javascript', help='Programming language')
@click.pass_context
def deploy(ctx, name, domain, source_file, language):
    """Deploy existing DApp from source file"""
    async def _deploy():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            # Check source file
            source_path = Path(source_file)
            if not source_path.exists():
                click.echo(f"Source file not found: {source_file}", err=True)
                return False
            
            async with ZhtpClient(config) as client:
                deployment_hash = await client.dapps.deploy_existing_dapp(
                    name=name,
                    domain=domain,
                    source_path=str(source_path),
                    language=language
                )
                
                click.echo(f"Deployed DApp: {name}")
                click.echo(f"Domain: {domain}")
                click.echo(f"Deployment Hash: {deployment_hash}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_deploy())
    if not success:
        exit(1)

@dapp.command()
@click.argument('domain')
@click.pass_context
def info(ctx, domain):
    """Get DApp information"""
    async def _info():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            async with ZhtpClient(config) as client:
                dapp_info = await client.dapps.get_dapp_info(domain)
                
                if not dapp_info:
                    click.echo(f"DApp not found: {domain}", err=True)
                    return False
                
                click.echo(f"Name: {dapp_info.name}")
                click.echo(f"Domain: {dapp_info.domain}")
                click.echo(f"Version: {dapp_info.version}")
                click.echo(f"Status: {dapp_info.status.value}")
                click.echo(f"Owner: {dapp_info.owner}")
                click.echo(f"Deployed: {dapp_info.deployed_at}")
                click.echo(f"Contract: {dapp_info.contract_address}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_info())
    if not success:
        exit(1)

@cli.group()
def contract():
    """Smart contract commands"""
    pass

@contract.command()
@click.argument('name')
@click.argument('source_file')
@click.option('--language', default='javascript', help='Contract language')
@click.pass_context
def deploy(ctx, name, source_file, language):
    """Deploy smart contract"""
    async def _deploy():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            # Read source code
            source_path = Path(source_file)
            if not source_path.exists():
                click.echo(f"Source file not found: {source_file}", err=True)
                return False
            
            with open(source_path, 'r', encoding='utf-8') as f:
                source_code = f.read()
            
            async with ZhtpClient(config) as client:
                contract_address = await client.contracts.deploy_contract(
                    name=name,
                    source_code=source_code,
                    language=language
                )
                
                click.echo(f"Deployed contract: {name}")
                click.echo(f"Address: {contract_address}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_deploy())
    if not success:
        exit(1)

@contract.command()
@click.argument('address')
@click.argument('method')
@click.option('--params', help='Method parameters (JSON)')
@click.pass_context
def call(ctx, address, method, params):
    """Call smart contract method"""
    async def _call():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            # Parse parameters
            parameters = {}
            if params:
                try:
                    parameters = json.loads(params)
                except json.JSONDecodeError as e:
                    click.echo(f"Invalid JSON parameters: {e}", err=True)
                    return False
            
            async with ZhtpClient(config) as client:
                result = await client.contracts.call_method(
                    contract_address=address,
                    method=method,
                    parameters=parameters
                )
                
                click.echo(f"Contract call result:")
                click.echo(json.dumps(result, indent=2))
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_call())
    if not success:
        exit(1)

@cli.command()
@click.pass_context
def status(ctx):
    """Show network status"""
    async def _status():
        try:
            from .client import ZhtpConfig
            config = ZhtpConfig(
                network=ctx.obj['network'],
                privacy_level=ctx.obj['privacy']
            )
            
            async with ZhtpClient(config) as client:
                stats = await client.network.get_network_stats()
                
                click.echo(f"Network: {ctx.obj['network']}")
                click.echo(f"Privacy Level: {ctx.obj['privacy']}")
                click.echo(f"Connected Peers: {stats.connected_peers}")
                click.echo(f"Total Peers: {stats.total_peers}")
                click.echo(f"Uptime: {stats.uptime:.1f}s")
                click.echo(f"Messages Sent: {stats.messages_sent}")
                click.echo(f"Messages Received: {stats.messages_received}")
                
        except ZhtpError as e:
            click.echo(f"Error: {e}", err=True)
            return False
        
        return True
    
    success = asyncio.run(_status())
    if not success:
        exit(1)

def main():
    """Main CLI entry point"""
    cli()

if __name__ == '__main__':
    main()

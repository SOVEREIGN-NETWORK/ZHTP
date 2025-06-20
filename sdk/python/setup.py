"""
ZHTP Python SDK - Complete Decentralized Internet Replacement
Zero-Knowledge, Quantum-Resistant, Privacy-First Internet Protocol
"""

from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

with open("requirements.txt", "r", encoding="utf-8") as fh:
    requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="zhtp-sdk",
    version="1.0.0",
    author="ZHTP Network",
    author_email="sdk@zhtp.network",
    description="Complete decentralized internet replacement - no HTTP, DNS, or SSL dependencies",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/zhtp-network/zhtp-python-sdk",
    project_urls={
        "Documentation": "https://docs.zhtp.network/api/python/",
        "Source": "https://github.com/zhtp-network/zhtp-python-sdk",
        "Tracker": "https://github.com/zhtp-network/zhtp-python-sdk/issues",
    },
    packages=find_packages(exclude=["tests*", "examples*"]),
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Topic :: Internet :: WWW/HTTP :: Dynamic Content",
        "Topic :: Security :: Cryptography",
        "Topic :: System :: Distributed Computing",
        "Topic :: System :: Networking",
    ],
    python_requires=">=3.8",
    install_requires=requirements,
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-asyncio>=0.21.0",
            "black>=22.0.0",
            "flake8>=4.0.0",
            "mypy>=1.0.0",
            "sphinx>=5.0.0",
            "sphinx-rtd-theme>=1.0.0",
        ],
        "full": [
            "cryptography>=41.0.0",
            "aiofiles>=23.0.0",
            "websockets>=11.0.0",
            "pycryptodome>=3.18.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "zhtp=zhtp.cli:main",
            "zhtp-deploy=zhtp.deploy:main",
            "zhtp-node=zhtp.node:main",
        ],
    },
    keywords=[
        "zhtp", "decentralized", "blockchain", "zero-knowledge", "quantum-resistant",
        "privacy", "p2p", "distributed", "dapp", "smart-contracts", "anonymous",
        "no-http", "no-dns", "no-ssl", "replacement-internet"
    ],
    zip_safe=False,
    include_package_data=True,
)

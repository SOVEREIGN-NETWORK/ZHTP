# Contributing to ZHTP Python SDK

We welcome contributions to the ZHTP Python SDK! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites
- Python 3.8 or higher
- Git
- Virtual environment tool (venv, conda, etc.)

### Development Setup
```bash
# Clone the repository
git clone https://github.com/zhtp-network/zhtp-python-sdk.git
cd zhtp-python-sdk

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install development dependencies
pip install -e .[dev]

# Run tests
pytest

# Run linting
flake8 zhtp/
black zhtp/
mypy zhtp/
```

## How to Contribute

### 1. Reporting Issues
- Use GitHub Issues to report bugs or request features
- Provide detailed information about the issue
- Include code examples and error messages
- Specify your Python version and operating system

### 2. Submitting Code Changes

#### Before You Start
- Check existing issues and pull requests
- Discuss major changes in an issue first
- Fork the repository and create a feature branch

#### Development Process
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make your changes
# Add tests for new functionality
# Update documentation if needed

# Run tests and linting
pytest
flake8 zhtp/
black zhtp/
mypy zhtp/

# Commit your changes
git add .
git commit -m "feat: add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Create pull request
```

### 3. Code Style Guidelines

#### Python Code Style
- Follow PEP 8 style guidelines
- Use Black for code formatting
- Use type hints for all public functions
- Write descriptive variable and function names
- Add docstrings to all public classes and methods

#### Example Code Style
```python
"""
Module docstring describing the purpose of the module.
"""

import asyncio
from typing import Dict, Any, Optional

class ExampleClass:
    """
    Example class with proper documentation.
    
    Args:
        param1: Description of parameter
        param2: Description of parameter
    """
    
    def __init__(self, param1: str, param2: Optional[int] = None):
        self.param1 = param1
        self.param2 = param2
    
    async def example_method(self, data: Dict[str, Any]) -> bool:
        """
        Example method with proper type hints and documentation.
        
        Args:
            data: Dictionary containing input data
            
        Returns:
            bool: True if operation successful
            
        Raises:
            ValueError: If data is invalid
        """
        if not data:
            raise ValueError("Data cannot be empty")
        
        # Implementation here
        return True
```

### 4. Testing Guidelines

#### Writing Tests
- Write unit tests for all new functionality
- Use pytest for testing framework
- Aim for high test coverage (>90%)
- Test both success and error cases
- Use mock objects for external dependencies

#### Test Structure
```python
import pytest
from unittest.mock import AsyncMock, patch
from zhtp import ZhtpClient
from zhtp.exceptions import ZhtpError

class TestZhtpClient:
    """Test cases for ZhtpClient class."""
    
    @pytest.fixture
    async def client(self):
        """Create test client."""
        client = ZhtpClient()
        yield client
        await client.disconnect()
    
    async def test_connect_success(self, client):
        """Test successful connection."""
        result = await client.connect()
        assert result is True
        assert client.connected is True
    
    async def test_connect_failure(self, client):
        """Test connection failure."""
        with patch.object(client.crypto, 'initialize', side_effect=Exception("Connection failed")):
            with pytest.raises(ZhtpError):
                await client.connect()
```

### 5. Documentation Guidelines

#### Code Documentation
- Write clear docstrings for all public APIs
- Include parameter descriptions and return values
- Provide usage examples in docstrings
- Document exceptions that may be raised

#### README and Guides
- Update README.md for significant changes
- Add examples for new features
- Update migration guides when APIs change
- Keep documentation up to date with code

### 6. Pull Request Guidelines

#### PR Description
- Provide clear description of changes
- Reference related issues
- List breaking changes if any
- Include screenshots for UI changes

#### PR Checklist
- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] Code follows style guidelines
- [ ] No breaking changes (or clearly documented)
- [ ] Changelog updated
- [ ] Type hints added
- [ ] Examples updated if needed

## Architecture Guidelines

### Design Principles
- **Privacy First**: All operations should be privacy-preserving by default
- **Quantum Resistant**: Use post-quantum cryptographic algorithms
- **Decentralized**: Avoid centralized dependencies
- **Developer Friendly**: Provide simple, intuitive APIs
- **Async First**: Use async/await for all network operations

### Code Organization
```
zhtp/
├── __init__.py          # Main exports
├── client.py            # Main client class
├── dns.py              # Blockchain DNS
├── certificates.py     # Certificate authority
├── routing.py          # Anonymous routing
├── storage.py          # Decentralized storage
├── contracts.py        # Smart contracts
├── crypto.py           # Cryptographic operations
├── network.py          # Network management
├── dapps.py            # DApp management
├── cli.py              # Command line interface
└── exceptions.py       # Exception classes
```

### Adding New Features

#### Core Components
- Add new components to the appropriate module
- Follow existing patterns and interfaces
- Maintain backward compatibility
- Add comprehensive tests

#### Extension Points
- Use plugin architecture for extensibility
- Provide clear interfaces for custom implementations
- Allow configuration of components
- Support multiple backends where appropriate

## 🚨 Security Considerations

### Security Guidelines
- Never store private keys in plain text
- Always validate input parameters
- Use secure random number generation
- Implement proper error handling
- Follow cryptographic best practices

### Reporting Security Issues
- Report security vulnerabilities privately
- Email security@zhtp.network for sensitive issues
- Provide detailed reproduction steps
- Allow time for security fixes before disclosure

## 📄 License

By contributing to the ZHTP Python SDK, you agree that your contributions will be licensed under the MIT License.

## 🤝 Community

### Getting Help
- GitHub Discussions for general questions
- GitHub Issues for bug reports
- Discord for real-time chat
- Documentation for guides and references

### Code of Conduct
- Be respectful and inclusive
- Help others learn and grow
- Focus on constructive feedback
- Celebrate diversity and different perspectives

## 🏆 Recognition

We appreciate all contributions! Contributors will be:
- Listed in the project's contributors list
- Mentioned in release notes for significant contributions
- Invited to join the ZHTP developer community
- Eligible for contributor rewards and recognition

Thank you for helping make the decentralized internet a reality!

# Blockchain Unleashed - Rust Implementation

A simple blockchain implementation in Rust, translated from a JavaScript example. This project demonstrates core blockchain concepts including:

- **Blockchain Core**: Block creation, hashing, and validation
- **Storage System**: Data storage with optional encryption
- **Smart Contracts**: Executable contracts with data access
- **Cryptography**: RSA key generation and encryption/decryption

## Features

- **SHA-256 Hashing**: Secure block hashing using SHA-256
- **RSA Encryption**: Public/private key encryption for data
- **Smart Contracts**: Executable contracts with storage access
- **Data Validation**: Blockchain-based data integrity verification
- **Async Operations**: Full async/await support with Tokio

## Project Structure

```
src/
├── main.rs          # Main application with Speed data example
├── lib.rs           # Module definitions
├── blockchain.rs    # Core blockchain functionality
├── storage.rs       # Data storage with encryption
├── contract.rs      # Smart contract execution
├── keys.rs          # RSA key management
└── utils.rs         # File system utilities
```

## Usage

1. **Build the project**:
   ```bash
   cargo build
   ```

2. **Run the example**:
   ```bash
   cargo run
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

## Example Output

The application demonstrates:
- Creating a blockchain
- Storing Speed records (value, unit, precision)
- Executing smart contracts to modify data
- Validating blockchain integrity

## Dependencies

- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `sha2`: SHA-256 hashing
- `rsa`: RSA encryption
- `chrono`: Timestamp handling
- `anyhow`: Error handling

## Architecture

The system follows a modular design where:
- **Blockchain** manages the chain of blocks
- **Storage** handles data persistence with blockchain validation
- **Smart Contracts** execute business logic with data access
- **Keys** manage cryptographic operations

This implementation provides a solid foundation for understanding blockchain concepts in Rust. 
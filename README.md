# Blockchain File Storage in Rust 🦀⛓️

A decentralized file storage system leveraging blockchain technology for immutable, verifiable file storage.


## Features ✨
- **File Chunking**: Split files into 256KB chunks
- **Merkle Tree Verification**: Ensure file integrity through cryptographic proofs
- **Immutable Ledger**: Blockchain-based metadata storage
- **SHA-256 Hashing**: Secure cryptographic hashing for data blocks
- **File Verification**: Validate stored files against blockchain records

## Installation ⚙️

### Prerequisites
- Rust 1.60+
- Cargo

```bash
cd blockchain-file-storage

# Build project
cargo build --release

src/
├── main.rs          # CLI interface
├── blockchain.rs    # Blockchain implementation
├── merkle.rs        # Merkle tree operations
├── storage.rs       # File chunk storage
└── crypto.rs        # Cryptographic utilities

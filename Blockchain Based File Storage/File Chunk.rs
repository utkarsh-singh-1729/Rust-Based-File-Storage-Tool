use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::fs;
use std::path::Path;
use merkletree::MerkleTree;

// File chunk structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileChunk {
    data: Vec<u8>,
    hash: String,
}

// File metadata stored in blockchain
#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    filename: String,
    chunks: Vec<String>, // Merkle tree leaf hashes
}

// Block structure
#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    file_metadata: FileMetadata,
    merkle_root: String,
    previous_hash: String,
    hash: String,
}

// Blockchain-based file storage
#[derive(Debug)]
struct FileStorageBlockchain {
    chain: Vec<Block>,
    stored_files: Vec<FileChunk>,
}

impl FileStorageBlockchain {
    // Initialize blockchain with genesis block
    fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            file_metadata: FileMetadata {
                filename: "genesis".to_string(),
                chunks: vec![],
            },
            merkle_root: "0".to_string(),
            previous_hash: "0".to_string(),
            hash: "genesis_hash".to_string(),
        };

        FileStorageBlockchain {
            chain: vec![genesis_block],
            stored_files: vec![],
        }
    }

    // Add file to blockchain and storage
    fn add_file(&mut self, path: &str) {
        let file_data = fs::read(path).unwrap();
        let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

        // Split file into 256KB chunks
        let chunk_size = 256 * 1024;
        let chunks: Vec<FileChunk> = file_data.chunks(chunk_size)
            .map(|chunk| {
                let mut hasher = Sha256::new();
                hasher.update(chunk);
                let hash = format!("{:x}", hasher.finalize_reset());
                
                FileChunk {
                    data: chunk.to_vec(),
                    hash: hash.clone(),
                }
            })
            .collect();

        // Build Merkle tree from chunk hashes
        let leaf_hashes: Vec<Vec<u8>> = chunks.iter()
            .map(|c| hex::decode(&c.hash).unwrap())
            .collect();

        let tree = MerkleTree::from_vec(leaf_hashes);
        let merkle_root = hex::encode(tree.root());

        // Create new block
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block {
            index: previous_block.index + 1,
            timestamp: Utc::now().timestamp(),
            file_metadata: FileMetadata {
                filename: filename.to_string(),
                chunks: chunks.iter().map(|c| c.hash.clone()).collect(),
            },
            merkle_root,
            previous_hash: previous_block.hash.clone(),
            hash: String::new(),
        };

        new_block.hash = self.calculate_block_hash(&new_block);
        self.chain.push(new_block);
        self.stored_files.extend(chunks);
    }

    fn calculate_block_hash(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(block.index.to_string());
        hasher.update(block.timestamp.to_string());
        hasher.update(&block.merkle_root);
        hasher.update(&block.previous_hash);
        format!("{:x}", hasher.finalize())
    }

    // Verify file integrity using Merkle root
    fn verify_file(&self, filename: &str) -> bool {
        if let Some(block) = self.chain.iter().find(|b| b.file_metadata.filename == filename) {
            let stored_chunks: Vec<&FileChunk> = self.stored_files.iter()
                .filter(|c| block.file_metadata.chunks.contains(&c.hash))
                .collect();

            let leaf_hashes: Vec<Vec<u8>> = stored_chunks.iter()
                .map(|c| hex::decode(&c.hash).unwrap())
                .collect();

            let tree = MerkleTree::from_vec(leaf_hashes);
            hex::encode(tree.root()) == block.merkle_root
        } else {
            false
        }
    }
}

fn main() {
    let mut storage = FileStorageBlockchain::new();
    
    // Store a file
    storage.add_file("example.txt");
    
    // Verify file integrity
    let valid = storage.verify_file("example.txt");
    println!("File integrity valid: {}", valid);
    
    println!("Blockchain: {:#?}", storage.chain);
}
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

// Block structure
#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String, // Could represent file metadata or hash
    previous_hash: String,
    hash: String,
}

impl Block {
    // Create a new block
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    // Calculate SHA-256 hash of the block
    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}

// Simple blockchain
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // Initialize with genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis File Data".to_string(), "0".to_string());
        Blockchain { chain: vec![genesis_block] }
    }

    // Add a new block (e.g., storing file metadata)
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("File Hash: QmXyZ...".to_string());
    blockchain.add_block("File Hash: AbC1...".to_string());
    println!("Blockchain: {:?}", blockchain.chain);
}
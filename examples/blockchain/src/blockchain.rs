use crate::utils;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs;

const BLOCKCHAIN_FILE: &str = ".blockchain.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub prev: String,
    pub timestamp: i64,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChainInfo {
    tail_hash: String,
    next_id: u64,
}

#[derive(Clone)]
pub struct Blockchain {
    path: String,
    tail_hash: String,
    next_id: u64,
}

impl Blockchain {
    pub async fn new(base_path: &str) -> Result<Self> {
        let path = base_path.to_string();
        let blockchain_file = Path::new(&path).join(BLOCKCHAIN_FILE);
        
        utils::ensure_directory(Path::new(&path)).await?;
        
        if !utils::exists(&blockchain_file).await {
            // Initialize new blockchain
            let mut chain = Blockchain {
                path,
                tail_hash: String::new(),
                next_id: 0,
            };
            
            let block = Block {
                id: "0".to_string(),
                prev: "0".to_string(),
                timestamp: Utc::now().timestamp(),
                data: serde_json::Value::Object(serde_json::Map::new()),
            };
            
            chain.tail_hash = chain.write_block(&block).await?;
            chain.next_id = 1;
            chain.write_chain().await?;
            
            Ok(chain)
        } else {
            // Load existing blockchain
            let content = fs::read_to_string(&blockchain_file).await?;
            let chain_info: ChainInfo = serde_json::from_str(&content)?;
            
            Ok(Blockchain {
                path,
                tail_hash: chain_info.tail_hash,
                next_id: chain_info.next_id,
            })
        }
    }

    async fn write_chain(&self) -> Result<()> {
        let chain_info = ChainInfo {
            tail_hash: self.tail_hash.clone(),
            next_id: self.next_id,
        };
        
        let file_path = Path::new(&self.path).join(BLOCKCHAIN_FILE);
        let content = serde_json::to_string_pretty(&chain_info)?;
        fs::write(file_path, content).await?;
        
        Ok(())
    }

    pub async fn add_block(&mut self, data: serde_json::Value) -> Result<(u64, String)> {
        let id = self.next_id;
        let prev = self.tail_hash.clone();
        let timestamp = Utc::now().timestamp();
        
        let block = Block {
            id: id.to_string(),
            prev,
            timestamp,
            data,
        };
        
        let hash = self.write_block(&block).await?;
        self.next_id += 1;
        self.tail_hash = hash.clone();
        self.write_chain().await?;
        
        Ok((id, hash))
    }

    async fn write_block(&self, block: &Block) -> Result<String> {
        let hash = calculate_hash(block);
        let file_path = Path::new(&self.path).join(format!("{}.json", hash));
        let content = serde_json::to_string_pretty(block)?;
        fs::write(file_path, content).await?;
        
        Ok(hash)
    }

    pub async fn read_block(&self, hash: &str) -> Result<Block> {
        let file_path = Path::new(&self.path).join(format!("{}.json", hash));
        
        if !utils::exists(&file_path).await {
            return Err(anyhow::anyhow!("Block file not found: {}", file_path.display()));
        }
        
        let content = fs::read_to_string(file_path).await?;
        let block: Block = serde_json::from_str(&content)?;
        
        Ok(block)
    }

    pub async fn is_valid(&self) -> Result<bool> {
        self.is_valid_with_limit(None).await
    }

    pub async fn is_valid_with_limit(&self, limit: Option<usize>) -> Result<bool> {
        let mut current_hash = self.tail_hash.clone();
        let mut count = 0;
        
        while !current_hash.is_empty() && current_hash != "0" {
            let block = self.read_block(&current_hash).await?;
            let expected_hash = calculate_hash(&block);
            let valid = current_hash == expected_hash;
            
            println!("{} {}", if valid { "✅" } else { "❌" }, expected_hash);
            
            if !valid {
                return Ok(false);
            }
            
            count += 1;
            if let Some(limit_val) = limit {
                if count >= limit_val {
                    break;
                }
            }
            
            if block.prev == "0" {
                break;
            }
            
            current_hash = block.prev;
        }
        
        Ok(true)
    }
}

pub fn calculate_hash(block: &Block) -> String {
    let block_str = serde_json::to_string(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(block_str.as_bytes());
    hex::encode(hasher.finalize())
} 
use crate::{blockchain::calculate_hash, keys::Keys, utils};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StorageEntry {
    data: serde_json::Value,
    encrypted: bool,
    timestamp: i64,
    block: String,
}

pub struct Storage {
    base_path: String,
    keys: Keys,
    pub chain: crate::blockchain::Blockchain,
}

impl Storage {
    pub async fn new(base_path: &str, chain: crate::blockchain::Blockchain, keys: Keys) -> Result<Self> {
        utils::ensure_directory(Path::new(base_path)).await?;
        
        Ok(Storage {
            base_path: base_path.to_string(),
            keys,
            chain,
        })
    }

    pub async fn save_data<T: Serialize>(&self, id: u64, data: &T, encrypted: bool) -> Result<()> {
        let record = if encrypted {
            let encrypted_data = crate::keys::encrypt(data, &self.keys.public_key)?;
            serde_json::Value::String(encrypted_data)
        } else {
            serde_json::to_value(data)?
        };

        let timestamp = chrono::Utc::now().timestamp();
        let hash = calculate_hash(&crate::blockchain::Block {
            id: id.to_string(),
            prev: "".to_string(),
            timestamp,
            data: record.clone(),
        });

        let mut chain = self.chain.clone();
        let block = chain.add_block(serde_json::json!({
            "id": id,
            "hash": hash
        })).await?;

        let entry = StorageEntry {
            data: record,
            encrypted,
            timestamp,
            block: block.1,
        };

        let file_path = Path::new(&self.base_path).join(format!("{}.json", id));
        let content = serde_json::to_string_pretty(&entry)?;
        fs::write(file_path, content).await?;

        Ok(())
    }

    pub async fn load_data<T: for<'de> Deserialize<'de>>(&self, id: u64) -> Result<Option<T>> {
        let file_path = Path::new(&self.base_path).join(format!("{}.json", id));
        
        if !utils::exists(&file_path).await {
            return Ok(None);
        }

        let content = fs::read_to_string(file_path).await?;
        let entry: StorageEntry = serde_json::from_str(&content)?;
        
        let is_valid = self.validate(id, &entry.data, &entry.block).await?;
        if !is_valid {
            return Err(anyhow::anyhow!("Storage record {} is invalid", id));
        }

        let data = if entry.encrypted {
            crate::keys::decrypt::<T>(&entry.data.as_str().unwrap(), &self.keys.private_key)?
        } else {
            serde_json::from_value(entry.data)?
        };

        Ok(Some(data))
    }

    async fn validate(&self, id: u64, data: &serde_json::Value, block_hash: &str) -> Result<bool> {
        match self.chain.read_block(block_hash).await {
            Ok(block) => {
                let expected_hash = calculate_hash(&crate::blockchain::Block {
                    id: id.to_string(),
                    prev: "".to_string(),
                    timestamp: block.timestamp,
                    data: data.clone(),
                });
                
                if let Some(block_data) = block.data.as_object() {
                    if let (Some(block_id), Some(block_hash_val)) = (
                        block_data.get("id").and_then(|v| v.as_u64()),
                        block_data.get("hash").and_then(|v| v.as_str())
                    ) {
                        return Ok(block_id == id && block_hash_val == expected_hash);
                    }
                }
                Ok(false)
            }
            Err(_) => Ok(false),
        }
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self {
            base_path: self.base_path.clone(),
            keys: self.keys.clone(),
            chain: self.chain.clone(),
        }
    }
} 
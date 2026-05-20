pub mod blockchain;
pub mod contract;
pub mod keys;
pub mod storage;
pub mod utils;

pub use blockchain::Blockchain;
pub use contract::{DataReader, SmartContract};
pub use keys::{load_keys, Keys};
pub use storage::Storage;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        value: i32,
        message: String,
    }

    #[tokio::test]
    async fn test_blockchain_basic_functionality() {
        // Test blockchain creation and validation
        let chain = Blockchain::new("./test_blockchain").await.unwrap();
        assert!(chain.is_valid().await.unwrap());

        // Test adding blocks
        let mut chain = Blockchain::new("./test_blockchain2").await.unwrap();
        let data = serde_json::json!({"test": "data"});
        let (id, hash) = chain.add_block(data).await.unwrap();
        assert_eq!(id, 1);
        assert!(!hash.is_empty());

        // Test reading blocks
        let block = chain.read_block(&hash).await.unwrap();
        assert_eq!(block.id, "1");
    }

    #[tokio::test]
    async fn test_storage_and_contracts() {
        let keys = load_keys("./test_keys").await.unwrap();
        let chain = Blockchain::new("./test_blockchain3").await.unwrap();
        let storage = Storage::new("./test_storage", chain.clone(), keys).await.unwrap();

        // Test data storage and retrieval
        let test_data = TestData {
            value: 42,
            message: "Hello, Blockchain!".to_string(),
        };

        storage.save_data(1, &test_data, false).await.unwrap();
        let retrieved: Option<TestData> = storage.load_data(1).await.unwrap();
        assert_eq!(retrieved, Some(test_data));

        // Test smart contract
        let proc = |reader: &DataReader, _args: &HashMap<String, serde_json::Value>| {
            let reader = reader.clone();
            async move {
                let data: TestData = reader.get(1).await.unwrap().unwrap();
                let updated = TestData {
                    value: data.value * 2,
                    message: data.message,
                };
                Ok(serde_json::to_value(updated).unwrap())
            }
        };

        let contract = SmartContract::new("Test Contract", storage.clone(), proc);
        let args = HashMap::new();
        let result = contract.execute(args).await.unwrap();
        let updated_data: TestData = serde_json::from_value(result).unwrap();
        assert_eq!(updated_data.value, 84);
    }
} 
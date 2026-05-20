use crate::Storage;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DataReader {
    storage: Storage,
}

impl DataReader {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub async fn get<T: for<'de> serde::Deserialize<'de>>(&self, id: u64) -> Result<Option<T>> {
        self.storage.load_data(id).await
    }
}

pub struct SmartContract {
    name: String,
    storage: Storage,
    proc: Box<dyn Fn(&DataReader, &HashMap<String, Value>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value>> + Send>> + Send + Sync>,
}

impl SmartContract {
    pub fn new<F, Fut>(name: &str, storage: Storage, proc: F) -> Self
    where
        F: Fn(&DataReader, &HashMap<String, Value>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value>> + Send + 'static,
    {
        let proc = Box::new(move |reader: &DataReader, args: &HashMap<String, Value>| {
            Box::pin(proc(reader, args)) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value>> + Send>>
        });

        Self {
            name: name.to_string(),
            storage,
            proc,
        }
    }

    pub async fn execute(&self, args: HashMap<String, Value>) -> Result<serde_json::Value> {
        let reader = DataReader::new(self.storage.clone());
        
        match (self.proc)(&reader, &args).await {
            Ok(result) => {
                if let Some(id) = args.get("id").and_then(|v| v.as_u64()) {
                    self.storage.save_data(id, &result, false).await?;
                }
                Ok(result)
            }
            Err(error) => {
                let contract_data = serde_json::json!({
                    "contract": self.name,
                    "args": args,
                    "error": error.to_string(),
                    "timestamp": chrono::Utc::now().timestamp()
                });
                
                let mut chain = self.storage.chain.clone();
                chain.add_block(contract_data).await?;
                
                Err(error)
            }
        }
    }
} 
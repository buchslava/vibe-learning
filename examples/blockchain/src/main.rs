use blockchain_unleashed::{
    blockchain::Blockchain,
    contract::SmartContract,
    keys::load_keys,
    storage::Storage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Speed {
    value: f64,
    unit: String,
    precision: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let keys = load_keys("./keys").await?;

    let chain = Blockchain::new("./blockchain").await?;
    let ready = chain.is_valid().await?;
    println!("🕵️  Blockchain valid: {}", ready);

    let storage = Storage::new("./storage", chain.clone(), keys).await?;

    let record1 = Speed {
        value: 13.5,
        unit: "m/s".to_string(),
        precision: 0.1,
    };
    let record2 = Speed {
        value: 13.6,
        unit: "m/s".to_string(),
        precision: 0.01,
    };
    let record3 = Speed {
        value: 13.2,
        unit: "m/s".to_string(),
        precision: 0.001,
    };
    let record4 = Speed {
        value: 13.4,
        unit: "m/s".to_string(),
        precision: 0.1,
    };

    storage.save_data(100, &record1, false).await?;
    storage.save_data(100, &record2, false).await?;
    storage.save_data(101, &record3, false).await?;
    storage.save_data(101, &record4, false).await?;

    let proc = |reader: &blockchain_unleashed::contract::DataReader, args: &HashMap<String, serde_json::Value>| {
        let reader = reader.clone();
        let args = args.clone();
        async move {
            println!("🗃️  Smart contract called with args: {:?}", args);
            
            let id = args.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
            let record: Speed = reader.get(id).await?.ok_or_else(|| anyhow::anyhow!("No record found"))?;
            println!("🗃️  Data loaded from storage: {:?}", record);
            
            let coefficient = args.get("coefficient").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let value = (record.value * coefficient).round() / (1.0 / record.precision);
            println!("🗃️  Smart Contract record update: {{ value: {} }}", value);
            
            Ok(serde_json::to_value(Speed {
                value,
                unit: record.unit,
                precision: record.precision,
            })?)
        }
    };

    let contract = SmartContract::new("Contract 1", storage.clone(), proc);

    let mut args = HashMap::new();
    args.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(100)));
    args.insert("coefficient".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(2.5).unwrap()));

    if let Err(error) = contract.execute(args).await {
        eprintln!("Contract failed: {}", error);
    }

    let valid = chain.is_valid_with_limit(Some(5)).await?;
    println!("🕵️  Blockchain valid after adding: {}", valid);

    Ok(())
} 
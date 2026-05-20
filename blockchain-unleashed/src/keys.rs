use crate::utils;
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    pub public_key: String,
    pub private_key: String,
}

impl Keys {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            public_key,
            private_key,
        }
    }
}

pub async fn generate_keys() -> Result<Keys> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = RsaPublicKey::from(&private_key);
    
    let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF)?.to_string();
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
    
    Ok(Keys::new(public_key_pem, private_key_pem))
}

pub fn encrypt<T: Serialize>(data: &T, public_key_pem: &str) -> Result<String> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;
    let data_json = serde_json::to_string(data)?;
    let data_bytes = data_json.as_bytes();
    
    let encrypted = public_key.encrypt(&mut rand::thread_rng(), rsa::Pkcs1v15Encrypt, data_bytes)?;
    let encoded = general_purpose::STANDARD.encode(encrypted);
    
    Ok(encoded)
}

pub fn decrypt<T: for<'de> Deserialize<'de>>(encrypted_data: &str, private_key_pem: &str) -> Result<T> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)?;
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_data)?;
    
    let decrypted = private_key.decrypt(rsa::Pkcs1v15Encrypt, &encrypted_bytes)?;
    let decrypted_str = String::from_utf8(decrypted)?;
    let data: T = serde_json::from_str(&decrypted_str)?;
    
    Ok(data)
}

pub async fn load_keys(base_path: &str) -> Result<Keys> {
    let public_key_path = Path::new(base_path).join("public.pem");
    let private_key_path = Path::new(base_path).join("private.pem");
    
    utils::ensure_directory(Path::new(base_path)).await?;
    
    if utils::exists(&public_key_path).await && utils::exists(&private_key_path).await {
        println!("🔑 Loading keys...");
        let public_key = fs::read_to_string(&public_key_path).await?;
        let private_key = fs::read_to_string(&private_key_path).await?;
        Ok(Keys::new(public_key, private_key))
    } else {
        println!("🔑 Generating keys...");
        let keys = generate_keys().await?;
        
        fs::write(&public_key_path, &keys.public_key).await?;
        fs::write(&private_key_path, &keys.private_key).await?;
        
        Ok(keys)
    }
} 
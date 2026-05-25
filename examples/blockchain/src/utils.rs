use std::path::Path;
use tokio::fs;

pub async fn exists(path: &Path) -> bool {
    fs::metadata(path).await.is_ok()
}

pub async fn ensure_directory(path: &Path) -> anyhow::Result<()> {
    if !exists(path).await {
        fs::create_dir_all(path).await?;
    }
    Ok(())
} 
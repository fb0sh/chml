use anyhow::Context;
use chml_api::res::{ApiResponse, ApiResult};

use crate::config;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

pub const FRPC_DOWNLOAD_URL: &str = "https://cf-v1.uapis.cn/download/frpc/frpc_info.json";

#[derive(Debug, Deserialize)]
pub struct FrpcRelease {
    pub version: String,
    pub downloads: Vec<FrpcDownload>,
}

#[derive(Debug, Deserialize)]
pub struct FrpcDownload {
    pub os: String,
    pub arch: String,
    pub link: String,
    pub platform: String,
    pub size: u64,
    pub hash: String,
    pub hash_type: String,
}

fn pick_download<'a>(downloads: &'a [FrpcDownload]) -> anyhow::Result<&'a FrpcDownload> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    downloads
        .iter()
        .find(|d| d.os == os && d.arch == arch)
        .ok_or_else(|| anyhow::anyhow!("no frpc binary for {} {}", os, arch))
}

pub async fn download_fpc_client(fpc_client_path: &Path) -> anyhow::Result<()> {
    let release = reqwest::get(FRPC_DOWNLOAD_URL)
        .await?
        .json::<ApiResponse<FrpcRelease>>()
        .await?
        .data
        .unwrap();

    let download = pick_download(&release.downloads)?;
    let bin_path = fpc_client_path;
    println!("[*] Downloading frpc to {}", &bin_path.display());

    download_file(&download.link, &bin_path).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bin_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms).await?;
    }

    Ok(())
}

async fn download_file(url: &str, target: &std::path::Path) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;

    let mut file = fs::File::create(target).await?;
    file.write_all(&bytes).await?;
    Ok(())
}

#[tokio::test]
async fn test_download_fpc_client_async() {
    let app_home = config::AppHome::new("chml").unwrap();
    app_home.ensure().unwrap();
    let bin_dir = app_home.join_dir("bin").unwrap();
    let bin_path = bin_dir.join(config::bin_name());
    download_fpc_client(&bin_path).await.unwrap();
}

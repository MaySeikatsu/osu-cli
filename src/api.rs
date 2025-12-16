use crate::config::ConfigFile;
use anyhow::{Context, Result};
use rosu_v2::prelude::*;


pub async fn init_api(cfg: &ConfigFile) -> Result<Osu> { // <- Return the Osu instance!
    let client_id: u64 = cfg.api_client_id.into();
    let client_secret = &cfg.api_secret;
    let osu_api = Osu::new(client_id, client_secret).await.unwrap();
    Ok(osu_api) // Return it! (if Ok)
}

// use confy;

use rosu_v2::prelude::*;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    // println!("{:?}", cfg);
    println!("api key: {:?}", cfg.api);

    init_api().await;
    
    let scores = Scores::scores().await;

    println!("username: {:?}", &scores.input_username);
    println!("user_ext: {:?}", &scores.user_ext);

    Ok(())
}

async fn init_api() -> Osu { // <- Return the Osu instance!
    let client_id: u64 = 42909;
    let client_secret = String::from("rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh");
    let osu = Osu::new(client_id, client_secret).await.unwrap();
    osu // Return it!
}

struct Scores {
        input_username: String ,
        user_ext: String,
        // user_scores: String ,
        // _user_mania: String,
}
impl Scores {
    async fn scores () -> Self {
        Self {
            input_username: "mayseikatsu".to_string(), // borrow this value with & 
            user_ext: osu.user(&input_username).mode(GameMode::Osu).to_string().await,
            // user_scores: osu.user_scores(&input_username).mode(GameMode::Osu).to_string(),
            // _user_mania: osu.user(&input_username).mode(GameMode::Mania).to_string().await,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    name: String,
    api: String,
}
impl Default for ConfigFile {
    fn default () -> Self {
        Self {
            name: "mayseikatsu".to_string(),
            api: "number132".to_string(),
        }
    }
}

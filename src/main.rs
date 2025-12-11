// use confy;
use rosu_v2::prelude::*;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    name: String,
    gamemode: String, // maybe list?
    api_client_id: u32,
    api_secret: String,
}
impl Default for ConfigFile {
    fn default () -> Self {
        Self {
            name: "mayseikatsu".to_string(),
            gamemode: "Osu".to_string(),
            api_client_id: 42909,
            api_secret: "rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh".to_string(),
        }
    }
}

async fn init_api() -> Osu { // <- Return the Osu instance!
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!").unwrap(); // learn how to pass this "cfg" as an argument and not unwrap it

    let client_id: u64 = cfg.api_client_id.into();
    let client_secret = cfg.api_secret;
    let osu_api = Osu::new(client_id, client_secret).await.unwrap();
    osu_api // Return it!
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    println!("api id: {:?}", cfg.api_client_id);

    // init_api().await;
    let api_osu = init_api().await;
    let gamemode = if cfg.gamemode == "Osu" {
        GameMode::Osu
    } else if cfg.gamemode == "Mania" {
        GameMode::Mania
    } else if cfg.gamemode == "Taiko" {
        GameMode::Taiko
    } else if cfg.gamemode == "Catch"{
        GameMode::Catch// or handle error, e.g. return Err(...)
    } else {
        GameMode::Osu// or handle error, e.g. return Err(...)
    };

    // let userall = api_osu.user("mayseikatsu").await.unwrap();
    // println!("{:?}", userall);

    // let user_scores = api_osu.user_scores("mayseikatsu").mode(GameMode::Osu).await.unwrap();

    let user = api_osu.user(cfg.name).mode(gamemode).await?;
    // let user = api_osu.user("mayseikatsu").mode(GameMode::Osu).await?;
    println!("Highest Rank {:#?}", user.highest_rank.unwrap().rank);

    // let scores = Scores::scores().await;
    // println!("username: {:?}", &scores.input_username);
    // println!("user_ext: {:?}", &scores.user_ext);

    Ok(())
}

struct Scores {
        input_username: String ,
        // user_ext: String,
        // user_scores: String ,
        // _user_mania: String,
}
impl Scores {
    async fn scores () -> Self {
        Self {
            input_username: "mayseikatsu".to_string(), // borrow this value with & 
            // user_ext: osu.user(&input_username).mode(GameMode::Osu).to_string().await,
            // user_scores: osu.user_scores(&input_username).mode(GameMode::Osu).to_string(),
            // _user_mania: osu.user(&input_username).mode(GameMode::Mania).to_string().await,
        }
    }
}


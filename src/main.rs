mod api;
mod config;

use api::init_api;
use config::ConfigFile;

// use confy;
use rosu_v2::prelude::*;
use anyhow::{Context, Result};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    println!("api id: {:?}", cfg.api_client_id);

    // init_api().await;
    let api_osu = init_api(&cfg).await?;

enum Gamemode {
    String,
}
// TODO: Implement gamemode as an enum instead of if-else queue
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
    // let user_scores = api_osu.user_scores("mayseikatsu").mode(GameMode::Osu).await.unwrap();

    let user = api_osu.user(cfg.name).mode(gamemode).await?;     // basically this: let user = api_osu.user("mayseikatsu").mode(GameMode::Osu).await?;
    
    let statistics = match user.statistics {
        Some(x) => x,
        None => {
            println!("Error while reading statistics!");
            return Ok(());
        }
    };
    let global_rank = match statistics.global_rank {
        Some(x) => x,
        none => {
            println!("Error reading global rank!");
            return Ok(());
        }
    };
    let country_rank = match statistics.country_rank {
        Some(x) => x,
        none => {
            println!("Error reading country rank!");
            return Ok(());
        }
    };

    println!("Highest Rank {:#?}", user.highest_rank.unwrap().rank);
    println!("Current Rank {:#?}", global_rank);
    println!("Country Rank {:#?}", country_rank);

    // let scores = Scores::scores().await;
    // println!("username: {:?}", &scores.input_username);
    // println!("user_ext: {:?}", &scores.user_ext);

    print_table();

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

fn print_table() {
let table = vec![
    vec!["mayseikatsu".cell(), "User".cell(), "peppy".cell().justify(Justify::Right)],
    vec!["1203".cell(), "pp".cell(), "9001".cell().justify(Justify::Right)],
    vec!["504930".cell(), "rank".cell(), "1".cell().justify(Justify::Right)],
].table()
.title(vec![
    "User1".cell().bold(true),
    "Info".cell().bold(true),
    "User2".cell().bold(true),
]).bold(true);

assert!(print_stdout(table).is_ok());
}

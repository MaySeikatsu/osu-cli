mod api;
mod config;
mod stats;
mod output;

use api::init_api;
use config::{ConfigFile, GamemodeOptions};
use stats::{UserStats};
use output::stats_out;

// use confy;
use rosu_v2::prelude::*;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    println!("api id: {:?}", cfg.api_client_id);
    let output_stats = output::stats_out().await;


    // name is iterator value, not an actual var, could be named anything
    for name in &cfg.names {
        println!("{}", name);
    }
    //Emmi loop to iterate over cells
    /*
        let mut users = Vec::new();
        for name in &cfg.names {
            println!("{}", name);
            let user = api_osu.user(name).mode(gamemode).await?;     // basically this: let user = api_osu.user("mayseikatsu").mode(GameMode::Osu).await?;
            users.push(user);
        }
        let mut cell_names = Vec::new();
        let username_cell = Cell::new("Username").fg(Color::Blue);
        cell_names.push(username_cell);
        for name in &cfg.names {
            let cell = Cell::new(name).fg(Color::Blue);
            cell_names.push(cell);
        }

        let mut table = Table::new();
            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                // .load_preset(UTF8_FULL)
                .load_preset(UTF8_FULL_CONDENSED)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(cell_names);
        for user in users {
            let statistics =  user.statistics.expect("Error reading user statistics!");

            table.add_row(vec![
                Cell::new("Country"),
                Cell::new(user.country),
            ]);
            println!("{}", table);
        }
    */
    
    Ok(())
}

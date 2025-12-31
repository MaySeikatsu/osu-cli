mod api;
mod config;
mod models;

use api::init_api;
use config::{ConfigFile, GamemodeOptions};
use models::{UserFetcher, UserData};

// use confy;
use rosu_v2::prelude::*;
use anyhow::{Context, Result};
// use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use comfy_table::{Cell, Table, ContentArrangement};
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
// use comfy_table::*;

/// Extension trait to add to_title_case method to strings
trait StringExt {
    fn to_title_case(&self) -> String;
}

impl StringExt for str {
    fn to_title_case(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    println!("api id: {:?}", cfg.api_client_id);

    // Create user fetcher
    let user_fetcher = UserFetcher::new(&cfg).await?;
    
    // Fetch data for all users
    let users_data = user_fetcher.fetch_users(&cfg.names).await?;
    
    // Print some debug info
    for user_data in &users_data {
        println!("Fetched data for: {}", user_data.username);
    }

    // let scores = Scores::scores().await;
    // println!("username: {:?}", &scores.input_username);
    // println!("user_ext: {:?}", &scores.user_ext);

    // print_table();
    
    // Create table with dynamic columns based on number of users
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS);

    // Create header with "Data" column followed by each username
    let mut header = vec!["Data".to_string()];
    for user_data in &users_data {
        header.push(user_data.username.clone());
    }
    table.set_header(header);

    // Helper function to get a value from UserData or return empty string
    fn get_user_value(user_data: &UserData, field: &str) -> String {
        match field {
            "username" => user_data.username.clone(),
            "country" => user_data.country.clone().unwrap_or_default(),
            "global_rank" => user_data.global_rank.map(|r| r.to_string()).unwrap_or_default(),
            "country_rank" => user_data.country_rank.map(|r| r.to_string()).unwrap_or_default(),
            "pp" => user_data.pp.to_string(),
            "accuracy" => user_data.accuracy.to_string(),
            "level" => user_data.level.to_string(),
            "max_combo" => user_data.max_combo.to_string(),
            "playcount" => user_data.playcount.to_string(),
            "playtime" => user_data.playtime.to_string(),
            "join_date" => user_data.join_date.date().to_string(),
            "last_visit" => user_data.last_visit.map(|d| d.date().to_string()).unwrap_or_default(),
            "user_id" => user_data.user_id.to_string(),
            "country_code" => user_data.country_code.clone(),
            "is_active" => user_data.is_active.to_string(),
            "is_online" => user_data.is_online.to_string(),
            "is_supporter" => user_data.is_supporter.to_string(),
            "has_supported" => user_data.is_supporter.to_string(),
            "total_hits" => user_data.total_hits.to_string(),
            "ranked_score" => user_data.ranked_score.to_string(),
            "total_score" => user_data.total_score.to_string(),
            _ => "".to_string(),
        }
    }

    // Add rows for each data field
    let fields = vec![
        "username", "country", "global_rank", "pp", "accuracy", "", 
        "level", "max_combo", "playcount", "playtime", "join_date", "last_visit",
        "", "user_id", "country_code", "country_rank", "", "is_active", "is_online",
        "is_supporter", "has_supported", "", "total_hits", "ranked_score", "total_score"
    ];

    for field in fields {
        if field.is_empty() {
            table.add_row(vec![""; users_data.len() + 1]);
            continue;
        }

        let mut row = vec![match field {
            "global_rank" => "Current Rank".to_string(),
            "country_rank" => "Country Rank".to_string(),
            "join_date" => "Join Date".to_string(),
            "last_visit" => "Last Seen".to_string(),
            "user_id" => "UserID".to_string(),
            "country_code" => "Country Code".to_string(),
            "is_active" => "Is Active".to_string(),
            "is_online" => "Is Online".to_string(),
            "is_supporter" => "Osu Supporter".to_string(),
            "has_supported" => "Was Supporter".to_string(),
            "total_hits" => "Total Hits".to_string(),
            "ranked_score" => "Ranked Score".to_string(),
            "total_score" => "Total Score".to_string(),
            _ => field.replace('_', " ").to_title_case(),
        }];

        for user_data in &users_data {
            row.push(get_user_value(user_data, field));
        }

        table.add_row(row);
    }

    // table.add_row(vec!["Bob", "900", "2"]);
    // table.add_row(vec!["Charlie", "800", "3"]);
    // table.add_row_if(vec!["Charlie", "800", "3"]);

    println!("{}", table);

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
    // fn print_table() {
    //     let table = vec![
    //         vec!["mayseikatsu".cell(), "User".cell(), "peppy".cell().justify(Justify::Right)],
    //         vec!["1203".cell(), "pp".cell(), "9001".cell().justify(Justify::Right)],
    //         vec!["504930".cell(), "rank".cell(), "1".cell().justify(Justify::Right)],
    //     ].table()
    //     .title(vec![
    //         "User1".cell().bold(true),
    //         "Info".cell().bold(true),
    //         "User2".cell().bold(true),
    //     ]).bold(true);
    //
    //     assert!(print_stdout(table).is_ok());
    // }

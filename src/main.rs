mod api;
mod config;

use api::init_api;
use config::{ConfigFile, GamemodeOptions};

// use confy;
use rosu_v2::prelude::*;
use anyhow::{Context, Result};
// use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use comfy_table::{Cell, Table, ContentArrangement};
use comfy_table::presets::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    println!("api id: {:?}", cfg.api_client_id);

    // init_api().await;
    let api_osu = init_api(&cfg).await?;

// TODO: Implement gamemode as an enum instead of if-else queue
    
    let gamemode = match cfg.gamemode {
        GamemodeOptions::Osu => GameMode::Osu,
        GamemodeOptions::Mania => GameMode::Mania,
        GamemodeOptions::Taiko=> GameMode::Taiko,
        GamemodeOptions::Catch=> GameMode::Catch,
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
        None => {
            println!("Error reading global rank!");
            return Ok(());
        }
    };
    let country_rank = match statistics.country_rank {
        Some(x) => x,
        None => {
            println!("Error reading country rank!");
            return Ok(());
        }
    };

    // println!("UserID {:#?}", user.user_id);
    // println!("Username {:#?}", user.username);
    // println!("Country {:#?}", user.country);
    // println!("Country Code {:#?}", user.country_code);
    // // println!("Highest Rank {:#?}", user.highest_rank.unwrap().rank);
    // println!("Current Rank {:#?}", global_rank);
    // println!("Country Rank {:#?}", country_rank);
    // println!("Active {:#?}", user.is_active);
    // println!("Online {:#?}", user.is_online);
    // println!("Osu Supporter {:#?}", user.is_supporter);
    // println!("Ranked Mapset Count {:#?}", user.ranked_mapset_count.unwrap());
    // println!("Last seen {:#?}", user.last_visit.unwrap());
    // println!("Has supported {:#?}", user.has_supported);
    // println!("PP {:#?}", statistics.pp);
    // println!("Accuracy {:#?}", statistics.accuracy);
    // println!("Level {:#?}", statistics.level.current);
    // println!("Max Combo {:#?}", statistics.max_combo);
    // println!("Total Playcount {:#?}", statistics.playcount);
    // println!("Total Playtime {:#?}", statistics.playtime);
    // println!("Ranked Score {:#?}", statistics.ranked_score);
    // println!("Total Hits {:#?}", statistics.total_hits);
    // println!("Total Score {:#?}", statistics.total_score);
    // println!("Rank since 1 month {:#?}", statistics.rank_change_since_30_days);

    // let scores = Scores::scores().await;
    // println!("username: {:?}", &scores.input_username);
    // println!("user_ext: {:?}", &scores.user_ext);

    // print_table();
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        // .load_preset(UTF8_FULL)
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Username").fg(Color::Blue),
            Cell::new(user.username).fg(Color::Blue),
            Cell::new("Second Username").fg(Color::Blue),
            // "Username".to_string(),
            // user.username.to_string(),
            // "Second Username".to_string(),
        ]);

    // Add rows for each field

    // table.add_row(vec![
    //     "Username".to_string(), 
    //     user.username.to_string(), 
    //     // user[index].to_string()
    // ]);
    table.add_row(vec![
        Cell::new("Country"),
        Cell::new(user.country),
        Cell::new("user2"),
        // "Country".to_string(),
        // user.country.to_string(),
        // "1000".to_string()
    ]);
    table.add_row(vec![
        "Current Rank".to_string(),
        global_rank.to_string(),
        "1000".to_string()
    ]);
    //TODO Maybe add bg color for the higher scores when comparing users (green best, red worst, blue mid)
    table.add_row(vec![
        Cell::new("PP").fg(Color::Rgb{r:255, g:000, b:200}),
        Cell::new(statistics.pp).fg(Color::Rgb{r:255, g:000, b:200}),
        // Cell::new("user2"),
        // "PP".to_string(),
        // statistics.pp.to_string(),
    ]);
    table.add_row(vec![
        "Accuracy".to_string(),
        statistics.accuracy.to_string(),
    ]);
    table.add_row(vec![
        "",
    ]);
    table.add_row(vec![
        "Level".to_string(),
        statistics.level.current.to_string(),
    ]);
    table.add_row(vec![
        "Max Combo".to_string(),
        statistics.max_combo.to_string(),
    ]);
    table.add_row(vec![
        "Total Playcount".to_string(),
        statistics.playcount.to_string(),
    ]);
    table.add_row(vec![
        "Total Playtime".to_string(),
        statistics.playtime.to_string(),
    ]);
    table.add_row(vec![
        "Join Date".to_string(),
        user.join_date.date().to_string(),
    ]);

    //TODO Fix error on last seen unwrap statement:
    //thread 'main' (108089) panicked at src/main.rs:148:25:
    // called `Option::unwrap()` on a `None` value
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // table.add_row(vec![
    //     "Last Seen".to_string(),
    //     user.last_visit.unwrap().date().to_string(),
    // ]);

    table.add_row(vec![
        "",
    ]);
    // TODO Set those optional with --all
    table.add_row(vec![
        "UserID".to_string(),
        user.user_id.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "Highest Rank".to_string(),
        user.highest_rank.unwrap().rank.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "Country Code".to_string(),
        user.country_code.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "Country Rank".to_string(),
        country_rank.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "",
    ]);
    table.add_row(vec![
        "Is Active".to_string(),
        user.is_active.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "Is Online".to_string(),
        user.is_online.to_string(),
        "1000".to_string()
    ]);
    table.add_row(vec![
        "Osu Supporter".to_string(),
        user.is_supporter.to_string(),
    ]);
    table.add_row(vec![
        "Was Supporter".to_string(),
        user.has_supported.to_string(),
    ]);
    table.add_row(vec![
        "",
    ]);
    table.add_row(vec![
        "Total Hits".to_string(),
        statistics.total_hits.to_string(),
    ]);
    table.add_row(vec![
        "Ranked Score".to_string(),
        statistics.ranked_score.to_string(),
    ]);
    table.add_row(vec![
        "Total Score".to_string(),
        statistics.total_score.to_string(),
    ]);

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

use crate::stats::{UserStats};
use crate::config::{ConfigFile};

use anyhow::{Context, Result};
use comfy_table::{Cell, Table, ContentArrangement};
use comfy_table::presets::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::*;

pub async fn stats_out () -> Result<(), anyhow::Error> {
    let cfg: ConfigFile = confy::load("osu-cli", "cli-config").context("Failed to load config!")?;
    // let user_stats = UserStats::fetch_user_stats(&cfg, &cfg.names[1]).await?;
    // let cookiezi_stats = UserStats::fetch_user_stats(&cfg, "cookiezi").await?;

    for username in &cfg.names {
        let user_stats = UserStats::fetch_user_stats(&cfg, username).await?;
        println!("UserID {:#?}", user_stats.user_id);
    }

    let mut table = Table::new();
     table
        .set_content_arrangement(ContentArrangement::Dynamic)
        // .load_preset(UTF8_FULL)
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Header1", "Header2", "Header3"]);
        println!("{}", table);


    Ok(())
    
    //let mut table = Table::new();
    // table
    //     .set_content_arrangement(ContentArrangement::Dynamic)
    //     // .load_preset(UTF8_FULL)
    //     .load_preset(UTF8_FULL_CONDENSED)
    //     .apply_modifier(UTF8_ROUND_CORNERS)
    //     .set_header(
            // table.add_row(vec![
            //     Cell::new("Country"),
            //     Cell::new(user.country),
            // ]);
    //     );
    //
    // table.add_row(vec![
    //     "",
    // ]);
    // // TODO Set those optional with --all
    // table.add_row(vec![
    //     "UserID".to_string(),
    //     user.user_id.to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "Highest Rank".to_string(),
    //     user.highest_rank.unwrap().rank.to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "Country Code".to_string(),
    //     user.country_code.to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "Country Rank".to_string(),
    //     statistics.country_rank.expect("").to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "",
    // ]);
    // table.add_row(vec![
    //     "Is Active".to_string(),
    //     user.is_active.to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "Is Online".to_string(),
    //     user.is_online.to_string(),
    //     "1000".to_string()
    // ]);
    // table.add_row(vec![
    //     "Osu Supporter".to_string(),
    //     user.is_supporter.to_string(),
    // ]);
    // table.add_row(vec![
    //     "Was Supporter".to_string(),
    //     user.has_supported.to_string(),
    // ]);
    // table.add_row(vec![
    //     "",
    // ]);
    // table.add_row(vec![
    //     "Total Hits".to_string(),
    //     statistics.total_hits.to_string(),
    // ]);
    // table.add_row(vec![
    //     "Ranked Score".to_string(),
    //     statistics.ranked_score.to_string(),
    // ]);
    // table.add_row(vec![
    //     "Total Score".to_string(),
    //     statistics.total_score.to_string(),
    // ]);
    //
    // println!("{}", table);
    //
}

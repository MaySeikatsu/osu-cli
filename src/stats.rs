use crate::api::init_api;
use crate::config::{ConfigFile, GamemodeOptions};
use rosu_v2::prelude::*;
use anyhow::{Context, Result};

pub struct UserStats {
    pub username: String,
    pub user_id: u32,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub global_rank: Option<u32>,
    pub country_rank: Option<u32>,
    pub pp: f32,
    pub accuracy: f32,
    pub level: f32,
    pub max_combo: u32,
    pub playcount: u32,
    pub playtime: u32,
    // pub join_date: OffsetDateTime,
    // pub last_visit: Option<OffsetDateTime>,
    pub is_active: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub ranked_score: u64,
    pub total_score: u64,
    pub total_hits: u64,
    pub ranked_mapset_count: Option<u32>,
}

impl UserStats {
    pub async fn fetch_user_stats(cfg: &ConfigFile, username: &str) -> Result<Self> {
        let api_osu = init_api(cfg).await?;
        // let gamemode = cfg.gamemode.into();
        let gamemode = match cfg.gamemode {
            GamemodeOptions::Osu => GameMode::Osu,
            GamemodeOptions::Mania => GameMode::Mania,
            GamemodeOptions::Taiko=> GameMode::Taiko,
            GamemodeOptions::Catch=> GameMode::Catch,
        };
        
        let user = api_osu.user(username).mode(gamemode).await?;
        let statistics = user.statistics.context("Error reading user statistics!")?;
        
        Ok(Self {
            username: user.username.to_string(),
            user_id: user.user_id,
            country: Some(user.country),
            country_code: Some(user.country_code.to_string()),
            global_rank: statistics.global_rank,
            country_rank: statistics.country_rank,
            pp: statistics.pp,
            accuracy: statistics.accuracy,
            level: statistics.level.current as f32,
            max_combo: statistics.max_combo,
            playcount: statistics.playcount,
            playtime: statistics.playtime,
            // join_date: user.join_date,
            // last_visit: user.last_visit,
            is_active: user.is_active,
            is_online: user.is_online,
            is_supporter: user.is_supporter,
            ranked_score: statistics.ranked_score,
            total_score: statistics.total_score,
            total_hits: statistics.total_hits,
            ranked_mapset_count: user.ranked_mapset_count,
        })
    }
}

    // let global_rank =  statistics.global_rank.expect("Error reading global rank!");
    // let country_rank = statistics.country_rank.expect("Error reading country rank!");

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

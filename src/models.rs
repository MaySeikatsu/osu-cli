use rosu_v2::prelude::*;
use time::OffsetDateTime;
use crate::api::init_api;
use crate::config::ConfigFile;
use anyhow::Result;

/// Struct to hold simplified user data for table display
#[derive(Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub user_id: u32,
    pub country: Option<String>,
    pub country_code: String,
    pub global_rank: Option<u32>,
    pub country_rank: Option<u32>,
    pub pp: f32,
    pub accuracy: f32,
    pub level: f32,
    pub max_combo: u32,
    pub playcount: u32,
    pub playtime: u32,
    pub join_date: OffsetDateTime,
    pub last_visit: Option<OffsetDateTime>,
    pub is_active: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub ranked_score: u64,
    pub total_score: u64,
    pub total_hits: u64,
    pub ranked_mapset_count: Option<u32>,
}

impl From<&UserExtended> for UserData {
    fn from(user: &UserExtended) -> Self {
        let statistics = user.statistics.as_ref().unwrap();
        
        UserData {
            username: user.username.to_string(),
            user_id: user.user_id,
            country: Some(user.country.clone()),
            country_code: user.country_code.to_string(),
            global_rank: statistics.global_rank,
            country_rank: statistics.country_rank,
            pp: statistics.pp,
            accuracy: statistics.accuracy,
            level: statistics.level.current as f32,
            max_combo: statistics.max_combo,
            playcount: statistics.playcount,
            playtime: statistics.playtime,
            join_date: user.join_date,
            last_visit: user.last_visit,
            is_active: user.is_active,
            is_online: user.is_online,
            is_supporter: user.is_supporter,
            ranked_score: statistics.ranked_score,
            total_score: statistics.total_score,
            total_hits: statistics.total_hits,
            ranked_mapset_count: user.ranked_mapset_count,
        }
    }
}

/// Struct that fetches user data for multiple usernames
pub struct UserFetcher {
    api: Osu,
    gamemode: GameMode,
}

impl UserFetcher {
    pub async fn new(cfg: &ConfigFile) -> Result<Self> {
        let api = init_api(cfg).await?;
        let gamemode = match cfg.gamemode {
            crate::config::GamemodeOptions::Osu => GameMode::Osu,
            crate::config::GamemodeOptions::Mania => GameMode::Mania,
            crate::config::GamemodeOptions::Taiko => GameMode::Taiko,
            crate::config::GamemodeOptions::Catch => GameMode::Catch,
        };
        
        Ok(Self {
            api,
            gamemode,
        })
    }

    /// Fetch user data for a single username
    pub async fn fetch_user(&self, username: &str) -> Result<UserData> {
        let user_ext = self.api.user(username).mode(self.gamemode).await?;
        Ok(UserData::from(&user_ext))
    }

    /// Fetch user data for multiple usernames
    pub async fn fetch_users(&self, usernames: &[String]) -> Result<Vec<UserData>> {
        let mut user_data = Vec::new();
        
        for username in usernames {
            let data = self.fetch_user(username).await?;
            user_data.push(data);
        }
        
        Ok(user_data)
    }
}
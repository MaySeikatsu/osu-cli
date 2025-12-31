use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub names: Vec<String>, // Changed from single name to vector of names
    pub gamemode: GamemodeOptions, // maybe list?
    // pub api_client_id: ApiClientId,
    pub api_client_id: u32,
    pub api_secret: String,
}
impl Default for ConfigFile {
    fn default () -> Self {
        Self {
            // Vector of usernames for multiple user comparison
            names: vec!["mayseikatsu".to_string(), "peppy".to_string()],
            gamemode: GamemodeOptions::Osu,
            // api_client_id: ApiClientId::new(42099),
            api_client_id: 47188,
            api_secret: "6jjRud9BM86DAFQG84ujCfTa7Lc6m2jU9aIT6jRF".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GamemodeOptions {
    Osu,
    Mania,
    Taiko,
    Catch,
}

// TODO: Replace defaults with impl that creates new values as they get provided by the user
// pub struct ApiClientId {
//     client_id: u32,
// }
//
// impl ApiClientId {
//     pub fn new(client_id: u32) -> Self{
//         Self {
//             client_id
//         }
//     }
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub name: String,
    pub gamemode: String, // maybe list?
    // pub api_client_id: ApiClientId,
    pub api_client_id: u32,
    pub api_secret: String,
}
impl Default for ConfigFile {
    fn default () -> Self {
        Self {
            name: "mayseikatsu".to_string(),
            gamemode: "Osu".to_string(),
            // api_client_id: ApiClientId::new(42099),
            api_client_id: 42099,
            api_secret: "rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh".to_string(),
        }
    }
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

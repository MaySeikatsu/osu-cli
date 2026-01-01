pub struct UserScores {

}

impl UserScores {

}

    // let scores = Scores::scores().await;
    // println!("username: {:?}", &scores.input_username);
    // println!("user_ext: {:?}", &scores.user_ext);

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

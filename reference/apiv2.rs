use rosu_v2::prelude::*;

// [tokio::main]
pub async fn v2() -> Result<(), Box<dyn std::error::Error>> {
    // Initilize Client
    let client_id: u64 = 42909;
    let client_secret = String::from("rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh");
    let osu = Osu::new(client_id, client_secret).await.unwrap();

    // Get peppy's top 10-15 scores in osu!standard.
    // Note that the username here can only be used because of the `cache` feature.
    // If you are fine with just providing user ids, consider disabling this feature.
    // println!("You should now see your score:");
    // let scores: Vec<Score> = osu
    //     .user_scores("mayseikatsu")
    //     .mode(GameMode::Osu)
    //     .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
    //     .offset(00)
    //     .limit(1)
    //     .await
    //     .unwrap();
    // println!("{:#?}", scores);

    println!("You should see your Rank:");
    let user_ext = osu.user("mayseikatsu").mode(GameMode::Osu).await?;
    // if let Some(stats) = &user_ext.statistics {
    //     if let Some(global_rank) = stats.global_rank {
    //         println!("Global rank: {}", global_rank);
    //     } else {
    //         println!("No global rank found for this user");
    //     }
    // } else {
    //     println!("No statistics found for this user");
    // }
    let statistics = match user_ext.statistics {
        Some(x) => x,
        None => println!("Error!"),
    };
    println!("{:#?}", statistics.global_rank.unwrap()); //.global_rank);
    println!("{:#?}", user_ext.statistics.unwrap().global_rank.unwrap()); //.global_rank);

    // // Search non-nsfw loved mania maps matching the given query.
    // // Note that the order of called methods doesn't matter for any endpoint.
    // let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
    //     .nsfw(false)
    //     .status(Some(RankStatus::Loved))
    //     .mode(GameMode::Mania)
    //     .query("blue army stars>3")
    //     .await
    //     .unwrap();
    //
    // // Get the french wiki page on the osu file format
    // let wiki_page: WikiPage = osu.wiki("fr")
    //     .page("Client/File_formats/osu_%28file_format%29")
    //     .await
    //     .unwrap();
    Ok(())
}

use std::collections::HashMap;

pub async fn send_discord(message: &str, settings_map: HashMap<String, String>) -> Result<(), reqwest::Error> {
    let discord_webhook_url = settings_map.get("discord_webhook_url").unwrap();
    let discord_webhook_avatar_name = settings_map.get("discord_webhook_avatar_name").unwrap();
    discord_webhook_lib::send_discord(discord_webhook_url, message ,discord_webhook_avatar_name).await
}

// pub async fn send_email() {}


// #[cfg(test)]
// mod tests {
//     use discord_webhook_lib::send_discord;
//
//     #[test]
//     fn test_send_wh() {
//         let rt = tokio::runtime::Runtime::new();
//         rt.unwrap().block_on(send_discord(
//             "https://discord.com/api/webhooks/1014319311197847593/jjY11oRqtES_FS7lz330mqi_4rSl-zA_rNvcg2yDySriqStqmuZtntLsF8dKY1sQvrEW",
//             "Hello World",
//             "Lazarus"
//         )).unwrap();
//     }
// }

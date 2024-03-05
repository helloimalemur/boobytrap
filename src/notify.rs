use std::collections::HashMap;

pub async fn send_discord(message: &str, settings_map: HashMap<String, String>) -> Result<(), reqwest::Error> {
    let discord_webhook_url = settings_map.get("discord_webhook_url").unwrap();
    let discord_webhook_avatar_name = settings_map.get("discord_webhook_avatar_name").unwrap();
    discord_webhook_lib::send_discord(discord_webhook_url, message ,discord_webhook_avatar_name).await
}

// pub async fn send_email() {}

use std::collections::HashMap;
use std::process::Command;
use reqwest::Error;

pub async fn send_discord(message: &str, settings_map: HashMap<String, String>) -> Result<(), anyhow::Error> {
    let mut final_message = String::new();

    if let Ok(output) = Command::new("hostnamectl").arg("hostname").output() {
        if let Ok(hostname) = String::from_utf8(output.stdout) {
            final_message = format!("{} Hostname: {}", message, hostname);
        } else {
            final_message = message.to_string();
        }
    } else {
        final_message = message.to_string();
    }

    let discord_webhook_url = settings_map.get("discord_webhook_url").unwrap();
    let discord_webhook_avatar_name = settings_map.get("discord_webhook_avatar_name").unwrap();
    discord_webhook_lib::send_discord(discord_webhook_url, final_message.as_str(), discord_webhook_avatar_name).await
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
//             "",
//             "Hello World",
//             "Lazarus"
//         )).unwrap();
//     }
// }

use config::Config;
use std::process::Command;

pub async fn send_discord(message: &str, settings_map: Config, append: &String) -> Result<(), anyhow::Error> {
    #[allow(unused)]
    let mut final_message = String::new();

    if let Ok(output) = Command::new("hostnamectl").arg("hostname").output() {
        if let Ok(hostname) = String::from_utf8(output.stdout) {
            final_message = format!("{} Hostname: {} :: {}", message, hostname, append);
        } else {
            final_message = message.to_string();
        }
    } else {
        final_message = message.to_string();
    }

    let discord_webhook_url = settings_map.get::<String>("discord_webhook_url").unwrap();
    let discord_webhook_avatar_name = settings_map
        .get::<String>("discord_webhook_avatar_name")
        .unwrap();
    discord_webhook_lib::send_discord(
        discord_webhook_url.as_str(),
        final_message.as_str(),
        discord_webhook_avatar_name.as_str(),
    )
    .await
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

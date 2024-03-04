use std::process::Command;

pub async fn reboot_system() {
    if let Ok(reboot) = Command::new("reboot").output() {
        println!("{:#?}", reboot)
    }
}

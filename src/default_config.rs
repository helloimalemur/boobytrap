use std::fs;
use std::path::Path;

pub fn get_cache_dir() -> String {
    #[allow(unused)]
    let mut cache_dir = String::new();
    let cur_user = whoami::username();
    if cur_user.eq_ignore_ascii_case("root") {
        let _ = fs::create_dir_all(Path::new("/root/.config/boobytrap/config/"));
        "/root/.config/boobytrap/".to_string()
    } else {
        let create_dir = format!("/home/{}/.config/boobytrap/config/", cur_user);
        let _ = fs::create_dir_all(Path::new(create_dir.as_str()));
        format!("/home/{}/.config/boobytrap/", cur_user)
    }
}

pub fn write_default_blacklist<T: ToString>(path: T) {
    let _ = fs::write(path.to_string(), default_blacklist());
}

fn default_blacklist() -> &'static str {
    r#"
/etc/mtab
"#
}

pub fn write_default_config<T: ToString>(path: T) {
    if fs::write(path.to_string(), default_config()).is_ok() {
        println!(
            "{}\n ~~~~~~~ UNABLE TO LOCATE CONFIG FILE - DEFAULT CONFIG CREATED ~~~~~~~",
            default_config()
        );
    }
}

fn default_config() -> &'static str {
    r#"
## General settings
tick_delay_seconds = "5"
fs_tick_delay_seconds = "300"

### File System Integrity
fs_mon_enabled = "true"
fs_mon_dir = ["/etc", "/bin", "$PATH"]
fs_mon_hash_type = "blake3"

### USB Monitor
usb_mon_enabled = "true"
reboot_on_increase_of_usb_devices = "false"
notify_on_increase_of_usb_devices = "true"
unmount_crypt_on_increase_of_usb_devices = "true"

### Burn File Monitor
burn_file_mon_enabled = "false"
unmount_crypt_on_file_burn = "true"
ssh_check_burn_host = "hostname"
ssh_check_burn_user = "root"
ssh_check_burn_key = "/home/user/.ssh/id_rsa"
ssh_check_burn_path = "/root/.config/burn"
ssh_check_burn_check_interval = "30"
burn_path_1 = "/root/test/"

### Network Monitor
net_mon_enabled = "false"

######## Notification settings
discord_webhook_url = "https://discord.com/api/webhooks/"
discord_webhook_avatar_name = "Lazarus"
"#
}

use std::fs;

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
fs_tick_delay_seconds = "60"

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
ssh_check_burn_path = "/root/.cache/burn"
ssh_check_burn_check_interval = "30"
burn_path_1 = "/root/test/"

### Network Monitor
net_mon_enabled = "false"

######## Notification settings
discord_webhook_url = "https://discord.com/api/webhooks/"
discord_webhook_avatar_name = "Lazarus"
    "#
}

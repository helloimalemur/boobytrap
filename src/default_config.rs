use std::{fs, process};
use std::path::Path;
use std::process::exit;

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
/etc/cups
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
fs_mon_path_variable = true
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


pub fn setup_service() {
    write_service_file("/etc/systemd/system/boobytrap.service");
    if let Ok(o) = process::Command::new("systemctl")
        .arg("daemon-reload")
        .output()
    {
        println!("{}", String::from_utf8_lossy(&o.stdout));
        if let Ok(o) = process::Command::new("systemctl")
            .arg("enable")
            .arg("boobytrap")
            .output()
        {
            println!("{}", String::from_utf8_lossy(&o.stdout));
            if let Ok(o) = process::Command::new("systemctl")
                .arg("start")
                .arg("boobytrap")
                .output()
            {
                println!("{}", String::from_utf8_lossy(&o.stdout));
                exit(0)
            }

        }

    }
}

pub fn write_service_file<T: ToString>(path: T) {
    if whoami::username().eq_ignore_ascii_case("root") {
        if fs::write(path.to_string(), service_file()).is_ok() {
            println!(
                "{}\n ~~~~~~~ SERVICE CONFIG CREATED ~~~~~~~",
                service_file()
            );
        } else {
            println!("{}", "COULD NOT WRITE SERVICE CONFIG TO FILE");
        }
    } else {
        println!("{}", "PLEASE RUN AS ROOT - COULD NOT WRITE SERVICE CONFIG TO FILE");
    }
}

fn service_file() -> &'static str {
    r#"
[Unit]
Description=Boobytrap

[Service]
Type=simple
User=root
Group=root
ExecStart=/root/.cargo/bin/boobytrap

[Install]
WantedBy=multi-user.target
    "#
}

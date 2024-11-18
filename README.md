# Boobytrap
#### (work-in-progress)
## Detect and Act on unauthorized access of any kind from any source

### Detect and Act on;
    - an increase of USB devices
    - network issues or network failure
    - filesystem changes
    - ssh "burn file"

#### Observed memory usage <100MB to ~1GB

### Install
```shell
## install binary
cargo install boobytrap
## configure service with discord webhook
boobytrap install-service webhook=https://discordapp.com/api/webhooks/121946119953658680...
```

# Setup
### create config/Settings.toml
```shell
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
ssh_check_burn_path = "/root/.config/burn"
ssh_check_burn_check_interval = "30"
burn_path_1 = "/root/test/"

### Network Monitor
net_mon_enabled = "false"

######## Notification settings
discord_webhook_url = "https://discord.com/api/webhooks/"
discord_webhook_avatar_name = "Lazarus"
```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

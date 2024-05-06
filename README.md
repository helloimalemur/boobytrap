# TripWire (work-in-progress)
## System Security Monitor
## Detect and Act on unauthorized access of any kind from any source

### Detect and Act on;
    - an increase of USB devices
    - network issues or network failure
    - file changes
    - ssh "burn file"
    - insecure configuration


# Setup
### create config/Settings.toml
```shell
## General settings
tick_delay_seconds = "5"
fs_tick_delay_seconds = "15"

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
ssh_check_burn_key = "/home/foxx/.ssh/id_ed25519"
ssh_check_burn_path = "/root/.cache/burn"
ssh_check_burn_check_interval = "30"
burn_path_1 = "/root/test/"

### Network Monitor
net_mon_enabled = "false"

######## Notification settings
discord_webhook_url = "https://discord.com/api/webhooks/"
discord_webhook_avatar_name = "Lazarus"

```
### Run
```shell
sudo bash -e install.sh
```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

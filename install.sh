#!/bin/bash
if [[ -f /etc/systemd/system/tw.service ]]; then systemctl stop tw; else echo "na"; fi;
if [[ -d /var/lib/tw/ ]]; then echo '/var/lib/tw/ exists'; else mkdir /var/lib/tw/; fi;
if [[ -f Cargo.toml ]]; then cargo build --release; else cd /var/lib/tw/ && cargo build --release; fi
CURDIR=$(pwd)
if [ "$CURDIR" != "/var/lib/tw/" ]; then
  cp -r ./config/ /var/lib/tw/
  cp -r ./target/ /var/lib/tw/
fi
cp ./run.sh /var/lib/tw/
cp tw.service /etc/systemd/system/tw.service
systemctl daemon-reload
if [[ -f /etc/systemd/system/tw.service ]]; then systemctl restart tw; else echo "na"; fi;

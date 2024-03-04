#!/bin/bash
if [[ -f /etc/systemd/system/tw.service ]]; then systemctl stop tw; else echo "na"; fi;
if [[ -d /var/lib/tw/ ]]; then echo rm -rf /var/lib/tw/*; fi;
if [[ -d /var/lib/tw/ ]]; then echo '/var/lib/tw/ exists'; else mkdir /var/lib/tw/; fi;
if [[ -f Cargo.toml ]]; then cargo build --release; fi
if [[ -f target/release/tw ]]; then cp target/release/tw /var/lib/tw; fi
if [[ -d config/ ]]; then cp -r config/ /var/lib/tw/; fi
if [[ -f run.sh ]]; then cp run.sh /var/lib/tw/; fi
if [[ -f tw.service ]]; then cp tw.service /etc/systemd/system/tw.service; fi
systemctl daemon-reload
if [[ -f /etc/systemd/system/tw.service ]]; then systemctl restart tw; else echo "na"; fi;

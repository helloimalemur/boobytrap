#!/bin/bash
if [[ -f /etc/systemd/system/boobytrap.service ]]; then systemctl stop boobytrap; else echo "na"; fi;
if [[ -d /var/lib/boobytrap/ ]]; then echo rm -rf /var/lib/boobytrap/*; fi;
if [[ -d /var/lib/boobytrap/ ]]; then echo '/var/lib/boobytrap/ exists'; else mkdir /var/lib/boobytrap/; fi;
if [[ -f Cargo.toml ]]; then cargo build --release; fi
if [[ -f target/release/boobytrap ]]; then cp target/release/boobytrap /var/lib/boobytrap; fi
if [[ -d config/ ]]; then cp -r config/ /var/lib/boobytrap/; fi
if [[ -f run.sh ]]; then cp run.sh /var/lib/boobytrap/; fi
if [[ -f boobytrap.service ]]; then cp boobytrap.service /etc/systemd/system/boobytrap.service; fi
systemctl daemon-reload
if [[ -f /etc/systemd/system/boobytrap.service ]]; then systemctl restart boobytrap; else echo "na"; fi;

#!/bin/bash
cp ./mayday.service /etc/systemd/system/mayday.service
systemctl daemon-reload
/root/.cargo/bin/rustup update
/root/.cargo/bin/cargo build --release -j 3

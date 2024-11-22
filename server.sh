#!/bin/bash
cd qrcode-helper/
cargo build --release
cp target/release/qrcode-helper ../sync-ftp-server/artifact/release

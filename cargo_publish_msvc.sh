#!/bin/sh

sudo rm -rf /opt/xwin /.xwinclear
sudo ~/.cargo/bin/xwin --accept-license splat --output /opt/xwin
cargo clean
cargo build --target=x86_64-pc-windows-msvc -Z unstable-options --out-dir=publish
cd publish || exit
./vanillacoffee.exe

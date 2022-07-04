# Project Vanilla Coffee

Took me like 3 years to get around to updating this, time to finally start i guess

## How to Build

for linux: cargo build --target x86_64-unknown-linux-gnu -Z unstable-options --out-dir=publish --release

for windows: cargo build --target=x86_64-pc-windows-msvc -Z unstable-options --out-dir=publish --release

users compiling on windows should comment out the rustflags for the msvc target or it wont compile

// macos sucks and i cant test it so no

make sure to move /assets to the correct place inside the build folder or move the exe into the same folder as this readme

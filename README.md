# Vanilla Coffee
Took me like 3 years to get around to updating this, time to finally start i guess



# build with 
cargo build --target x86_64-unknown-linux-gnu -Z unstable-options --out-dir=publish --release       //for linux
cargo build --target=x86_64-pc-windows-msvc -Z unstable-options --out-dir=publish --release //for windows host machine comment out the rustflags for the msvc target or youll get no compile

// macos sucks and i cant test it so no


cargo build --out-dir=publish -Z unstable-options --target x86_64-pc-windows-msvc
is a zip ready setup once assets is in publish folder with nothing extra (sorta can prolly delete rlib and pdb)


make sure to move /assets to the correct place inside the build folder or move the exe into the same folder as this readme



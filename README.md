# Vanilla Coffee
Took me like 3 years to get around to updating this, time to finally start i guess



# build with 

cargo build --release --target x86_64-unknown-linux-gnu        //for linux
cargo build --release --target x86_64-pc-windows-msvc          //for windows  comment out the rustflags for the msvc target or youll get no compile
                                                               // macos sucks and i cant test it so no


cargo build --out-dir=publish -Z unstable-options --target x86_64-pc-windows-msvc
is a zip ready setup once assets is in publish folder with nothing extra (sorta can prolly delete rlib and pdb)


make sure to move /assets to the correct place inside the build folder or move the exe into the same folder as this readme



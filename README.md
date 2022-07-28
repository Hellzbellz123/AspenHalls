# Project Vanilla Coffee

this is **NOT** stable, comes as is and im probably gonna change it alot or it might even just vanish one day.

Took me like 3 years to get around to updating this, time to finally start i guess

## How to Build

for linux: cargo build --target x86_64-unknown-linux-gnu -Z unstable-options --out-dir=publish --release

for windows: cargo build --target=x86_64-pc-windows-msvc -Z unstable-options --out-dir=publish --release

users compiling on windows should comment out the rustflags for the msvc target or it wont compile

// macos sucks and i cant test it so no

make sure to move /assets next to the application exe, same folder.

## For Linux users wanting to build for window

cargo install c cargo-xwin

im just keeping this old snippet here for others too see how to use xwin on its own

xwin

```# [target.x86_64-pc-windows-msvc]
# linker = "lld-link"
# rustflags = [
#   "-C",
#   "target-feature=+crt-static",
#   "-Zshare-generics=off",
#   "-Lnative=/opt/xwin/crt/lib/x86_64",
#   "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
#   "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64",
# ]
```

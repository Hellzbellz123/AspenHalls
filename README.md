# Project Vanilla Coffee [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/) ![Repo Size](https://img.shields.io/github/repo-size/hellzbellz123/vanillacoffee?color=2948ff&label=Repo%20Size&style=flat-square)

this is **NOT** stable, comes as is and im probably gonna change it alot or it might even just vanish one day.

Took me like 3 years to get around to updating this, time to finally start i guess
funny story, this was orignally started as 3d zelda clone in unity but it never really got anywhere

## How to Build

for linux: cargo build --target x86_64-unknown-linux-gnu -Z unstable-options --out-dir=publish --release

for windows: cargo build --target=x86_64-pc-windows-msvc -Z unstable-options --out-dir=publish --release

users compiling on windows should comment out the rustflags for the msvc target or it wont compile

// macos sucks and i cant test it so no

make sure to move /assets next to the application exe, same folder.

## For Linux users wanting to build for windows

cargo install c cargo-xwin

im just keeping this old snippet here for others too see how to use xwin on its own

xwin

```rust
[target.x86_64-pc-windows-msvc]
 linker = "lld-link"
 rustflags = [
   "-C",
   "target-feature=+crt-static",
   "-Zshare-generics=off",
   "-Lnative=/opt/xwin/crt/lib/x86_64",
   "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
   "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64",
 ]
```

This crate is a bit redundant now since Bevy 0.8 as sending events using World is very easy. With commands.add you can queue a closure to dispatch an event like so:

```rust
commands.add(|world: &mut World|
    world.send_event(MyEvent)
);
```

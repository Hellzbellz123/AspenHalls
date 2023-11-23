# Aspen Halls (a video game)

> This projects details can be found at my [page](https://hellzbellz123.github.io/AspenHalls/)

Took me 3 years too get around to updating this, time to finally get started i guess.

funny story, this was originally started as 3d zelda clone in unity.
However i gave up and did not touch it for a really long time

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/) ![Repo Size](https://img.shields.io/github/repo-size/hellzbellz123/AspenHalls?color=2948ff&label=Repo%20Size&style=flat-square)

## Ci

<p align="center">
    <img alt="GitHub CI Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/ci.yml?label=ci&style=flat-square">
    <img alt="GitHub Build Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/build.yml?label=Build%20Native&style=flat-square">
    <img alt="GitHub Android Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/build-android.yml?label=Build%20Android&style=flat-square">
    <a href="https://hellzbellz123.github.io/AspenHalls/"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/release-gh-pages.yml?label=Build%20Web&style=flat-square"></a>
    <a href="https://github.com/Hellzbellz123/AspenHalls/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/Hellzbellz123/AspenHalls?label=download&style=flat-square"></a>
</p>

## Platforms

- Library (The game can be used as a library for porting too other platforms or using other init strategys)
- Native (MacOs, Linux & Windows, a single launcher built for each target)
- Web (Wasm)
- Mobile
  - Android
  - iOS (⚠️ Soon)

## Requirements

- Rust
- Cargo
- [Cargo Make](https://github.com/sagiegurari/cargo-make)
- [Trunk](https://trunkrs.dev) (Optional for web development)

## Development Guide

- Edit the `.env` file if you need
- Run `cargo make run-native` for run desktop dev mode
- Run `cargo make run-mobile` too build and install on connected adb device
- Run `cargo make run-web` too build and install on connected adb device
- Run `cargo make` for all available tasks

### Other CargoMake Tasks

- **build** - Generate release binary/lib
- **check** - Check all issues, format and code quality
- **clean** - Clean all target directory
- **clippy** - Check code quality
- **default** - Check all issues, format and code quality
- **dev** - Run native launcher with development configuration
- **fix-all** - Try fix all clippy and format issues
- **fix-clippy** - Fix code quality
- **fix-fmt** - Fix format
- **fmt** - Check format quality
- **test** - Check all unit test

## Usage as Library

> ⚠️ Check the `launchers` folders for examples

why would you use this as a library?

- create ports too new platforms
- maybe mods?

## Build/Compile Time Benchmarks

Host Specs:

- cpu: Ryzen 5 5600X
- ram: 32gb 3600mhz
- os: Archlinux
- Compiler info
  - Rust Version: nightly-2023-11-20

Benchmarks:

- Benchmark 1: cargo cranky [with sccache, cold cache]
  - Time (mean ± σ):     88.580 s ±  1.590 s    [User: 220.533 s, System: 25.353 s]
  - Range (min … max):   87.227 s … 90.331 s    3 runs
- Benchmark 2: cargo cranky [without sccache, cold cache]
  - Time (mean ± σ):     78.362 s ±  0.912 s    [User: 476.357 s, System: 42.161 s]
  - Range (min … max):   77.703 s … 79.403 s    3 runs
- Benchmark 3: cargo cranky [with sccache, cargo clean]
  - Time (mean ± σ):     84.620 s ±  1.081 s    [User: 218.377 s, System: 24.133 s]
  - Range (min … max):   83.436 s … 85.555 s    3 runs
- Benchmark 4: cargo cranky [with sccache, no clean]
  - Time (mean ± σ):     377.2 ms ±   6.6 ms    [User: 260.2 ms, System: 114.0 ms]
  - Range (min … max):   369.8 ms … 382.5 ms    3 runs
- Benchmark 5: cargo cranky [with sccache, no clean, modified project src]
  - Time (mean ± σ):      1.138 s ±  1.332 s    [User: 0.892 s, System: 0.237 s]
  - Range (min … max):    0.365 s …  2.677 s    3 runs

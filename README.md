# Aspen Halls (a video game)
Took me 3 years too get around to updating this, time to finally get started i guess.

funny story, this was originally started as 3d zelda clone in unity but i gave up and did not touch it for a really long time

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/) ![Repo Size](https://img.shields.io/github/repo-size/hellzbellz123/AspenHalls?color=2948ff&label=Repo%20Size&style=flat-square)
<p align="center">
    <img alt="GitHub CI Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/ci.yml?label=ci&style=flat-square">
    <img alt="GitHub Build Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/build.yml?label=Build%20Native&style=flat-square">
    <img alt="GitHub Android Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/build-android.yml?label=Build%20Android&style=flat-square">
    <a href="https://hellzbellz123.github.io/AspenHalls/"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Hellzbellz123/AspenHalls/release-gh-pages.yml?label=Build%20Web&style=flat-square"></a>
    <a href="https://github.com/Hellzbellz123/AspenHalls/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/Hellzbellz123/AspenHalls?label=download&style=flat-square"></a>
</p>

> ℹ️ This projects details can be found at my [page](<https://hellzbellz123.github.io/AspenHalls/>)

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
>
> ⚠️ Check the `launchers` folders

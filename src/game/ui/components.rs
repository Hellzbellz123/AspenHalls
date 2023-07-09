use bevy::prelude::Component;

/// marker for root of all ui
#[derive(Component)]
pub struct UiRoot;

/// start menu root marker
#[derive(Component)]
pub struct StartMenuRoot;

/// pause menu root marker
#[derive(Component)]
pub struct PauseMenuRoot;

/// settings menu root marker
#[derive(Component)]
pub struct SettingsMenuRoot;

/// play button marker
#[derive(Component)]
pub struct PlayButton;

/// continue button marker
#[derive(Component)]
pub struct ContinueButton;

/// exit button marker
#[derive(Component)]
pub struct ExitButton;

/// settings button marker
#[derive(Component)]
pub struct SettingsButton;

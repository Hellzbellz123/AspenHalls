use bevy::ecs::schedule::IntoSystemSetConfigs;

use crate::ahp::engine::{
    App, InputManagerPlugin, InputManagerSystem, Plugin, PreUpdate,
    SystemSet,
};

/// holds action maps
pub mod action_maps;
/// keyboard input systems
mod kbm;
/// software cursor plugin updated with touch and kbm input settings
mod software_cursor;
/// touch input systems
mod touch;

/// system set for ordering input related systems
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspenInputSystemSet {
    /// KBM input is collected first
    KBMInput,
    /// Then touch input is collected, overwriting KBM input if touches present
    TouchInput,
    /// software cursor is updated after mouse/touch input is added
    SoftwareCursor,
}

/// player input plugin
pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            InputManagerPlugin::<action_maps::Gameplay>::default(),
        );
        // TODO: make this plugin only active by default if target_platform == (ANDROID || IOS) else make it a setting too enable
        app.add_plugins(touch::TouchInputPlugin);
        // updates LookWorld and LookLocal based off mouse position inside window
        app.add_plugins(kbm::KBMPlugin);
        // TODO: make software cursor an option in the settings, mostly only useful for debugging
        app.add_plugins(software_cursor::SoftwareCursorPlugin);

        app.configure_sets(
            PreUpdate,
            (
                AspenInputSystemSet::KBMInput,
                AspenInputSystemSet::TouchInput,
                AspenInputSystemSet::SoftwareCursor,
            )
                .chain()
                .in_set(InputManagerSystem::ManualControl),
        );
    }
}

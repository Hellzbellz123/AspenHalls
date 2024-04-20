use bevy::prelude::*;

use crate::{
    game::components::TimeToLive,
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition},
        registry::RegistryIdentifier,
    },
    utilities::scale_to_fit,
    AppState,
};

/// animation functionality
pub mod animations;
/// character/item stats functionality
pub mod attributes_stats;
/// audio data for game
pub mod audio;
/// game characters spawning and functionality
pub mod characters;
/// combat functionality plugin
pub mod combat;
/// shared components for game
pub mod components;
/// sanctuary and dungeon generator
pub mod game_world;
/// input from player
pub mod input;
/// Game `UserInterface` Module, contains interface plugin
pub mod interface;
/// game item spawning and functionality
pub mod items;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameProgress {
    /// homeroom
    #[default]
    Sanctuary,
    /// in dungeon now
    Dungeon,
}

/// each dungeon run has 4 stages that get progressivly larger/harder
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DungeonFloor {
    /// easiest level, start here
    #[default]
    One,
    /// slighlty deeper, bit larger, more creeps
    Two,
    ///
    Three,
    /// final level of the dungeon
    Four,
}

/// plugin that holds all game functionality as plugin modules
pub struct AspenHallsPlugin;

impl Plugin for AspenHallsPlugin {
    fn build(&self, app: &mut App) {
        app
            // actual game plugin
            .add_plugins((
                combat::CombatPlugin,
                characters::CharactersPlugin,
                items::ItemsPlugin,
                input::InputPlugin,
                game_world::GameWorldPlugin,
                interface::InterfacePlugin,
                audio::AudioPlugin,
                animations::AnimationsPlugin,
            ))
            .add_systems(
                Update,
                ((update_actor_size, time_to_live)
                    .run_if(in_state(AppState::PlayingGame)),),
            );
    }
}

/// despawn any entity with `TimeToLive` timer thats finished
fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// update actor size if its custom size is not already set
fn update_actor_size(
    mut query: Query<(
        &mut Sprite,
        &TextureAtlas,
        &RegistryIdentifier,
    )>,
    texture_atlasses: Res<Assets<TextureAtlasLayout>>,
    obje_assets: Res<Assets<ItemDefinition>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    for (mut sprite, texture_atlas, registry_identifier) in &mut query {
        if sprite.custom_size.is_some() {
            continue;
        }

        let atlas = texture_atlasses
            .get(texture_atlas.layout.clone())
            .expect("texture atlas layout for this spritesheet is missing");
        let original_size = atlas.textures.first().expect("no textures in atlas").size();
        let aspect_ratio = original_size.x / original_size.y;

        trace!(
            "image size: {}, aspect ratio: {}",
            original_size,
            aspect_ratio
        );

        let final_size: Vec2 = {
            let maybe_characer = char_assets
                .iter()
                .find(|(_, asset)| asset.actor.identifier == *registry_identifier);
            let maybe_item = obje_assets
                .iter()
                .find(|(_, asset)| asset.actor.identifier == *registry_identifier);

            if let Some((_, def)) = maybe_characer {
                def.actor.pixel_size
            } else if let Some((_, def)) = maybe_item {
                def.actor.pixel_size
            } else {
                warn!("character has no asset");
                return;
            }
        };

        let new_custom_size = scale_to_fit(original_size, final_size);

        // info!(
        //     "target size: {}, new_custom_size: {}",
        //     final_size, new_custom_size
        // );
        sprite.custom_size = Some(new_custom_size);
    }
}

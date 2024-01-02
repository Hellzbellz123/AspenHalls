use bevy::{
    app::Update,
    asset::{Assets, Handle},
    ecs::{
        query::Changed,
        system::{Query, Res},
    },
    log::{info, warn},
    math::Vec2,
    prelude::{state_exists_and_equals, IntoSystemConfigs},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    consts::{MIN_VELOCITY, WALK_MODIFIER},
    game::actors::{
        attributes_stats::CharacterStats,
        components::{ActorMoveState, CurrentMovement},
    },
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition},
        registry::RegistryIdentifier,
    },
    prelude::engine::{App, Plugin},
    AppState,
};

/// all functionality for artificial intelligence on actors is stored here
pub mod ai;
/// holds animation functionality for actors plugin
pub mod animation;
/// holds player stat functionality
pub mod attributes_stats;
/// game combat functionality
pub mod combat;
/// shared actor components
pub mod components;
/// holds enemy functionality
pub mod enemies;
/// holds player information and functions
pub mod player;
/// holds spawner info
pub mod spawners;

/// all Characters in the game, along with spawners for spawn able characters
// TODO: make actors "configurable". load actor types from $PACK/definitions/$ACTORTYPE/ and add them too a database.
// use this database for "available actors" when spawning
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharacterStats>()
            .add_plugins((
                spawners::SpawnerPlugin,
                animation::AnimationPlugin,
                player::PlayerPlugin,
                combat::ActorWeaponPlugin,
                enemies::EnemyPlugin,
                ai::AIPlugin,
            ))
            .add_systems(
                Update,
                (update_character_move_status, update_actor_size)
                    .run_if(state_exists_and_equals(AppState::PlayingGame)),
            );
    }
}

/// updates actors move status component based on actors velocity and speed attribute
fn update_character_move_status(
    mut actor_query: Query<(&mut ActorMoveState, &Velocity, &CharacterStats), Changed<Velocity>>,
) {
    for (mut move_state, velocity, stats) in &mut actor_query {
        let stats = stats.attrs();
        if velocity.linvel.abs().max_element() < MIN_VELOCITY {
            if move_state.move_status != CurrentMovement::None {
                move_state.move_status = CurrentMovement::None;
                return;
            }
        } else if velocity.linvel.abs().max_element() <= (stats.move_speed * WALK_MODIFIER) {
            if move_state.move_status != CurrentMovement::Walk {
                move_state.move_status = CurrentMovement::Walk;
                return;
            }
        } else if velocity.linvel.abs().max_element() <= stats.move_speed
            && move_state.move_status != CurrentMovement::Run
        {
            move_state.move_status = CurrentMovement::Run;
            return;
        }
    }
}

/// update actor size if its custom size is not already set
fn update_actor_size(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &RegistryIdentifier,
    )>,
    texture_atlass: Res<Assets<TextureAtlas>>,
    obje_assets: Res<Assets<ItemDefinition>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    for (mut sprite, texture_atlas, registry_identifier) in &mut query {
        if sprite.custom_size.is_some() {
            continue;
        }

        let atlas = texture_atlass
            .get(texture_atlas)
            .expect("texture for this spritesheet is missing");
        let original_size = atlas.textures.first().expect("no textures in atlas").size();
        let aspect_ratio = original_size.x / original_size.y;

        info!(
            "image size: {}, aspect ratio: {}",
            original_size, aspect_ratio
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

        info!(
            "target size: {}, new_custom_size: {}",
            final_size, new_custom_size
        );
        sprite.custom_size = Some(new_custom_size);
    }
}

/// scales a `Vec2` so its largest value is smaller than x/y of final size
fn scale_to_fit(current: Vec2, final_size: Vec2) -> Vec2 {
    // Calculate scaling factors for both dimensions
    let min_scale = (final_size.x / current.x).min(final_size.y / current.y);

    // Scale the Vec2
    Vec2 {
        x: current.x * min_scale,
        y: current.y * min_scale,
    }
}

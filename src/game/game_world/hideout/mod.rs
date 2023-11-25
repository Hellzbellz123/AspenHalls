use bevy::{
    log::{debug, info},
    prelude::{
        resource_exists, run_once, state_exists_and_equals, Assets,
        Commands, Condition, DespawnRecursiveExt, Entity, Event,
        IntoSystemConfigs, Name, Plugin, Query, Res, ResMut, Update, With,
    },
    render::view::NoFrustumCulling,
    utils::default,
};

use crate::{
    game::{
        actors::{
            ai::components::Enemy, spawners::components::WeaponType,
        },
        game_world::hideout::systems::{
            // enter_the_dungeon,
            home_world_teleporter_collisions,
        },
        AppState,
    },
    loading::{
        assets::{MapAssetHandles, SingleTileTextureHandles},
        // custom_assets::background_shader::ScaledBackgroundMaterial
    },
};

use self::systems::MapContainerTag;

/// shared map components
pub mod map_components;
/// hideout systems
pub mod systems;

/// event for player teleportation
#[derive(Event)]
pub struct PlayerTeleportEvent;

/// plugin for safe house
pub struct HideOutPlugin;

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        // app.add_plugins(bevy_tiling_background::TilingBackgroundPlugin::<
        // ScaledBackgroundMaterial,
        // >::default());
        app.add_event::<PlayerTeleportEvent>().add_systems(
            Update,
            (
                // TODO: fix scheduling
                systems::spawn_hideout.run_if(
                    state_exists_and_equals(AppState::StartMenu)
                        .and_then(run_once()),
                ),
                (
                    // enter_the_dungeon,
                    home_world_teleporter_collisions
                )
                    .run_if(state_exists_and_equals(
                        AppState::PlayingGame,
                    )),
                // cleanup_start_world.run_if(state_exists_and_equals(GeneratorStage::Initialization)),
            )
                .run_if(resource_exists::<MapAssetHandles>()),
        );
    }
}

/// despawn all entities that should be cleaned up on restart
fn cleanup_start_world(
    mut commands: Commands,
    enemies_query: Query<Entity, With<Enemy>>,
    home_world_container: Query<Entity, With<MapContainerTag>>,
    weapons: Query<Entity, With<WeaponType>>,
) {
    if home_world_container.is_empty() {
        debug!("no home world?");
        return;
    }
    commands
        .entity(home_world_container.single())
        .despawn_recursive();
    weapons.for_each(|ent| commands.entity(ent).despawn_recursive());
    enemies_query.for_each(|ent| commands.entity(ent).despawn_recursive());
}

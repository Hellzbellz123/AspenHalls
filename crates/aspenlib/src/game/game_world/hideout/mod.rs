use bevy::{
    log::{debug, info},
    prelude::{
        state_exists_and_equals, Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnEnter,
        Plugin, Query, Update, With,
    },
};

use crate::{
    game::{
        actors::{ai::components::Enemy, spawners::components::WeaponType},
        game_world::{
            dungeonator_v2::DungeonGeneratorState,
            hideout::systems::{
                spawn_hideout,
                // enter_the_dungeon,
                teleporter_collisions,
            },
        },
    },
    AppState,
};

use self::systems::MapContainerTag;

/// hideout systems
pub mod systems;

/// plugin for safe house
pub struct HideOutPlugin;

// TODO: spawn different hideout when player beats boss
// spawn TestingHalls as first level if debug ONLY

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        // app.add_plugins(bevy_tiling_background::TilingBackgroundPlugin::<ScaledBackgroundMaterial>::default());
        app.add_systems(OnEnter(AppState::StartMenu), spawn_hideout)
            .add_systems(
                OnEnter(DungeonGeneratorState::PrepareDungeon),
                cleanup_start_world,
            )
            .add_systems(
                Update,
                (
                    // TODO: fix scheduling
                    teleporter_collisions,
                )
                    .run_if(state_exists_and_equals(AppState::PlayingGame)),
            );
    }
}

// TODO: remove this infavor of DespawnWhenStateIs(Option<S: States/State>)
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
    for ent in &weapons {
        commands.entity(ent).despawn_recursive();
    }
    for ent in &enemies_query {
        commands.entity(ent).despawn_recursive();
    }
}

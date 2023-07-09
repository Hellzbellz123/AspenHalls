use bevy::prelude::{
    info, warn, Commands, DespawnRecursiveExt, Entity, IntoSystemAppConfig, IntoSystemConfig,
    OnEnter, OnUpdate, Plugin, Query, With,
};

use crate::game::{
    actors::{ai::components::Enemy, spawners::components::WeaponType},
    game_world::{
        dungeonator::GeneratorStage,
        hideout::systems::{enter_the_dungeon, homeworld_teleporter_collisions},
    },
    GameStage,
};

use self::systems::MapContainerTag;

/// shared map components
pub mod map_components;
/// hideout systems
pub mod systems;

/// event for player teleportation
pub struct PlayerTeleportEvent;

/// plugin for safe house
pub struct HideOutPlugin;

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app.add_event::<PlayerTeleportEvent>().add_systems((
            systems::spawn_hideout.in_schedule(OnEnter(GameStage::StartMenu)),
            enter_the_dungeon.in_set(OnUpdate(GameStage::PlayingGame)),
            homeworld_teleporter_collisions.in_set(OnUpdate(GameStage::PlayingGame)),
            cleanup_start_world.in_schedule(OnEnter(GeneratorStage::Initialization)),
        ));
    }
}

/// despawns all entities that should be cleaned up on restart
fn cleanup_start_world(
    mut commands: Commands,
    enemys_query: Query<Entity, With<Enemy>>,
    homeworld_container: Query<Entity, With<MapContainerTag>>,
    weapons: Query<Entity, With<WeaponType>>,
) {
    if homeworld_container.is_empty() {
        warn!("no homeworld?");
        return;
    }
    commands
        .entity(homeworld_container.single())
        .despawn_recursive();
    weapons.for_each(|ent| commands.entity(ent).despawn_recursive());
    enemys_query.for_each(|ent| commands.entity(ent).despawn_recursive());
}

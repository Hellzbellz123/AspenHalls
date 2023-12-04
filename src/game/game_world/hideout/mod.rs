use bevy::{
    log::{debug, info},
    math::{IVec2, Vec2},
    prelude::{
        any_with_component, on_event, resource_exists, run_once, state_exists_and_equals, Assets,
        Commands, Condition, DespawnRecursiveExt, Entity, Event, IntoSystemConfigs, Name, OnEnter,
        Plugin, Query, Res, ResMut, Update, With,
    },
    render::view::NoFrustumCulling,
    utils::default,
};
use bevy_ecs_ldtk::prelude::ldtk::ReferenceToAnEntityInstance;

use crate::{
    ahp::game::Player,
    game::{
        actors::{ai::components::Enemy, spawners::components::WeaponType},
        game_world::{hideout::systems::{
            spawn_hideout,
            // enter_the_dungeon,
            teleporter_collisions,
        }, dungeonator_v2::DungeonGeneratorState},
        AppState, GameProgress,
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
#[derive(Event, Debug)]
pub struct ActorTeleportEvent {
    /// enum deciding weather this teleport triggers an aciton or moves entity directly
    /// unhandled tp_actions get warned about
    pub tp_type: TPType,
    /// affected entitiy for this teleport
    pub target: Option<Entity>,
    /// sensor entity that sent this event
    pub sender: Option<Entity>,
}

#[derive(Debug, Clone)]
pub enum TPType {
    //TODO: expand this for better type checking
    /// string type triggering other `Event`
    Event(String),
    /// local teleport. this is alays in tiles, per room
    Local(ReferenceToAnEntityInstance),
    /// teleport with a global pixel position
    Global(Vec2),
}

impl Default for TPType {
    fn default() -> Self {
        TPType::Global(Vec2::ZERO)
    }
}

/// plugin for safe house
pub struct HideOutPlugin;

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        // app.add_plugins(bevy_tiling_background::TilingBackgroundPlugin::<ScaledBackgroundMaterial>::default());
        app.add_event::<ActorTeleportEvent>()
            .add_systems(OnEnter(AppState::StartMenu), spawn_hideout)
            .add_systems(OnEnter(DungeonGeneratorState::PrepareDungeon), cleanup_start_world)
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

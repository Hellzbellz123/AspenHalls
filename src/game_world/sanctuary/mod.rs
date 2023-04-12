use bevy::{
    math::vec3,
    prelude::{
        info, EventWriter, IntoSystemAppConfigs, IntoSystemConfigs, OnExit, OnUpdate, Plugin,
    },
};
use bevy_ecs_ldtk::prelude::{LdtkEntityAppExt, LdtkIntCellAppExt};

use crate::{
    components::actors::spawners::{SpawnWeaponEvent, WeaponType},
    consts::ACTOR_Z_INDEX,
    game::GameStage,
    game_world::{
        components::{LdtkCollisionBundle, LdtkPlayerBundle, LdtkSensorBundle, LdtkSpawnerBundle},
        sanctuary::systems::{enter_the_dungeon, homeworld_teleport},
    },
};

pub mod map_components;
pub mod systems;

pub struct PlayerTeleportEvent;
pub struct HideOutPlugin;

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app.register_ldtk_int_cell_for_layer_optional::<LdtkCollisionBundle>(
            Some("Collision_Layer".to_string()),
            None,
        )
        .register_ldtk_entity::<LdtkSensorBundle>("TeleportSensor")
        .register_ldtk_entity::<LdtkSpawnerBundle>("EnemySpawner")
        .register_ldtk_entity::<LdtkPlayerBundle>("Player")
        .add_event::<PlayerTeleportEvent>()
        .add_systems(
            (
                systems::spawn_mapbundle,
                systems::spawn_homeworld,
                spawn_some_weapons,
            )
                // changed from ononter playsubstage to exiting main menu, to hopefully make pause logic easier
                .in_schedule(OnExit(GameStage::StartMenu)),
        )
        .add_systems(
            (homeworld_teleport, enter_the_dungeon).in_set(OnUpdate(GameStage::PlayingGame)),
        );
    }
}

fn spawn_some_weapons(mut ew: EventWriter<SpawnWeaponEvent>) {
    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallSMG,
        spawn_position: vec3(-833.0, 1270.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });

    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallPistol,
        spawn_position: vec3(-606.0, 1290.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });
}

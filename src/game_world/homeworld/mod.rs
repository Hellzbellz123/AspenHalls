use bevy::{
    math::vec3,
    prelude::{
        info, EventWriter, IntoSystemAppConfigs, IntoSystemConfigs, OnEnter, OnUpdate, Plugin,
    },
};
use bevy_ecs_ldtk::prelude::{LdtkEntityAppExt, LdtkIntCellAppExt};

use crate::{
    components::actors::spawners::{SpawnWeaponEvent, WeaponType},
    game::GameStage,
    game_world::homeworld::{
        map_components::{LdtkCollisionBundle, LdtkSensorBundle, LdtkSpawnerBundle},
        systems::{enter_the_dungeon, homeworld_teleport},
    },
    utilities::game::ACTOR_Z_INDEX,
};

pub mod map_components;
pub mod systems;

pub struct PlayerTeleportEvent;
pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app
            // .register_ldtk_int_cell_for_layer::<RapierSensorBundle>("Physics_Layer", 2)
            .register_ldtk_int_cell_for_layer_optional::<LdtkCollisionBundle>(
                Some("Collision_Layer".to_string()),
                None,
            )
            .register_ldtk_entity::<LdtkSensorBundle>("TeleportSensor")
            .register_ldtk_entity::<LdtkSpawnerBundle>("EnemySpawner")
            .add_event::<PlayerTeleportEvent>()
            .add_systems(
                (
                    systems::spawn_mapbundle,
                    systems::spawn_homeworld,
                    spawn_initial_stuff,
                )
                    .in_schedule(OnEnter(GameStage::PlaySubStage)),
            )
            .add_systems(
                (homeworld_teleport, enter_the_dungeon).in_set(OnUpdate(GameStage::PlaySubStage)),
            );
    }
}

fn spawn_initial_stuff(mut ew: EventWriter<SpawnWeaponEvent>) {
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

    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallSMG,
        spawn_position: vec3(-611.0, 1637.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });

    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallPistol,
        spawn_position: vec3(-881.0, 1637.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });
}

use bevy::{
    math::vec3,
    prelude::{
        info, Commands, DespawnRecursiveExt, Entity, EventWriter, IntoSystemAppConfig,
        IntoSystemAppConfigs, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate,
        Plugin, Query, With,
    },
};
use bevy_ecs_ldtk::prelude::{LdtkEntityAppExt, LdtkIntCellAppExt};

use crate::{
    components::actors::{
        ai::AIEnemy,
        spawners::{SpawnWeaponEvent, WeaponType},
    },
    consts::ACTOR_Z_INDEX,
    game::GameStage,
    game_world::{
        components::{LdtkCollisionBundle, LdtkPlayerBundle, LdtkSensorBundle, LdtkSpawnerBundle},
        dungeon_generator::GeneratorStage,
        sanctuary::systems::{enter_the_dungeon, homeworld_teleport},
    },
};

use self::systems::MapContainerTag;

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
        .add_systems((
            cleanup_start_world.in_schedule(OnEnter(GeneratorStage::Initialization)),
            homeworld_teleport.in_set(OnUpdate(GameStage::PlayingGame)),
            enter_the_dungeon.in_set(OnUpdate(GameStage::PlayingGame)),
            systems::spawn_mapbundle.in_schedule(OnExit(GameStage::StartMenu)),
            systems::spawn_homeworld.in_schedule(OnExit(GameStage::StartMenu)),
            self::spawn_some_weapons.in_schedule(OnExit(GameStage::StartMenu)),
        ));
    }
}

fn cleanup_start_world(
    mut commands: Commands,
    enemys_query: Query<Entity, With<AIEnemy>>,
    homeworld_container: Query<Entity, With<MapContainerTag>>,
    weapons: Query<Entity, With<WeaponType>>,
) {
    commands
        .entity(homeworld_container.single())
        .despawn_recursive();
    weapons.for_each(|ent| commands.entity(ent).despawn_recursive());
    enemys_query.for_each(|ent| commands.entity(ent).despawn_recursive());
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

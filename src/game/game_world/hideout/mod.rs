use bevy::{
    prelude::{
        info, resource_exists, run_once, state_exists_and_equals, warn, Assets, Commands,
        Condition, DespawnRecursiveExt, Entity, Event, IntoSystemConfigs, Name, Plugin, Query, Res,
        ResMut, Update, Vec3, With, Color, Vec2,
    },
    render::view::NoFrustumCulling,
};
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, CustomBackgroundImageBundle, SetImageRepeatingExt,
};

use crate::{
    game::{
        actors::{ai::components::Enemy, spawners::components::WeaponType},
        game_world::hideout::systems::{enter_the_dungeon, home_world_teleporter_collisions},
        AppStage,
    },
    loading::{assets::{MapAssetHandles, SingleTileTextureHandles}, custom_assets::background_shader::ScaledBackgroundMaterial},
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
        app.add_plugins(bevy_tiling_background::TilingBackgroundPlugin::<
            ScaledBackgroundMaterial,
        >::default());
        app.add_event::<PlayerTeleportEvent>().add_systems(
            Update,
            (
                // TODO: fix scheduling
                (systems::spawn_hideout, set_overworld_background)
                    .run_if(state_exists_and_equals(AppStage::StartMenu).and_then(run_once())),
                // .in_schedule(OnEnter(GameStage::StartMenu)),
                (enter_the_dungeon, home_world_teleporter_collisions)
                    .run_if(state_exists_and_equals(AppStage::PlayingGame)),
                // .in_set(OnUpdate(GameStage::PlayingGame)),
                // .in_set(OnUpdate(GameStage::PlayingGame)),
                // cleanup_start_world.run_if(state_exists_and_equals(GeneratorStage::Initialization)),
                // .in_schedule(OnEnter(GeneratorStage::Initialization)),
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
        warn!("no home world?");
        return;
    }
    commands
        .entity(home_world_container.single())
        .despawn_recursive();
    weapons.for_each(|ent| commands.entity(ent).despawn_recursive());
    enemies_query.for_each(|ent| commands.entity(ent).despawn_recursive());
}

pub fn set_overworld_background(
    mut commands: Commands,
    misc: Res<SingleTileTextureHandles>,
    mut materials: ResMut<Assets<ScaledBackgroundMaterial>>,
) {
    let material = ScaledBackgroundMaterial {
        movement_scale: 1.0,
        _wasm_padding: Vec3::ZERO,
        texture: misc.grass.clone(),
    };

    commands.set_image_repeating(misc.grass.clone());
    commands.spawn((
        NoFrustumCulling,
        Name::new("BackgroundImage"),
        CustomBackgroundImageBundle::with_material(material, &mut materials),
    ));
}

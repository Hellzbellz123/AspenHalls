use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};

use crate::Layer;

use self::level::components::{ColliderBundle, Collides};

pub mod level;
pub mod world_components;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .register_ldtk_int_cell_for_layer::<ColliderBundle>("CollisionGrid", 1)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
            })
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_enter(crate::game::GameStage::Playing)
                    .with_system(spawn_world_when_playing),
            )
            .add_system(name_colliders);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/testinghall.ldtk"),
        // level_set: todo!(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 3.0,
                y: 3.0,
                z: 1.0,
            },
            ..default()
        },
        // global_transform: todo!(),
        // visibility: todo!(),
        // computed_visibility: todo!(),
        ..default()
    });
}

fn spawn_world_when_playing(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(0));
}

fn name_colliders(
    mut commands: Commands,
    entity_query: Query<(Entity, &Parent, &Collides, &Transform), Added<Collides>>,
) {
    for entity in entity_query.iter() {
        info!("naming colliders: {}", entity.0.id());
        commands
            .entity(entity.0)
            .insert(Name::new("levelCollider"))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(16.0, 16.0, 0.0),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::World)
                    .with_mask(Layer::Player),
            );
    }
}

//             ColliderBundle {
//                 collider: CollisionShape::Cuboid {
//                     half_extends: Vec3::new(8., 8., 0.),
//                     border_radius: None,
//                 },
//                 rigidbody: RigidBody::Static,
//                 ..Default::default()
//             }

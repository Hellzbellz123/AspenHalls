use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            // .register_ldtk_int_cell::<CollisionBundle>(1)
            .register_ldtk_int_cell_for_layer::<CollisionBundle>("CollisonGrid", 1)
            // .register_default_ldtk_int_cell_for_layer::<CollisionBundle>("CollisionGrid")
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
            })
            .add_startup_system(setup)
            // .add_system(add_names_to_intgrid)
            // .add_system(add_names_to_colliders)
            .add_system_set(
                SystemSet::on_update(crate::game::GameStage::Playing)
                    .with_system(spawn_world_when_playing),
            );
        // .register_ldtk_entity_for_layer::<CollisionBundle>("CollisionGrid", "PlayerCollides");
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Inspectable)]
struct ColliderComponent {
    active: bool,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct CollisionBundle {
    name: Name,
    collider: ColliderComponent,
}

// impl CollisionBundle {
//     pub fn name(&self) -> Name {
//         self.name.clone()
//     }
// }

impl Default for CollisionBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Collider"),
            collider: ColliderComponent { active: true },
        }
    }
}

// fn add_names_to_colliders(
//     named_query: Query<&Name>,
//     mut commands: Commands,
//     entity_query: Query<(Entity, &Parent, &ColliderComponent), Added<ColliderComponent>>, //, Added<ColliderComponent>>,
// ) {
//     for entity in entity_query.iter() {
//         info!("naming colliders: {}", entity.0.id());
//         commands.entity(entity.0).insert(Name::new("Collider"));
//     }
// }

// fn add_names_to_intgrid(
//     mut commands: Commands,
//     entity_query: Query<(Entity, &IntGridCell, &Parent), Added<IntGridCell>>,
// ) {
//     for entit in entity_query.iter() {
//         info!("naming intgrids: {}", entit.0.id());
//         commands.entity(entit.0).insert(Name::new("intgridcell"));
//     }

// .insert_bundle(CollisionBundle {
//         name: Name::new("Collider"),
//         collider: ColliderComponent { active: true },
//     });

// }

// impl From<IntGridCell> for CollisionBundle {
//     fn from(int_grid_cell: IntGridCell) -> CollisionBundle {
//         if int_grid_cell.value == 1 {
//             CollisionBundle {
//                 collider: ColliderComponent { active: true },
//                 name: Name::new("collider"),
//             }
//         } else {
//             CollisionBundle {
//                 name: Name::new("valuenot1"),
//                 ..default()
//             }
//             // CollisionBundle::bundle_int_cell(int_grid_cell, layer_instance)
//         }
//     }
// }

// fn dispose_expired_food(
//     mut commands: Commands,
//     query: Query<Entity, With<Expired>>

// ) {
//     for food_entity in &query {
//         commands.entity(food_entity).despawn();
//     }
// }

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .insert_resource(LevelSelection::Index(1))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Colorful,
                level_background: LevelBackground::Nonexistent,
            })
            .register_ldtk_entity::<MyBundle>("MyEntityIdentifier");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/testinghall.ldtk"),
        // level_set: todo!(),
        transform: Transform {
            translation: Vec3 {
                x: -800.0,
                y: -1000.0,
                z: 1.0,
            },
            rotation: Quat::from_axis_angle(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                90.0,
            ),
            scale: Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
        },
        // global_transform: todo!(),
        // visibility: todo!(),
        // computed_visibility: todo!(),
        ..Default::default()
    });
}

#[derive(Component, Default)]
struct LdtkTileEntity {}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    tile: LdtkTileEntity,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

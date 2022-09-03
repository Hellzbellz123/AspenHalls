use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Colorful,
                level_background: LevelBackground::Nonexistent,
            })
            .add_system_set(
                SystemSet::on_update(crate::game::GameStage::Playing)
                            .with_system(spawn_world_when_playing)
            );
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

fn spawn_world_when_playing (mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(0));
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

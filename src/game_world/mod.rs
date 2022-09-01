use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .insert_resource(LevelSelection::Index(1))
            .register_ldtk_entity::<MyBundle>("MyEntityIdentifier");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/testinghall.ldtk"),
        // level_set: todo!(),
        // transform: todo!(),
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

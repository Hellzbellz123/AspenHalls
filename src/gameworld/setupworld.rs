use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct setupworld;

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/testinghall.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct ComponentC;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentC,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}


#[derive(Bundle, LdtkEntity)]
pub struct ContainerNiceBundle {
    a: ComponentC,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;



pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/testinghall.ldtk"),
        ..Default::default()
    });
}

//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  no terminal on windows in release?
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use gameworld::{setupworld};

mod gameworld;

fn main() {
    let mut kiraApp = App::new();
    kiraApp.insert_resource(WindowDescriptor {
        title: "Project Kira".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    });
    kiraApp.add_startup_system(setupworld::setup);
    kiraApp.add_plugins(DefaultPlugins);
    kiraApp.add_plugin(LdtkPlugin);
    kiraApp.insert_resource(LevelSelection::Index(0));
    kiraApp.register_ldtk_entity::<MyBundle>("MyEntityIdentifier");
    kiraApp.run();
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
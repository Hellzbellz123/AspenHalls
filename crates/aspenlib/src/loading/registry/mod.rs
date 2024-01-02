use std::fmt::Debug;

use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        reflect::ReflectResource,
        system::{Res, Resource},
    },
    prelude::{AssetServer, Assets, Commands, OnExit, ResMut, Deref, DerefMut},
    reflect::Reflect,
    sprite::TextureAtlas,
    utils::HashMap,
};

use crate::{
    bundles::{CharacterBundle, WeaponBundle},
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ObjectDefinition},
        registry::utils::{build_character_prefabs, build_object_bundles},
    },
    AppState,
};

mod reg_impl;
mod utils;

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ActorRegistry>()
            .register_type::<RegistryIdentifier>()
            .add_systems(OnExit(AppState::Loading), create_actor_registry);
    }
}

#[derive(
    Debug,
    Default,
    Hash,
    Eq,
    PartialEq,
    Clone,
    Deref,
    DerefMut,
    serde::Deserialize,
    serde::Serialize,
    Component,
    Reflect,
)]
pub struct RegistryIdentifier(pub String);

// create items before weapons and weapons before characters
// this way we can use the created weapon bundles too put weapons on the characters
/// Database of all actors that can spawn in the game
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ActorRegistry {
    pub objects: ObjectRegistry,
    pub characters: CharacterRegistry,
}

#[derive(Default, Reflect)]
pub struct CharacterRegistry {
    /// final bad guys
    pub bosses: HashMap<RegistryIdentifier, CharacterBundle>,
    /// generic bad guys
    pub creeps: HashMap<RegistryIdentifier, CharacterBundle>,
    /// flavor characters
    pub critters: HashMap<RegistryIdentifier, CharacterBundle>,
    /// pets for other characters
    pub minions: HashMap<RegistryIdentifier, CharacterBundle>,
    /// characters player can select too play or for followers
    pub heroes: HashMap<RegistryIdentifier, CharacterBundle>,
    /// helpful npcs for player, merchants etc etc etc
    pub freindlies: HashMap<RegistryIdentifier, CharacterBundle>,
}

#[derive(Default, Reflect)]
pub struct ObjectRegistry {
    /// availbe weapons for game
    pub weapons: HashMap<RegistryIdentifier, WeaponBundle>,
}

pub fn create_actor_registry(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    character_definitions: Res<Assets<CharacterDefinition>>,
    weapon_definition: Res<Assets<ObjectDefinition>>,
) {
    let mut registry = ActorRegistry::default();

    build_object_bundles(
        &mut cmds,
        weapon_definition,
        &asset_server,
        &mut registry.objects,
    );

    build_character_prefabs(
        &mut cmds,
        character_definitions,
        asset_server,
        &mut registry.characters,
    );

    cmds.insert_resource(registry);
}

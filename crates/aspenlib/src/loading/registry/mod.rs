use std::fmt::Debug;

use bevy::{
    app::Plugin,
    ecs::{
        component::Component,
        reflect::{ReflectComponent, ReflectResource},
        system::{Res, Resource},
    },
    prelude::{AssetServer, Assets, Commands, OnExit},
    reflect::Reflect,
    utils::HashMap,
};

use crate::{
    bundles::{CharacterBundle, WeaponBundle},
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition},
        registry::utils::{build_character_bundles, build_item_bundles},
    },
    AppState,
};

/// impls for registry and supporting parts
mod reg_impl;
/// misc functions for registry building
mod utils;

/// plugin handles creating of actor registry
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ActorRegistry>()
            .register_type::<RegistryIdentifier>()
            .add_systems(OnExit(AppState::Loading), create_actor_registry);
    }
}

/// ID for all spawnable things in the game, one per spawnable actor
#[derive(
    Default,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    Component,
    Reflect,
)]
#[reflect(Component)]
pub struct RegistryIdentifier(pub String);

// create items before weapons and weapons before characters
// this way we can use the created weapon bundles too put weapons on the characters
/// Database of all actors that can spawn in the game
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ActorRegistry {
    /// database of all carryables
    pub items: ItemRegistry,
    /// database of all characters
    pub characters: CharacterRegistry,
}

/// list of all NPCs for the game, one of the heroes is the player
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

/// list of all useable/equipabble/holdable actors for the game
#[derive(Default, Reflect)]
pub struct ItemRegistry {
    /// availbe weapons for game
    pub weapons: HashMap<RegistryIdentifier, WeaponBundle>,
}

/// creates an actor registry and populates it from actor asset definitons
pub fn create_actor_registry(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    character_definitions: Res<Assets<CharacterDefinition>>,
    weapon_definition: Res<Assets<ItemDefinition>>,
) {
    let mut registry = ActorRegistry::default();

    build_item_bundles(
        &mut cmds,
        weapon_definition,
        &asset_server,
        &mut registry.items,
    );

    build_character_bundles(
        &mut cmds,
        character_definitions,
        asset_server,
        &mut registry.characters,
    );

    cmds.insert_resource(registry);
}

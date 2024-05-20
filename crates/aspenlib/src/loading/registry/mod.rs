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
use rand::prelude::IteratorRandom;

use crate::{
    bundles::{CharacterBundle, WeaponBundle},
    game::{characters::components::CharacterType, items::components::ItemType},
    loading::{
        custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition},
        registry::utils::{build_character_bundles, build_item_bundles},
    },
    register_types, AppState,
};

/// impls for registry and supporting parts
mod reg_impl;
/// misc functions for registry building
mod utils;

/// plugin handles creating of actor registry
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        register_types!(app, [ActorRegistry, RegistryIdentifier]);
        app.add_systems(OnExit(AppState::Loading), create_actor_registry);
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
    pub hero_pets: HashMap<RegistryIdentifier, CharacterBundle>,
    /// characters player can select too play or for followers
    pub heroes: HashMap<RegistryIdentifier, CharacterBundle>,
    /// helpful npcs for player, merchants etc etc etc
    pub freindlies: HashMap<RegistryIdentifier, CharacterBundle>,
}

impl CharacterRegistry {
    /// gets type of character for identifier
    pub fn get_character_type(&self, identifier: &RegistryIdentifier) -> Option<CharacterType> {
        if self.bosses.contains_key(identifier) {
            return Some(CharacterType::Boss);
        } else if self.creeps.contains_key(identifier) {
            return Some(CharacterType::Creep);
        } else if self.critters.contains_key(identifier) {
            return Some(CharacterType::Critter);
        } else if self.freindlies.contains_key(identifier) {
            return Some(CharacterType::Shopkeep);
        } else if self.heroes.contains_key(identifier) {
            return Some(CharacterType::Hero);
        } else if self.hero_pets.contains_key(identifier) {
            return Some(CharacterType::HeroPet);
        }
        None
    }

    /// gets character bundle for requested identifier
    pub fn get_character(&self, identifier: &RegistryIdentifier) -> Option<CharacterBundle> {
        if self.bosses.contains_key(identifier) {
            self.bosses.get(identifier).cloned()
        } else if self.creeps.contains_key(identifier) {
            self.creeps.get(identifier).cloned()
        } else if self.critters.contains_key(identifier) {
            self.critters.get(identifier).cloned()
        } else if self.freindlies.contains_key(identifier) {
            self.freindlies.get(identifier).cloned()
        } else if self.heroes.contains_key(identifier) {
            self.heroes.get(identifier).cloned()
        } else if self.hero_pets.contains_key(identifier) {
            self.hero_pets.get(identifier).cloned()
        } else {
            None
        }
    }

    /// returns random creep identifier
    pub fn random_creep(&self) -> Option<&RegistryIdentifier> {
        let mut rng = rand::thread_rng();
        self.creeps.keys().choose(&mut rng)
    }
}

/// list of all useable/equipabble/holdable actors for the game
#[derive(Default, Reflect)]
pub struct ItemRegistry {
    /// availbe weapons for game
    pub weapons: HashMap<RegistryIdentifier, WeaponBundle>,
    /// availbe armor for game
    pub armor: HashMap<RegistryIdentifier, WeaponBundle>,
    /// availbe special items for game
    pub trinkets: HashMap<RegistryIdentifier, WeaponBundle>,
    /// availbe food style items for game
    pub food: HashMap<RegistryIdentifier, WeaponBundle>,
}

impl ItemRegistry {
    /// returns item type for this identifier, if it exists, else None
    pub fn get_item_type(&self, identifier: &RegistryIdentifier) -> Option<ItemType> {
        if self.weapons.contains_key(identifier) {
            return Some(ItemType::Weapon);
        } else if self.armor.contains_key(identifier) {
            return Some(ItemType::Armor);
        } else if self.trinkets.contains_key(identifier) {
            return Some(ItemType::Trinket);
        } else if self.food.contains_key(identifier) {
            return Some(ItemType::Food);
        }
        None
    }
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

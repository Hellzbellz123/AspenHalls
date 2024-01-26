use std::fmt::Debug;

use bevy::{asset::ReflectAsset, prelude::*};
use bevy_asepritesheet::core::AsepritesheetPlugin;
use bevy_common_assets::{ron::RonAssetPlugin, toml::TomlAssetPlugin};

use crate::{
    game::{
        attributes_stats::{Attributes, Damage, ElementalEffect, PhysicalDamage},
        characters::{ai::components::AiType, components::CharacterType},
        items::weapons::components::{AttackDamage, GunCfg, WeaponDescriptor},
    },
    loading::registry::RegistryIdentifier,
};

/// plugin for actor asset definitions
pub struct ActorAssetPlugin;

impl Plugin for ActorAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_reflect::<CharacterDefinition>()
            .register_asset_reflect::<ItemDefinition>()
            .add_systems(Startup, write_example_definitions)
            .add_plugins((
                // plugin for character definition from files
                TomlAssetPlugin::<CharacterDefinition>::new(&["character.toml"]),
                RonAssetPlugin::<CharacterDefinition>::new(&["character.ron"]),
                // plugin for item definition from files
                TomlAssetPlugin::<ItemDefinition>::new(&["weapon.toml"]),
                RonAssetPlugin::<ItemDefinition>::new(&["weapon.ron"]),
                // actor sprite sheet data
                AsepritesheetPlugin::new(&["sprite.json"]),
            ));
    }
}

/// writes example actor asset definiitons too respective folders on game start
fn write_example_definitions() {
    write_character_def(None);
    write_weapon_def(None);
}

/// character actor asset definition
#[derive(Debug, Asset, Reflect, serde::Deserialize, serde::Serialize)]
#[reflect(Asset)]
pub struct CharacterDefinition {
    /// what type of character is this
    pub character_type: CharacterAssetType,
    /// shared data for all actors
    pub actor: ActorData,
}

/// item actor asset definition
#[derive(Debug, Asset, Reflect, serde::Deserialize, serde::Serialize)]
#[reflect(Asset)]
pub struct ItemDefinition {
    /// info that describes this item
    pub item_type: ItemAssetType,
    /// shared data required for all actors
    pub actor: ActorData,
}

/// shared actor asset data
#[derive(Debug, Reflect, serde::Deserialize, serde::Serialize)]
pub struct ActorData {
    /// actors name
    pub name: String,
    /// actors name
    pub identifier: RegistryIdentifier,
    /// path too aseprite containing animations and images
    pub aseprite_path: String,
    /// optional custom scale for weapon
    pub pixel_size: Vec2,
    /// npc stats
    pub stats: Attributes,
}

/// information used too decide assets function
#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum CharacterAssetType {
    /// - final enemy of dungeon level
    /// - hostile too all npcs
    Boss {
        /// requested ai
        ai: AiType,
    },
    /// - generic enemy for dungeon levels
    /// - passive too creep
    Creep {
        /// requested ai
        ai: AiType,
    },
    /// - runs away from creeps
    /// - passive too self and freindly
    Critter {
        /// requested ai
        ai: AiType,
    },
    /// player pet
    HeroPet {
        /// requested ai
        ai: AiType,
    },
    /// passive too player
    Hero {
        /// requested ai
        ai: AiType,
    },
    /// sells stuff too player
    Shopkeep {
        /// requested ai
        ai: AiType,
    },
}

impl CharacterAssetType {
    /// gets requested ai type for this character asset
    pub const fn get_ai(self) -> AiType {
        match self {
            Self::Boss { ai }
            | Self::Creep { ai }
            | Self::Critter { ai }
            | Self::HeroPet { ai }
            | Self::Hero { ai }
            | Self::Shopkeep { ai } => ai,
        }
    }
    /// gets assets corresponding `CharacterType`
    pub const fn as_charactertype(self) -> CharacterType {
        match self {
            Self::Boss { .. } => CharacterType::Boss,
            Self::Creep { .. } => CharacterType::Creep,
            Self::Critter { .. } => CharacterType::Critter,
            Self::HeroPet { .. } => CharacterType::HeroPet,
            Self::Hero { .. } => CharacterType::Hero,
            Self::Shopkeep { .. } => CharacterType::Shopkeep,
        }
    }
}

/// different classes of items that can exist in the game
#[derive(Debug, Copy, Clone, Reflect, serde::Deserialize, serde::Serialize)]
pub enum ItemAssetType {
    /// items that the holder can attack with
    Weapon {
        /// weapon damage
        damage: AttackDamage,
        /// weapon form and function descriptor
        form: WeaponDescriptor,
    },
    /// items that give the holder armor and attrs
    Armor {},
    /// items that give the holder small bonus / unique effects
    Trinket {},
    /// items that give the user status effects
    Food {},
}

/// creates new weapon definition folder
#[allow(unused)]
pub fn write_weapon_def(def: Option<ItemDefinition>) {
    let def = def.unwrap_or(ItemDefinition {
        item_type: ItemAssetType::Weapon {
            damage: AttackDamage(Damage {
                physical: PhysicalDamage(30.0),
                elemental: ElementalEffect::Fire(10.0),
            }),
            form: WeaponDescriptor::Gun(GunCfg {
                projectile_speed: 50.0,
                projectile_size: 15.0,
                barrel_end: Vec2 { x: 20.0, y: 0.0 },
                max_ammo: 50,
                reload_time: 1.5,
                fire_rate: 0.25,
            }),
        },
        actor: ActorData {
            name: "ExampleWeapon".to_owned(),
            identifier: RegistryIdentifier("exampleweapon".to_owned()),
            aseprite_path: "sprite_sheet.png".to_owned(),
            pixel_size: Vec2 { x: 32.0, y: 32.0 },
            stats: Attributes::WEAPON_DEFAULT,
        },
    });

    let folder_path = format!("assets/packs/asha/items/w{}", def.actor.identifier.0);
    let toml_path = format!("{}/{}.weapon.toml", folder_path, def.actor.identifier.0);
    let ron_path = format!("{}/{}.weapon.ron", folder_path, def.actor.identifier.0);
    write_definition(def, folder_path, ron_path, toml_path);
}

/// creates new character definition folder
#[allow(unused)]
pub fn write_character_def(def: Option<CharacterDefinition>) {
    let def = def.unwrap_or(CharacterDefinition {
        character_type: CharacterAssetType::Creep { ai: AiType::Stupid },
        actor: ActorData {
            name: "ExampleNpc".to_owned(),
            identifier: RegistryIdentifier("examplenpc".to_owned()),
            aseprite_path: "sprite_sheet.png".to_owned(),
            pixel_size: Vec2 { x: 32.0, y: 32.0 },
            stats: Attributes::CREEP_DEFAULT,
        },
    });
    let folder_path = format!("assets/packs/asha/characters/{}", def.actor.identifier.0);
    let ron_path = format!("{}/{}.npc.ron", folder_path, def.actor.identifier.0);
    let toml_path = format!("{}/{}.npc.toml", folder_path, def.actor.identifier.0);
    write_definition(def, folder_path, ron_path, toml_path);
}

//TODO: move folder formatting too write fn, only pass string and def too write_definition
/// writes asset definiiton too file
fn write_definition<T: Sized + serde::Serialize>(
    def: T,
    folder_path: String,
    ron_path: String,
    toml_path: String,
) {
    let Ok(toml) = toml::to_string(&def) else {
        warn!("could not deserialize the asset");
        return;
    };
    let Ok(ron) = ron::to_string(&def) else {
        warn!("");
        return;
    };
    match std::fs::create_dir(folder_path) {
        Ok(()) => {
            match std::fs::write(ron_path, ron) {
                Ok(()) => {
                    warn!("Wrote new definition");
                }
                Err(e) => {
                    warn!("Couldnt write definiton: {}", e);
                }
            };
            match std::fs::write(toml_path, toml) {
                Ok(()) => {
                    warn!("Wrote new definition");
                }
                Err(e) => {
                    warn!("Couldnt write definiton: {}", e);
                }
            };
        }
        Err(e) => {
            warn!("Error making new definition folder: {}", e);
        }
    };
}

// TODO: this actor asset definition structure might be better

// #[derive(Debug, Default, Asset, Component, Reflect, serde::Deserialize, serde::Serialize)]
// #[reflect(Asset, Component)]
// pub struct ActorDefinition {
//     /// actor display name
//     pub name: String,
//     /// actor registry identifier
//     pub identifier: RegistryIdentifier,
//     /// path too aseprite json containing animation and image data
//     pub aseprite_path: String,
//     /// optional custom scale for weapon
//     pub pixel_size: Vec2,
//     /// unique actor data
//     pub unique_data: ActorDefType,
// }

// #[derive(Debug, Reflect, serde::Deserialize, serde::Serialize)]
// pub enum ActorDefType {
//     /// this actor is a character
//     /// - let (character_type, controller, stats) = ActorDefType else {warn!("actor is not a character"); return}
//     Character {
//         character_type: CharacterType,
//         controller: AiSetupConfig,
//         stats: Attributes,
//     },
//     /// this actor is a weapon
//     Weapon {
//         /// how much weapon this damage does on attack
//         damage: AttackDamage,
//         /// what kind of weapon this is
//         form: WeaponDescriptor,
//         /// how much this weapon improves its holders stats
//         stats: Attributes,
//     },
// }

// impl ActorDefType {
//     pub const DEFAULT_WEAPON: Self = Self::Weapon {
//         damage: AttackDamage(Damage {
//             physical: PhysicalDamage(10.0),
//             elemental: ElementalEffect::None,
//         }),
//         form: WeaponDescriptor::Gun {
//             projectile_speed: 10.0,
//             projectile_size: 2.0,
//             barrel_end: Vec2 { x: 0.0, y: 0.0 },
//             ammo_amount: 7,
//             reload_time: 1.0,
//             fire_rate: 0.02,
//         },
//         stats: Attributes::WEAPON_DEFAULT,
//     };

//     pub const DEFAULT_CHARACTER: Self = Self::Character {
//         character_type: CharacterType::Npc(NpcType::Creep),
//         controller: AiSetupConfig::GameAI(AiType::Stupid),
//         stats: Attributes::CREEP_DEFAULT,
//     };
// }

// impl Default for ActorDefType {
//     fn default() -> Self {
//         Self::DEFAULT_WEAPON
//     }
// }

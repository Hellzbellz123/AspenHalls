use std::fmt::Debug;

use bevy::{
    app::Plugin,
    asset::{Asset, ReflectAsset},
    ecs::component::Component,
    log::warn,
    math::Vec2,
    prelude::{AssetApp, Startup},
    reflect::Reflect,
};

use bevy_common_assets::toml::TomlAssetPlugin;

use crate::{
    game::actors::{
        ai::components::AiType,
        attributes_stats::{Attributes, Damage, EffectQueue, ElementalEffect, PhysicalDamage},
        combat::components::{AttackDamage, WeaponForm},
    },
    loading::registry::RegistryIdentifier,
    prelude::game::{ActorType, NpcType},
};

pub struct ActorAssetPlugin;

impl Plugin for ActorAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_reflect::<CharacterDefinition>()
            .register_asset_reflect::<ObjectDefinition>()
            .add_systems(Startup, write_example_definitions)
            .add_plugins(TomlAssetPlugin::<CharacterDefinition>::new(&[
                "character.toml",
            ]))
            .add_plugins(TomlAssetPlugin::<ObjectDefinition>::new(&["weapon.toml"]));
    }
}

fn write_example_definitions() {
    write_character_def(None);
    write_weapon_def(None);
}

#[derive(Debug, Asset, Reflect, serde::Deserialize, serde::Serialize)]
#[reflect(Asset)]
pub struct CharacterDefinition {
    /// what type of character is this
    pub character_type: CharacterType,
    /// does ai or player control this character
    pub controller: AiSetupConfig,
    /// generic data for all actors
    pub actor: ActorData,
}

#[derive(Debug, Asset, Reflect, serde::Deserialize, serde::Serialize)]
#[reflect(Asset)]
pub struct ObjectDefinition {
    pub object_type: ObjectType,
    pub actor: ActorData,
}

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

#[derive(Debug, Clone, Component, Reflect, serde::Deserialize, serde::Serialize)]
pub enum CharacterType {
    Hero,
    Npc(NpcType),
}

#[derive(Debug, Clone, Component, Reflect, serde::Deserialize, serde::Serialize)]
pub enum AiSetupConfig {
    Player,
    GameAI(AiType),
}

#[derive(Debug, Copy, Clone, Reflect, serde::Deserialize, serde::Serialize)]
pub enum ObjectType {
    Weapon {
        damage: AttackDamage,
        form: WeaponForm,
    },
    Trinket {},
    Armor {},
    Food {},
}

impl CharacterType {
    pub fn into_actor_type(&self) -> ActorType {
        match self {
            CharacterType::Hero => ActorType::Hero,
            CharacterType::Npc(a) => ActorType::Npc(*a),
        }
    }
}

/// creates new weapon definition folder
#[allow(unused)]
pub fn write_weapon_def(def: Option<ObjectDefinition>) {
    let def = def.unwrap_or(ObjectDefinition {
        object_type: ObjectType::Weapon {
            damage: AttackDamage(Damage {
                physical: PhysicalDamage(30.0),
                elemental: ElementalEffect::Fire(10.0),
            }),
            form: WeaponForm::Gun {
                projectile_speed: 50.0,
                projectile_size: 15.0,
                barrel_end: Vec2 { x: 20.0, y: 0.0 },
                ammo_amount: 50,
                reload_time: 1.5,
                fire_rate: 0.25,
            },
        },
        actor: ActorData {
            name: "ExampleWeapon".to_owned(),
            identifier: RegistryIdentifier("exampleweapon".to_owned()),
            aseprite_path: "sprite_sheet.png".to_owned(),
            pixel_size: Vec2 { x: 32.0, y: 32.0 },
            stats: Attributes::WEAPON_DEFAULT,
        },
    });

    let folder_path = format!("assets/packs/asha/objects/w{}", def.actor.identifier.0);
    let file_path = format!("{}/{}.weapon.toml", folder_path, def.actor.identifier.0);

    write_definition(def, folder_path, file_path);
}

/// creates new character definition folder
#[allow(unused)]
pub fn write_character_def(def: Option<CharacterDefinition>) {
    let def = def.unwrap_or(CharacterDefinition {
        character_type: CharacterType::Npc(NpcType::Creep),
        controller: AiSetupConfig::GameAI(AiType::Stupid),
        actor: ActorData {
            name: "ExampleNpc".to_owned(),
            identifier: RegistryIdentifier("examplenpc".to_owned()),
            aseprite_path: "sprite_sheet.png".to_owned(),
            pixel_size: Vec2 { x: 32.0, y: 32.0 },
            stats: Attributes::CREEP_DEFAULT,
        },
    });
    let folder_path = format!("assets/packs/asha/characters/{}", def.actor.identifier.0);
    let file_path = format!("{}/{}.npc.toml", folder_path, def.actor.identifier.0);

    write_definition(def, folder_path, file_path);
}

//TODO: move folder formatting too write fn, only pass string and def too write_definition
/// writes asset definiiton too file
fn write_definition<T: Sized + serde::Serialize>(def: T, folder_path: String, file_path: String) {
    let Ok(toml) = toml::to_string(&def) else {
        warn!("could not deserialize the asset");
        return;
    };
    match std::fs::create_dir(folder_path) {
        Ok(()) => {
            match std::fs::write(file_path, toml) {
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
//         form: WeaponForm,
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
//         form: WeaponForm::Gun {
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

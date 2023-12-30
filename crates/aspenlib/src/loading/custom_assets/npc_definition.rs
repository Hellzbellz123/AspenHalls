use std::fmt::Debug;

use bevy::{
    app::{Plugin, Update},
    asset::{Asset, LoadedFolder, RecursiveDependencyLoadState, ReflectAsset},
    core::Name,
    ecs::{
        component::Component,
        reflect::{ReflectComponent, ReflectResource},
        schedule::OnEnter,
        system::Res,
    },
    log::{error, info, warn},
    math::{vec2, Vec2},
    prelude::{
        default, state_exists_and_equals, AssetApp, AssetServer, Assets, Commands, Handle, Image,
        IntoSystemConfigs, Query, ResMut, Resource, With,
    },
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    utils::HashMap,
};

use bevy_asepritesheet::{
    animator::AnimatedSpriteBundle,
    core::load_spritesheet,
    prelude::{AnimHandle, SpriteAnimator},
};
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::{
    dynamics::{Damping, LockedAxes, RigidBody, Velocity},
    geometry::{ColliderMassProperties, Friction, Restitution},
};

use crate::{
    bundles::{CharacterBundle, RigidBodyBundle},
    game::actors::{
        ai::components::AiType,
        attributes_stats::{
            Attributes, CharacterStatBundle, CharacterStats, Damage, ElementalEffect,
            EquipmentStats, PhysicalDamage,
        },
        combat::components::{AttackDamage, WeaponBundle, WeaponForm},
        components::ActorMoveState,
    },
    prelude::game::{ActorType, NpcType, WeaponHolder},
    AppState,
};

// create items before weapons and weapons before characters
// this way we can use the created weapon bundles too put weapons on the characters
/// Database of all actors that can spawn in the game
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ActorRegistry {
    pub characters: CharacterRegistry,
    pub objects: ObjectRegistry,
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

#[derive(
    Debug,
    Default,
    Hash,
    Eq,
    PartialEq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    Component,
    Reflect,
)]
pub struct RegistryIdentifier(pub String);

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
pub struct WeaponDefinition {
    /// how much damage weapon does when attack hits
    pub damage: AttackDamage,
    /// what weapon does when character attacks with it
    pub weapon_type: WeaponForm,
    /// generic data for all actors
    pub actor: ActorData,
}

#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct LoadedDataFolders {
    pub characters: Handle<LoadedFolder>,
    pub weapons: Handle<LoadedFolder>,
    pub items: Handle<LoadedFolder>,
}

pub struct ActorRegistryPlugin;

impl Plugin for ActorRegistryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TomlAssetPlugin::<CharacterDefinition>::new(&[
            "character.toml",
        ]))
        .add_plugins(TomlAssetPlugin::<WeaponDefinition>::new(&["weapon.toml"]))
        .register_asset_reflect::<CharacterDefinition>()
        .register_asset_reflect::<WeaponDefinition>()
        .register_type::<LoadedDataFolders>()
        .register_type::<ActorRegistry>()
        .register_type::<RegistryIdentifier>()
        .add_systems(OnEnter(AppState::BootingApp), load_data_folders)
        .add_systems(OnEnter(AppState::Loading), create_actor_registry)
        .add_systems(
            Update,
            update_character_size.run_if(state_exists_and_equals(AppState::PlayingGame)),
        );
    }
}

fn load_data_folders(asset_server: ResMut<AssetServer>, mut cmds: Commands) {
    let characters = asset_server.load_folder("packs/asha/characters");
    let weapons = asset_server.load_folder("packs/asha/weapons");
    let items = asset_server.load_folder("packs/asha/items");

    cmds.insert_resource(LoadedDataFolders {
        characters,
        weapons,
        items,
    });
}

fn create_actor_registry(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    folders: Res<LoadedDataFolders>,
    character_definitions: Res<Assets<CharacterDefinition>>,
    weapon_definition: Res<Assets<WeaponDefinition>>,
) {
    let char_state = asset_server
        .get_recursive_dependency_load_state(folders.characters.clone())
        .unwrap();
    let weap_state = asset_server
        .get_recursive_dependency_load_state(folders.weapons.clone())
        .unwrap();

    if weap_state == RecursiveDependencyLoadState::Failed {
        error!("Weapon definitions did not load properly");
    } else if char_state == RecursiveDependencyLoadState::Failed {
        error!("Character Definitions did not load properly");
    }

    let mut weapons: HashMap<RegistryIdentifier, WeaponBundle> = HashMap::new();
    build_weapon_prefabs(
        &mut cmds,
        weapon_definition,
        &asset_server,
        &mut texture_atlases,
        &mut weapons,
    );

    let mut bosses: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();
    let mut creeps: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();
    let mut critters: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();
    let mut minions: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();
    let mut heroes: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();
    let mut freindlies: HashMap<RegistryIdentifier, CharacterBundle> = HashMap::new();

    build_character_prefabs(
        &mut cmds,
        character_definitions,
        asset_server,
        &mut bosses,
        &mut creeps,
        &mut critters,
        &mut minions,
        &mut heroes,
        &mut freindlies,
    );

    cmds.insert_resource(ActorRegistry {
        objects: ObjectRegistry { weapons },
        characters: CharacterRegistry {
            bosses,
            creeps,
            critters,
            minions,
            heroes,
            freindlies,
        },
    });
}

fn build_weapon_prefabs(
    mut cmds: &mut Commands,
    weapon_definition: Res<'_, Assets<WeaponDefinition>>,
    asset_server: &Res<'_, AssetServer>,
    texture_atlases: &mut ResMut<'_, Assets<TextureAtlas>>,
    weapons: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, WeaponBundle>,
) {
    for (id, definition) in weapon_definition.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_path = folder_path.join(definition.actor.aseprite_path.clone());

        let sprite_handle: Handle<Image> = asset_server.load(sprite_path);
        let new_atlas =
            TextureAtlas::from_grid(sprite_handle, Vec2 { x: 64.0, y: 64.0 }, 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(new_atlas);

        let weapon_bundle = WeaponBundle {
            name: Name::new(definition.actor.name.clone()),
            holder: WeaponHolder::default(),
            damage: definition.damage.clone(),
            weapon_type: definition.weapon_type.clone(),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(definition.actor.pixel_size),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                rigidbody: RigidBody::default(),
                velocity: Velocity::default(),
                friction: Friction::default(),
                how_bouncy: Restitution::default(),
                mass_prop: ColliderMassProperties::default(),
                rotation_locks: LockedAxes::default(),
                damping_prop: Damping::default(),
            },
            stats: EquipmentStats::from_attrs(definition.actor.stats, None),
        };

        weapons.insert(definition.actor.identifier.clone(), weapon_bundle);
    }
}

fn build_character_prefabs(
    mut cmds: &mut Commands,
    character_definitions: Res<'_, Assets<CharacterDefinition>>,
    asset_server: Res<'_, AssetServer>,
    bosses: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
    creeps: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
    critters: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
    minions: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
    heroes: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
    freindlies: &mut bevy::utils::hashbrown::HashMap<RegistryIdentifier, CharacterBundle>,
) {
    for (id, character_def) in character_definitions.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(character_def.actor.aseprite_path.clone());

        info!("loading sprite json: {:?}", sprite_json_path);
        // load the spritesheet and get it's handle
        let sheet_handle = load_spritesheet(
            &mut cmds,
            &asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::TopCenter,
        );

        let actor_bundle = CharacterBundle {
            name: Name::new(character_def.actor.name.clone()),
            identifier: character_def.actor.identifier.clone(),
            actor_type: character_def.character_type.into_actor_type(),
            stats: CharacterStatBundle::from_attrs(character_def.actor.stats),
            move_state: ActorMoveState::DEFAULT,
            aseprite: AnimatedSpriteBundle {
                spritesheet: sheet_handle,
                animator: SpriteAnimator::from_anim(AnimHandle::from_index(0)),
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle::ENEMY,
            controller: character_def.controller.clone(),
        };

        match character_def.character_type {
            CharacterType::Npc(cpt) => match cpt {
                NpcType::Boss => {
                    bosses.insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Creep => {
                    creeps.insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Critter => {
                    critters.insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Friendly => {
                    freindlies.insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Minion => {
                    minions.insert(character_def.actor.identifier.clone(), actor_bundle);
                }
            },
            CharacterType::Hero => {
                heroes.insert(character_def.actor.identifier.clone(), actor_bundle);
            }
        }
        continue;
    }
}

fn update_character_size(
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &RegistryIdentifier,
        ),
        With<CharacterStats>,
    >,
    texture_atlass: Res<Assets<TextureAtlas>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    for (mut sprite, texture_atlas, registry_identifier) in &mut query {
        if sprite.custom_size.is_some() {
            continue;
        }

        let (_, char_def) = char_assets
            .iter()
            .find(|(_, asset)| asset.actor.identifier == *registry_identifier)
            .expect("Spawned characters asset definition did not exist");

        let atlas = texture_atlass
            .get(texture_atlas)
            .expect("texture for this spritesheet is missing");
        let original_size = atlas.textures.first().expect("no textures in atlas").size();
        let aspect_ratio = original_size.x / original_size.y;

        info!(
            "image size: {}, aspect ratio: {}",
            original_size, aspect_ratio
        );

        let final_size = char_def.actor.pixel_size;
        let new_custom_size = scale_to_fit(original_size, final_size);

        info!(
            "target size: {}, new_custom_size: {}",
            final_size, new_custom_size
        );
        sprite.custom_size = Some(new_custom_size)
    }
}

fn scale_to_fit(current: Vec2, final_size: Vec2) -> Vec2 {
    // Calculate scaling factors for both dimensions
    let scale_x = final_size.x / current.x;
    let scale_y = final_size.y / current.y;

    // Use the minimum scaling factor to maintain aspect ratio
    let min_scale = scale_x.min(scale_y);

    // Scale the Vec2
    let scaled_vec = Vec2 {
        x: current.x * min_scale,
        y: current.y * min_scale,
    };

    // Return the scaled Vec2
    scaled_vec
}

/// creates new character definition folder
#[allow(unused)]
pub fn write_npc_definition(def: Option<CharacterDefinition>) {
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
    let folder_path = format!("assets/packs/asha/characters/{}", def.actor.name);
    let file_path = format!("{}/{}.npc.toml", folder_path, def.actor.name);

    write_definition(def, folder_path, file_path);
}

/// creates new weapon definition folder
#[allow(unused)]
pub fn write_weapon_definition(def: Option<WeaponDefinition>) {
    let def = def.unwrap_or(WeaponDefinition {
        damage: AttackDamage(Damage {
            physical: PhysicalDamage(50.0),
            elemental: ElementalEffect::Fire(10.0),
        }),
        weapon_type: WeaponForm::Gun {
            projectile_speed: 50.0,
            projectile_size: 20.0,
            barrel_end: Vec2 { x: 0.0, y: 0.0 },
            ammo_amount: 5,
            reload_time: 1.7,
            fire_rate: 0.37,
        },
        actor: ActorData {
            name: "ExampleWeapon".to_owned(),
            identifier: RegistryIdentifier("exampleweapon".to_owned()),
            aseprite_path: "sprite_sheet.png".to_owned(),
            pixel_size: Vec2 { x: 32.0, y: 32.0 },
            stats: Attributes::WEAPON_DEFAULT,
        },
    });

    let folder_path = format!("assets/packs/asha/weapons/{}", def.actor.name);
    let file_path = format!("{}/{}.weapon.toml", folder_path, def.actor.name);

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

impl CharacterType {
    pub fn into_actor_type(&self) -> ActorType {
        match self {
            CharacterType::Hero => ActorType::Hero,
            CharacterType::Npc(a) => ActorType::Npc(*a),
        }
    }
}

impl Debug for ActorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec_creep = &self
            .characters
            .creeps
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        let vec_heroes = &self
            .characters
            .heroes
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        let vec_items = &self
            .objects
            .weapons
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        f.debug_struct("ActorRegistry")
            .field("creeps", vec_creep)
            .field("heroes", vec_heroes)
            .field("weapons", vec_items)
            .finish()
    }
}

impl From<String> for RegistryIdentifier {
    fn from(s: String) -> Self {
        RegistryIdentifier(s)
    }
}


// TODO: the below actor asset definition might be better
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

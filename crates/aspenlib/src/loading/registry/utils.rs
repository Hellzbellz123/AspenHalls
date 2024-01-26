use std::path::PathBuf;

use bevy::{
    core::Name,
    ecs::system::Res,
    log::info,
    prelude::{default, AssetServer, Assets, Commands, Handle},
};
use bevy_asepritesheet::{
    animator::AnimatedSpriteBundle,
    core::load_spritesheet,
    prelude::{load_spritesheet_then, AnimHandle, SpriteAnimator},
    sprite::Spritesheet,
};
use bevy_rapier2d::{
    dynamics::{Damping, LockedAxes, RigidBody, Velocity},
    geometry::{ColliderMassProperties, Friction, Restitution},
};

use crate::{
    bundles::{CharacterBundle, RigidBodyBundle, WeaponBundle},
    game::{
        attributes_stats::{Attributes, CharacterStatBundle, EquipmentStats},
        characters::{components::CharacterMoveState, utils::format_character_animations},
        items::weapons::{
            components::{AttackDamage, WeaponDescriptor, WeaponHolder},
            forms::format_gun_animations,
        },
    },
    loading::{
        custom_assets::actor_definitions::{CharacterAssetType, ItemAssetType},
        registry::{CharacterDefinition, ItemDefinition, RegistryIdentifier},
        registry::{CharacterRegistry, ItemRegistry},
    },
};

/// adds characters too `CharacterRegistry` with character definitions loaded from disk
pub fn build_character_bundles(
    cmds: &mut Commands,
    character_definitions: Res<'_, Assets<CharacterDefinition>>,
    asset_server: Res<'_, AssetServer>,
    character_registry: &mut CharacterRegistry,
) {
    for (id, character_def) in character_definitions.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(character_def.actor.aseprite_path.clone());

        info!("loading sprite json: {:?}", sprite_json_path);
        // load the spritesheet and get it's handle
        let sheet_handle = load_spritesheet_then(
            cmds,
            &asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::TopCenter,
            |sheet| format_character_animations(sheet)
        );

        let actor_bundle = CharacterBundle {
            name: Name::new(character_def.actor.name.clone()),
            identifier: character_def.actor.identifier.clone(),
            actor_type: character_def.character_type.as_charactertype(),
            stats: CharacterStatBundle::from_attrs(character_def.actor.stats),
            move_state: CharacterMoveState::DEFAULT,
            aseprite: AnimatedSpriteBundle {
                spritesheet: sheet_handle,
                animator: SpriteAnimator::from_anim(AnimHandle::from_index(0)),
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle::DEFAULT_CHARACTER,
            controller: character_def.character_type.get_ai(),
        };

        match character_def.character_type {
            CharacterAssetType::Boss { .. } => {
                character_registry
                    .bosses
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
            CharacterAssetType::Creep { .. } => {
                character_registry
                    .creeps
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
            CharacterAssetType::Critter { .. } => {
                character_registry
                    .critters
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
            CharacterAssetType::HeroPet { .. } => {
                character_registry
                    .hero_pets
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
            CharacterAssetType::Hero { .. } => {
                character_registry
                    .heroes
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
            CharacterAssetType::Shopkeep { .. } => {
                todo!()
            }
        }
        continue;
    }
}

/// adds items too `ItemRegistry` with item definitions loaded from disk
pub fn build_item_bundles(
    mut cmds: &mut Commands,
    item_defs: Res<'_, Assets<ItemDefinition>>,
    asset_server: &Res<'_, AssetServer>,
    item_registry: &mut ItemRegistry,
) {
    for (id, definition) in item_defs.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(definition.actor.aseprite_path.clone());

        match definition.item_type {
            ItemAssetType::Weapon { damage, form } => {
                insert_weapon_into_registry(
                    &mut cmds,
                    &asset_server,
                    sprite_json_path,
                    item_registry,
                    definition.actor.name.clone(),
                    definition.actor.identifier.clone(),
                    damage,
                    form,
                    definition.actor.stats,
                );
            }
            ItemAssetType::Trinket {} => todo!("trinket items not implmented"),
            ItemAssetType::Armor {} => todo!("armor items not implmented"),
            ItemAssetType::Food {} => todo!("food items not implmented"),
        }
    }
}

/// creates weapon bundle from an item definition and then adds it too item registry
fn insert_weapon_into_registry(
    cmds: &mut Commands,
    asset_server: &Res<'_, AssetServer>,
    sprite_json_path: PathBuf,
    item_registry: &mut ItemRegistry,
    name: String,
    identifier: RegistryIdentifier,
    damage: AttackDamage,
    weapon_type: WeaponDescriptor,
    stats: Attributes,
) {
    let sheet_handle = match weapon_type {
        WeaponDescriptor::Gun { .. } => load_spritesheet_then(
            cmds,
            asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::Center,
            |sheet| format_gun_animations(sheet),
        ),
    };

    item_registry.weapons.insert(
        identifier.clone(),
        WeaponBundle {
            name: Name::new(name),
            identifier,
            holder: WeaponHolder::default(),
            damage,
            weapon_type,
            sprite: AnimatedSpriteBundle {
                spritesheet: sheet_handle,
                animator: SpriteAnimator::from_anim(AnimHandle::from_index(1)),
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
            stats: EquipmentStats::from_attrs(stats, None),
        },
    );
}

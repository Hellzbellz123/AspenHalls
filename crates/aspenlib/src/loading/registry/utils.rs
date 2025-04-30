use std::path::PathBuf;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_aseprite_ultra::prelude::{Animation, AseSpriteAnimation};

use crate::{
    bundles::{Aspen2dRenderBundle, CharacterBundle, WeaponBundle},
    game::{
        attributes_stats::{Attributes, CharacterStatBundle, EquipmentStats},
        characters::components::CharacterMoveState,
        items::weapons::components::{AttackDamage, WeaponDescriptor, WeaponHolder},
    },
    loading::{
        custom_assets::actor_definitions::{CharacterAssetType, ItemAssetType},
        registry::{
            CharacterDefinition, CharacterRegistry, ItemDefinition, ItemRegistry,
            RegistryIdentifier,
        },
    },
};

/// adds characters too `CharacterRegistry` with character definitions loaded from disk
pub fn build_character_bundles(
    character_definitions: Res<'_, Assets<CharacterDefinition>>,
    asset_server: &ResMut<AssetServer>,
    character_registry: &mut CharacterRegistry,
) {
    for (id, character_def) in character_definitions.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let aseprite_path = folder_path.join(character_def.actor.aseprite_path.clone());
        let aseprite_handle = asset_server.load(aseprite_path);

        let actor_bundle = CharacterBundle {
            name: Name::new(character_def.actor.name.clone()),
            identifier: character_def.actor.identifier.clone(),
            actor_type: character_def.character_type.as_charactertype(),
            stats: CharacterStatBundle::from_attrs(character_def.actor.stats),
            move_state: CharacterMoveState::DEFAULT,
            controller: character_def.character_type.get_ai(),
            render: Aspen2dRenderBundle {
                handle: AseSpriteAnimation {
                    animation: Animation::default().with_tag("idle"),
                    aseprite: aseprite_handle,
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(character_def.actor.tile_size)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                ..default()
            },
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

// TODO: are weapons items?
/// adds items too `ItemRegistry` with item definitions loaded from disk
pub fn build_item_bundles(
    item_defs: Res<'_, Assets<ItemDefinition>>,
    asset_server: &ResMut<AssetServer>,
    item_registry: &mut ItemRegistry,
) {
    for (id, definition) in item_defs.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(definition.actor.aseprite_path.clone());

        match definition.item_type {
            ItemAssetType::Weapon { damage, form } => {
                let weapon = form_weapon_bundle(
                    asset_server,
                    (definition.actor.identifier.clone(), sprite_json_path),
                    definition.actor.name.clone().into(),
                    damage,
                    form,
                    definition.actor.stats,
                    definition.actor.tile_size,
                );
                item_registry
                    .weapons
                    .insert(weapon.identifier.clone(), weapon);
            }
            ItemAssetType::Trinket {} => todo!("trinket items not implmented"),
            ItemAssetType::Armor {} => todo!("armor items not implmented"),
            ItemAssetType::Food {} => todo!("food items not implmented"),
        }
    }
}

/// creates weapon bundle from an item definition and then adds it too item registry
fn form_weapon_bundle(
    asset_server: &ResMut<'_, AssetServer>,
    asset_data: (RegistryIdentifier, PathBuf),
    name: Name,
    damage: AttackDamage,
    descriptor: WeaponDescriptor,
    attributes: Attributes,
    tile_size: f32,
) -> WeaponBundle {
    let aseprite_handle = asset_server.load(asset_data.1);

    WeaponBundle {
        name,
        identifier: asset_data.0,
        holder: WeaponHolder::default(),
        damage,
        weapon_type: descriptor,
        stats: EquipmentStats::from_attrs(attributes, None),
        render: Aspen2dRenderBundle {
            handle: AseSpriteAnimation {
                animation: Animation::default().with_tag("idle"),
                aseprite: aseprite_handle,
            },
            sprite: Sprite {
                anchor: Anchor::default(),
                custom_size: Some(Vec2::splat(tile_size)),
                ..default()
            },
            ..default()
        },
    }
}

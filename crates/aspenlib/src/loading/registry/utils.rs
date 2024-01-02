use bevy::{
    core::Name,
    ecs::system::Res,
    log::info,
    prelude::{default, AssetServer, Assets, Commands, Handle},
};
use bevy_asepritesheet::{
    animator::AnimatedSpriteBundle,
    core::load_spritesheet,
    prelude::{AnimHandle, SpriteAnimator},
    sprite::Spritesheet,
};
use bevy_rapier2d::{
    dynamics::{Damping, LockedAxes, RigidBody, Velocity},
    geometry::{ColliderMassProperties, Friction, Restitution},
};

use crate::{
    bundles::{CharacterBundle, RigidBodyBundle, WeaponBundle},
    game::actors::{
        attributes_stats::{Attributes, CharacterStatBundle, EquipmentStats},
        combat::components::{AttackDamage, WeaponDescriptor},
        components::ActorMoveState,
    },
    loading::{
        custom_assets::actor_definitions::{CharacterType, ItemType},
        registry::{CharacterDefinition, ItemDefinition, RegistryIdentifier},
        registry::{CharacterRegistry, ItemRegistry},
    },
    prelude::game::{NpcType, WeaponHolder},
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
        let sheet_handle = load_spritesheet(
            cmds,
            &asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::TopCenter,
        );

        let actor_bundle = CharacterBundle {
            name: Name::new(character_def.actor.name.clone()),
            identifier: character_def.actor.identifier.clone(),
            actor_type: character_def.character_type.clone().into_actor_type(),
            stats: CharacterStatBundle::from_attrs(character_def.actor.stats),
            move_state: ActorMoveState::DEFAULT,
            aseprite: AnimatedSpriteBundle {
                spritesheet: sheet_handle,
                animator: SpriteAnimator::from_anim(AnimHandle::from_index(0)),
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle::DEFAULT_CHARACTER,
            controller: character_def.controller.clone(),
        };

        match character_def.character_type {
            CharacterType::Npc(cpt) => match cpt {
                NpcType::Boss => {
                    character_registry
                        .bosses
                        .insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Creep => {
                    character_registry
                        .creeps
                        .insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Critter => {
                    character_registry
                        .critters
                        .insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Friendly => {
                    character_registry
                        .freindlies
                        .insert(character_def.actor.identifier.clone(), actor_bundle);
                }
                NpcType::Minion => {
                    character_registry
                        .minions
                        .insert(character_def.actor.identifier.clone(), actor_bundle);
                }
            },
            CharacterType::Hero => {
                character_registry
                    .heroes
                    .insert(character_def.actor.identifier.clone(), actor_bundle);
            }
        }
        continue;
    }
}

/// adds items too `ItemRegistry` with item definitions loaded from disk
pub fn build_item_bundles(
    cmds: &mut Commands,
    item_defs: Res<'_, Assets<ItemDefinition>>,
    asset_server: &Res<'_, AssetServer>,
    item_registry: &mut ItemRegistry,
) {
    for (id, definition) in item_defs.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(definition.actor.aseprite_path.clone());

        let sheet_handle = load_spritesheet(
            cmds,
            asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::Center,
        );

        match definition.item_type {
            ItemType::Weapon { damage, form } => {
                insert_weapon_into_registry(
                    item_registry,
                    definition.actor.name.clone(),
                    definition.actor.identifier.clone(),
                    damage,
                    form,
                    definition.actor.stats,
                    sheet_handle,
                );
            }
            ItemType::Trinket {} => todo!("trinket items not implmented"),
            ItemType::Armor {} => todo!("armor items not implmented"),
            ItemType::Food {} => todo!("food items not implmented"),
        }
    }
}

/// creates weapon bundle from an item definition and then adds it too item registry
fn insert_weapon_into_registry(
    item_registry: &mut ItemRegistry,
    name: String,
    identifier: RegistryIdentifier,
    damage: AttackDamage,
    weapon_type: WeaponDescriptor,
    stats: Attributes,
    sheet_handle: Handle<Spritesheet>,
) {
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

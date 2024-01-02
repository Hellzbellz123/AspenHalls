use std::{fmt::Debug, marker::PhantomData};

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
        IntoSystemConfigs, OnExit, Query, ResMut, Resource, With,
    },
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    utils::HashMap,
};

use bevy_asepritesheet::{
    animator::AnimatedSpriteBundle,
    core::load_spritesheet,
    prelude::{AnimHandle, SpriteAnimator},
    sprite::{Anim, Spritesheet},
};
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_rapier2d::{
    dynamics::{Damping, LockedAxes, RigidBody, Velocity},
    geometry::{ColliderMassProperties, Friction, Restitution},
};

use crate::{
    bundles::{CharacterBundle, WeaponBundle, RigidBodyBundle},
    game::actors::{
        attributes_stats::{CharacterStatBundle, EquipmentStats},
        combat::components::{AttackDamage, WeaponForm},
        components::ActorMoveState,
    },
    loading::{
        custom_assets::actor_definitions::{CharacterType, ObjectType},
        registry::{CharacterDefinition, ObjectDefinition},
        registry::{CharacterRegistry, ObjectRegistry, RegistryIdentifier},
    },
    prelude::game::{NpcType, WeaponHolder},
};

pub fn build_character_prefabs(
    mut cmds: &mut Commands,
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

pub fn build_object_bundles(
    mut cmds: &mut Commands,
    object_defs: Res<'_, Assets<ObjectDefinition>>,
    asset_server: &Res<'_, AssetServer>,
    object_registry: &mut ObjectRegistry,
) {
    for (id, definition) in object_defs.iter() {
        let asset_path = asset_server.get_path(id).unwrap();
        let folder_path = asset_path.path().parent().unwrap();
        let sprite_json_path = folder_path.join(definition.actor.aseprite_path.clone());

        let sheet_handle = load_spritesheet(
            &mut cmds,
            &asset_server,
            sprite_json_path,
            bevy::sprite::Anchor::Center,
        );

        match definition.object_type {
            ObjectType::Weapon { damage, form } => {
                insert_weapon_into_registry(
                    object_registry,
                    definition,
                    damage,
                    form,
                    sheet_handle,
                );
            }
            ObjectType::Trinket {} => todo!("trinket objects not implmented"),
            ObjectType::Armor {} => todo!("armor objects not implmented"),
            ObjectType::Food {} => todo!("food objects not implmented"),
        }
    }
}

fn insert_weapon_into_registry(
    object_registry: &mut ObjectRegistry,
    definition: &ObjectDefinition,
    damage: AttackDamage,
    form: WeaponForm,
    sheet_handle: Handle<Spritesheet>,
) {
    object_registry.weapons.insert(
        definition.actor.identifier.clone(),
        WeaponBundle {
            name: Name::new(definition.actor.name.clone()),
            identifier: definition.actor.identifier.clone(),
            holder: WeaponHolder::default(),
            damage,
            weapon_type: form,
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
            stats: EquipmentStats::from_attrs(definition.actor.stats, None),
        },
    );
}

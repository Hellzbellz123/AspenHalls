use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::{Highlight, ListenerInput, On, PickingInteraction},
    PickableBundle,
};

use crate::{
    bundles::CharacterColliderBundle,
    consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX},
    game::{actors::components::CharacterColliderTag, interface::StartMenu},
    game::{
        actors::{
            combat::components::WeaponSlots,
            player::movement::{camera_movement_system, update_player_velocity},
        },
        input::action_maps::PlayerBundle,
    },
    loading::{custom_assets::actor_definitions::CharacterDefinition, registry::RegistryIdentifier},
    AppState,
};

use bevy_rapier2d::prelude::CollisionGroups;

use self::{
    actions::{equip_closest_weapon, spawn_custom_on_button},
    actions::{player_attack_sender, ShootEvent},
};

use super::combat::components::WeaponSocket;

/// new type for animations
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// player actions
pub mod actions;
/// player movement functions
mod movement;

/// handles player events, and fn
pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_event::<SelectThisHeroForPlayer>()
            .add_systems(
                Update,
                (
                    update_player_velocity,
                    camera_movement_system,
                    spawn_custom_on_button,
                    player_attack_sender,
                    equip_closest_weapon,
                )
                    .run_if(state_exists_and_equals(AppState::PlayingGame)),
            )
            .add_systems(
                OnEnter(AppState::PlayingGame),
                build_player_from_selected_hero,
            )
            .add_systems(
                Update,
                select_wanted_hero.run_if(
                    state_exists_and_equals(AppState::StartMenu)
                        .and_then(not(any_with_component::<SelectedHero>())),
                ),
            );
    }
}

#[derive(Debug, Component)]
pub struct SelectedHero;

#[derive(Event)]
pub struct SelectThisHeroForPlayer(Entity, f32);

impl From<ListenerInput<Pointer<Down>>> for SelectThisHeroForPlayer {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        SelectThisHeroForPlayer(event.target, event.hit.depth)
    }
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
fn select_wanted_hero(
    start_menu_query: Query<&Style, (With<Node>, With<StartMenu>)>,
    mut cmds: Commands,
    mut select_events: EventReader<SelectThisHeroForPlayer>,
    // mut pickable_query: Query<Entity, With<On<Pointer<Down>>>>
) {
    let start_menu_style = start_menu_query.single();
    if start_menu_style.display != Display::None {
        return;
    }

    for event in select_events.read() {
        debug!("selecting hero");
        cmds.entity(event.0).insert(SelectedHero).remove::<(
            PickableBundle,
            Highlight<StandardMaterial>,
            On<Pointer<Down>>,
        )>();
        cmds.insert_resource(NextState(Some(AppState::PlayingGame)));
    }
}

/// spawns player with no weapons
pub fn build_player_from_selected_hero(
    mut commands: Commands,
    player_selected_hero: Query<(Entity, &RegistryIdentifier), With<SelectedHero>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    let (selected_hero, player_registry_identifier) = player_selected_hero.single();

    let (_, char_def) = char_assets
        .iter()
        .find(|(_, asset)| asset.actor.identifier == *player_registry_identifier)
        .expect("Spawned characters asset definition did not exist");

    commands
        .entity(selected_hero)
        .remove::<(SelectedHero, PickingInteraction)>();

    info!("Finalizing player before game start");
    commands
        .entity(selected_hero)
        .insert((
            PlayerBundle::default(),
            WeaponSocket {
                drawn_slot: Some(WeaponSlots::Slot1),
                weapon_slots: hero_weapon_slots(),
            },
        ))
        .with_children(|child| {
            child.spawn((CharacterColliderBundle {
                tag: CharacterColliderTag,
                name: Name::new("PlayerCollider"),
                transform_bundle: TransformBundle {
                    local: (Transform {
                        // transform relative to parent
                        translation: (Vec3 {
                            x: 0.,
                            y: 0.,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        }),
                        ..default()
                    }),
                    ..default()
                },
                collider: actor_collider(char_def.actor.pixel_size),
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::ACTOR,
                    AspenCollisionLayer::EVERYTHING,
                ),
            },));
        });
}

/// creates empty weapon slots
pub fn hero_weapon_slots() -> HashMap<WeaponSlots, Option<Entity>> {
    let mut weapon_slots = HashMap::new();
    weapon_slots.insert(WeaponSlots::Slot1, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot2, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot3, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot4, None::<Entity>);

    weapon_slots
}

/// creates empty weapon slots
pub fn enemy_weapon_slots() -> HashMap<WeaponSlots, Option<Entity>> {
    let mut weapon_slots = HashMap::new();
    weapon_slots.insert(WeaponSlots::Slot1, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot2, None::<Entity>);

    weapon_slots
}

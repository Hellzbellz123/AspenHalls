use bevy::{input::mouse::AccumulatedMouseScroll, platform::collections::HashMap, prelude::*};

use crate::{
    bundles::{AspenColliderBundle, NeedsCollider},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX},
    game::{
        characters::components::WeaponSlot, components::ActorColliderType, items::weapons::components::WeaponCarrier,
    },
    loading::{
        custom_assets::actor_definitions::CharacterDefinition, registry::RegistryIdentifier,
        splashscreen::MainCamera,
    },
    playing_game,
    utilities::EntityCreator,
    GameStage, GeneralSettings,
};

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
        app.add_event::<SelectThisHeroForPlayer>()
            .add_systems(
                Update,
                (
                    select_wanted_hero.run_if(in_state(GameStage::SelectCharacter)),
                    (
                        movement::camera_movement_system,
                        movement::update_player_velocity,
                        actions::spawn_custom,
                        actions::player_attack,
                        actions::equip_closest_weapon,
                        actions::zoom_control,
                        actions::change_weapon,
                        actions::aim_weapon,
                    )
                        .run_if(playing_game()),
                ),
            )
            .add_systems(
                OnExit(GameStage::SelectCharacter),
                build_player_from_selected_hero,
            );
    }
}

/// hero player has selected for play
#[derive(Debug, Component)]
pub struct PlayerSelectedHero;

/// event sent when player selects available hero too play
#[derive(Event)]
pub struct SelectThisHeroForPlayer(pub Entity);

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
fn select_wanted_hero(
    mut cmds: Commands,
    mut select_events: EventReader<SelectThisHeroForPlayer>,
    // TODO: this is super annoying to get.
    camera_projection: Single<&mut Projection, With<MainCamera>>,
    settings: Res<GeneralSettings>,
) {
    if !select_events.is_empty() {
        trace!("resetting zoom");
        match *camera_projection.into_inner() {
            Projection::Orthographic(ref mut orthographic_projection) => {
                orthographic_projection.scale = settings.camera_zoom
            },
            _ => {}
        }
    }

    for SelectThisHeroForPlayer(hero, ..) in select_events.read() {     
        trace!("selecting hero");
        cmds.entity(*hero)
            .insert(PlayerSelectedHero)
            .with_children(|child| {
                child.spawn((
                    EntityCreator(*hero),
                    AspenColliderBundle {
                        tag: ActorColliderType::Character,
                        name: Name::new("PlayerCollider"),
                        transform: Transform {
                            // transform relative to parent
                            translation: (Vec3 {
                                x: 0.,
                                y: 0.,
                                z: ACTOR_PHYSICS_Z_INDEX,
                            }),
                            ..default()
                        },
                        collider: NeedsCollider::Aabb,
                        collision_groups: AspenCollisionLayer::dynamic_actor(),
                    },
                ));
            });

        cmds.insert_resource(NextState::Pending(GameStage::PlayingGame));
    }
}


// TODO: add colliders/weaponcarriers too players when spawned?
/// spawns player with no weapons
pub fn build_player_from_selected_hero(
    mut commands: Commands,
    player_selected_hero: Query<(Entity, &RegistryIdentifier), With<PlayerSelectedHero>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    let Ok((selected_hero, player_registry_identifier)) = player_selected_hero.single() else {
        warn!("no player entity available too build off");
        return;
    };

    let (_, char_def) = char_assets
        .iter()
        .find(|(_, asset)| asset.actor.identifier == *player_registry_identifier)
        .expect("Spawned characters asset definition did not exist");

    info!("Finalizing player before game start");
    commands
        .entity(selected_hero)
        .insert((WeaponCarrier {
            drawn_slot: None,
            weapon_slots: hero_weapon_slots(),
        },))
        .with_children(|child| {
            child.spawn((
                EntityCreator(selected_hero),
                AspenColliderBundle {
                    tag: ActorColliderType::Character,
                    name: Name::new("PlayerCollider"),
                    transform: Transform {
                        // transform relative to parent
                        translation: (Vec3 {
                            x: 0.,
                            y: 0.,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        }),
                        ..default()
                    },
                    collider: NeedsCollider::Aabb,
                    collision_groups: AspenCollisionLayer::dynamic_actor(),
                },
            ));
        });
}

/// creates empty weapon slots
pub fn hero_weapon_slots() -> HashMap<WeaponSlot, Option<Entity>> {
    let mut weapon_slots = HashMap::new();
    weapon_slots.insert(WeaponSlot::Slot1, None::<Entity>);
    weapon_slots.insert(WeaponSlot::Slot2, None::<Entity>);
    weapon_slots.insert(WeaponSlot::Slot3, None::<Entity>);
    weapon_slots.insert(WeaponSlot::Slot4, None::<Entity>);

    weapon_slots
}

// /// creates empty weapon slots
// pub fn enemy_weapon_slots() -> HashMap<WeaponSlot, Option<Entity>> {
//     let mut weapon_slots = HashMap::new();
//     weapon_slots.insert(WeaponSlot::Slot1, None::<Entity>);
//     weapon_slots.insert(WeaponSlot::Slot2, None::<Entity>);

//     weapon_slots
// }

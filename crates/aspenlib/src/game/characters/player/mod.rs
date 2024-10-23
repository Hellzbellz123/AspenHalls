use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::{ListenerInput, On, PickingInteraction},
    PickableBundle,
};

use crate::{
    bundles::ActorColliderBundle, consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX}, game::{
        characters::components::WeaponSlot, components::ActorColliderType,
        interface::start_menu::StartMenuTag, items::weapons::components::WeaponCarrier,
    },
    loading::{
        custom_assets::actor_definitions::CharacterDefinition, registry::RegistryIdentifier,
        splashscreen::MainCamera,
    },
    AppState, GeneralSettings,
    utilities::EntityCreator,
};

use bevy_rapier2d::prelude::CollisionGroups;

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
                ((
                    movement::update_player_velocity,
                    movement::camera_movement_system,
                    actions::spawn_custom,
                    actions::player_attack,
                    actions::equip_closest_weapon,
                    actions::zoom_control,
                    actions::change_weapon,
                    actions::aim_weapon,
                )
                    .run_if(in_state(AppState::PlayingGame)),),
            )
            .add_systems(OnExit(AppState::StartMenu), build_player_from_selected_hero)
            .add_systems(
                Update,
                select_wanted_hero.run_if(
                    in_state(AppState::StartMenu).and_then(on_event::<SelectThisHeroForPlayer>()),
                ),
            );
    }
}

/// hero player has selected for play
#[derive(Debug, Component)]
pub struct PlayerSelectedHero;

/// event sent when player selects available hero too play
#[derive(Event)]
pub struct SelectThisHeroForPlayer(Entity, ());

impl From<ListenerInput<Pointer<Down>>> for SelectThisHeroForPlayer {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        Self(event.target, ()) //event.hit.depth)
    }
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
fn select_wanted_hero(
    start_menu_query: Query<&Style, (With<Node>, With<StartMenuTag>)>,
    mut cmds: Commands,
    mut select_events: EventReader<SelectThisHeroForPlayer>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    settings: Res<GeneralSettings>,
) {
    let start_menu_style = start_menu_query.single();
    let mut camera_projection = camera_query.single_mut();

    if start_menu_style.display != Display::None {
        return;
    }

    for SelectThisHeroForPlayer(hero, ..) in select_events.read() {
        trace!("resetting zoom");
        camera_projection.scale = settings.camera_zoom;

        trace!("selecting hero");
        cmds.entity(*hero)
            .insert(PlayerSelectedHero)
            .remove::<On<Pointer<Down>>>()
            .remove::<PickableBundle>();

        cmds.insert_resource(NextState(Some(AppState::PlayingGame)));
    }
}

/// spawns player with no weapons
pub fn build_player_from_selected_hero(
    mut commands: Commands,
    player_selected_hero: Query<(Entity, &RegistryIdentifier), With<PlayerSelectedHero>>,
    char_assets: Res<Assets<CharacterDefinition>>,
) {
    let (selected_hero, player_registry_identifier) = player_selected_hero.single();

    let (_, char_def) = char_assets
        .iter()
        .find(|(_, asset)| asset.actor.identifier == *player_registry_identifier)
        .expect("Spawned characters asset definition did not exist");

    commands
        .entity(selected_hero)
        .remove::<PickingInteraction>();

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
                ActorColliderBundle {
                    tag: ActorColliderType::Character,
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

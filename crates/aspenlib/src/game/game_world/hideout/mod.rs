use std::time::Duration;

use bevy::{
    ecs::{schedule::Condition, system::Res},
    log::{debug, error, info},
    math::Vec2,
    prelude::{
        any_with_component, on_event, run_once, state_exists_and_equals, Commands,
        DespawnRecursiveExt, Entity, GlobalTransform, IntoSystemConfigs, OnEnter,
        OrthographicProjection, Plugin, Query, Transform, Update, With, Without, OnExit,
    },
    time::common_conditions::on_timer,
};
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::{On, PickableBundle},
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    consts::{ACTOR_Z_INDEX, HIGHLIGHT_TINT},
    game::{
        characters::{
            components::CharacterMoveState,
            player::{PlayerSelectedHero, SelectThisHeroForPlayer},
        },
        game_world::{
            components::HeroSpot,
            dungeonator_v2::GeneratorState,
            hideout::systems::{
                spawn_hideout,
                // enter_the_dungeon,
                teleporter_collisions,
            },
        },
        items::weapons::components::AttackDamage,
    },
    loading::{registry::ActorRegistry, splashscreen::MainCamera},
    AppState,
};

use self::systems::MapContainerTag;

/// hideout systems
pub mod systems;

/// plugin for safe house
pub struct HideOutPlugin;

// TODO: spawn different hideout when player beats boss
// spawn TestingHalls as first level if debug ONLY

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        // app.add_plugins(bevy_tiling_background::TilingBackgroundPlugin::<ScaledBackgroundMaterial>::default());
        app.add_systems(OnEnter(AppState::StartMenu), spawn_hideout)
            .add_systems(
                Update,
                (select_hero_focus, populate_selectable_heroes).run_if(
                    state_exists_and_equals(AppState::StartMenu)
                        .and_then(on_timer(Duration::from_secs_f32(0.2)))
                        .and_then(any_with_component::<HeroSpot>())
                        .and_then(run_once()),
                ),
            )
            .add_systems(
                OnExit(GeneratorState::SelectPresets),
                cleanup_start_world,
            )
            .add_systems(
                Update,
                (
                    // TODO: fix scheduling
                    teleporter_collisions.run_if(on_event::<CollisionEvent>()),
                ),
            );
    }
}

/// spawns selectable heroes at each available `HeroSpot`
fn populate_selectable_heroes(
    mut commands: Commands,
    registry: Res<ActorRegistry>,
    hero_spots: Query<&GlobalTransform, With<HeroSpot>>,
) {
    let mut hero_spots = hero_spots.iter();
    if registry.characters.heroes.is_empty() {
        error!("no heroes too pick from");
    }
    for thing in registry.characters.heroes.values() {
        let Some(spot) = hero_spots.next() else {
            error!("no more hero spots");
            return;
        };
        let mut bundle = thing.clone();
        bundle.aseprite.sprite_bundle.transform.translation =
            spot.translation().truncate().extend(ACTOR_Z_INDEX);
        error!("placing at hero spot");
        commands.spawn((
            bundle,
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<SelectThisHeroForPlayer>(),
            HIGHLIGHT_TINT,
        ));
    }
}

// TODO: re apply camera scale AFTER player is selected
/// modifies main camera too focus all the available hero spots
fn select_hero_focus(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    hero_spots: Query<&GlobalTransform, With<HeroSpot>>,
) {
    let hero_spots_amnt = hero_spots.iter().len() as f32;
    let sum_hero_spots: Vec2 = hero_spots.iter().map(|f| f.translation().truncate()).sum();
    let avg = sum_hero_spots / hero_spots_amnt;

    info!("hero spots amount: {}", hero_spots_amnt);
    info!("hero spots sum: {}", sum_hero_spots);
    info!("calculated avg: {}", avg);

    let (mut camera_pos, mut camera_proj) = camera_query.single_mut();
    camera_proj.scale = 6.0;
    camera_pos.translation = avg.extend(camera_pos.translation.z);
}

// TODO: remove this infavor of DespawnWhenStateIs(Option<S: States/State>)
/// despawn all entities that should be cleaned up on restart
fn cleanup_start_world(
    mut commands: Commands,
    characters_not_player: Query<Entity, (With<CharacterMoveState>, Without<PlayerSelectedHero>)>,
    home_world_container: Query<Entity, With<MapContainerTag>>,
    weapons: Query<Entity, With<AttackDamage>>,
) {
    if home_world_container.is_empty() {
        debug!("no home world?");
        return;
    }
    commands
        .entity(home_world_container.single())
        .despawn_recursive();
    for ent in &weapons {
        commands.entity(ent).despawn_recursive();
    }
    for ent in &characters_not_player {
        commands.entity(ent).despawn_recursive();
    }
}

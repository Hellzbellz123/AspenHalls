use bevy::{
    ecs::{schedule::Condition, system::Res},
    log::{error, info},
    math::Vec2,
    prelude::{
        in_state, on_event, Commands, DespawnRecursiveExt, Entity, EventReader, GlobalTransform,
        IntoSystemConfigs, OnExit, OrthographicProjection, Plugin, Query, Transform, Update, With,
        Without,
    },
};
use bevy_ecs_ldtk::prelude::{LevelEvent, LevelSet};
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::{On, PickableBundle},
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        characters::{
            components::CharacterMoveState,
            player::{PlayerSelectedHero, SelectThisHeroForPlayer},
        },
        game_world::{
            components::HeroSpot,
            dungeonator_v2::GeneratorState,
            hideout::systems::{spawn_world_container, teleporter_collisions},
        },
        items::weapons::components::AttackDamage,
    },
    loading::{registry::ActorRegistry, splashscreen::MainCamera},
    AppState,
};

use self::systems::HideoutTag;

/// hideout systems
pub mod systems;

/// plugin for safe house
pub struct HideOutPlugin;

// TODO: spawn different hideout when player beats boss
// spawn TestingHalls as first level if debug ONLY

impl Plugin for HideOutPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app.add_systems(OnExit(AppState::Loading), spawn_world_container);
        app.add_systems(OnExit(GeneratorState::NoDungeon), despawn_hideout);
        app.add_systems(
            Update,
            (
                // TODO: fix scheduling
                teleporter_collisions.run_if(on_event::<CollisionEvent>()),
                create_playable_heroes
                    .run_if(in_state(AppState::StartMenu).and_then(on_event::<LevelEvent>())),
            ),
        );
    }
}

/// spawns selectable heroes at each available `HeroSpot`
fn create_playable_heroes(
    mut level_spawn_events: EventReader<LevelEvent>,
    mut commands: Commands,
    registry: Res<ActorRegistry>,
    hero_spots: Query<&GlobalTransform, With<HeroSpot>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
) {
    for event in level_spawn_events.read() {
        if let LevelEvent::Transformed(_iid) = event {
            let hero_spots: Vec<&GlobalTransform> = hero_spots.iter().collect();
            if registry.characters.heroes.is_empty() {
                error!("no heroes too pick from");
            }
            if hero_spots.is_empty() {
                error!("no hero spots too put heroes");
            }

            info!("preparing heroes and focusing camera");
            populate_hero_spots(&registry, &hero_spots, &mut commands);
            adjust_camera_focus(hero_spots, &mut camera_query);
        }
    }
}

// TODO: re apply camera scale AFTER player is selected
/// modifies main camera too focus all the available hero spots
fn adjust_camera_focus(
    hero_spots: Vec<&GlobalTransform>,
    camera_query: &mut Query<
        '_,
        '_,
        (&mut Transform, &mut OrthographicProjection),
        With<MainCamera>,
    >,
) {
    let hero_spots_amnt = hero_spots.len() as f32;
    let sum_hero_spots: Vec2 = hero_spots.iter().map(|f| f.translation().truncate()).sum();
    let avg = sum_hero_spots / hero_spots_amnt;

    info!("focusing camera on all heroes");
    let (mut camera_pos, mut camera_proj) = camera_query.single_mut();
    camera_proj.scale = 6.0;
    camera_pos.translation = avg.extend(camera_pos.translation.z);
}

/// fills hero slots with selectable heroes
fn populate_hero_spots(
    registry: &Res<ActorRegistry>,
    hero_spots: &[&GlobalTransform],
    commands: &mut Commands,
) {
    let mut hero_spots = hero_spots.iter();

    info!("placing heroes");
    // TODO: swap this around for better expandability?
    registry.characters.heroes.values().for_each(|thing| {
        let Some(spot) = hero_spots.next() else {
            error!("no more hero spots");
            return;
        };
        let mut bundle = thing.clone();
        bundle.aseprite.sprite_bundle.transform.translation =
            spot.translation().truncate().extend(ACTOR_Z_INDEX);
        commands.spawn((
            bundle,
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<SelectThisHeroForPlayer>(),
        ));
    });
}

// TODO: find all uses of cmds.spawn(()) and add cleanup component
// cleanup component should be a system that querys for a specific DespawnComponent and despawns all entitys in the query
// DespawnWhenStateIs(Option<S: States/State>)
/// despawn all entities that should be cleaned up on restart
fn despawn_hideout(
    mut commands: Commands,
    characters_not_player: Query<Entity, (With<CharacterMoveState>, Without<PlayerSelectedHero>)>,
    weapons: Query<Entity, With<AttackDamage>>,
    hideout: Query<(Entity, &LevelSet), With<HideoutTag>>,
) {
    let (hideout, _levelset) = hideout.single();
    commands.entity(hideout).despawn_recursive();

    for ent in &weapons {
        commands.entity(ent).despawn_recursive();
    }
    for ent in &characters_not_player {
        commands.entity(ent).despawn_recursive();
    }
}

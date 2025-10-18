use avian2d::prelude::CollisionStarted;
use bevy::{
    ecs::{
        schedule::{Condition, IntoScheduleConfigs},
        system::{Res, Single},
    },
    log::{error, info},
    math::Vec2,
    picking::{Pickable, events::Pressed},
    prelude::{
        Assets, ChildOf, Commands, Entity, EventReader, EventWriter, GlobalTransform, OnEnter,
        OrthographicProjection, Plugin, Pointer, Query, Reflect, Transform, Trigger, Update, With,
        Without, in_state, on_event, warn,
    },
    render::camera::Projection,
};
use bevy_ecs_ldtk::{
    LevelIid, LevelSelection,
    prelude::{LdtkExternalLevel, LevelEvent, LevelSet},
};

use crate::{
    AppStage,
    bundles::Aspen2dPhysicsBundle,
    consts::ACTOR_Z_INDEX,
    game::{
        characters::{
            components::CharacterMoveState,
            player::{PlayerSelectedHero, SelectThisHeroForPlayer},
        },
        game_world::{
            components::HeroLocation,
            dungeonator_v2::GeneratorState,
            hideout::systems::{spawn_hideout, teleporter_collisions},
        },
        items::weapons::components::AttackDamage,
    },
    loading::{
        registry::{ActorRegistry, RegistryIdentifier},
        splashscreen::MainCamera,
    },
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
        app.add_systems(OnEnter(AppStage::Starting), spawn_hideout);
        app.add_systems(OnEnter(GeneratorState::LayoutDungeon), despawn_hideout);
        app.add_systems(
            Update,
            (
                // TODO: fix scheduling
                teleporter_collisions,
                create_playable_heroes
                    .run_if(in_state(AppStage::Running).and(on_event::<LevelEvent>)),
            ),
        );
    }
}

/// spawns selectable heroes at each available `HeroSpot`
fn create_playable_heroes(
    registry: Res<ActorRegistry>,
    selected_level: Res<LevelSelection>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
    hero_spots: Query<&GlobalTransform, With<HeroLocation>>,
    mut level_spawn_events: EventReader<LevelEvent>,
    mut commands: Commands,
    mut already_spawned_hero: Query<
        (&RegistryIdentifier, &mut Transform),
        (With<PlayerSelectedHero>, Without<MainCamera>),
    >,
    mut camera_query: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let level = match selected_level.into_inner() {
        LevelSelection::Identifier(a) => {
            let level_asset = level_assets
                .iter()
                .find(|f| f.1.data().identifier() == a)
                .expect("msg")
                .1
                .data();
            let level_iid = level_asset.iid();
            LevelIid::new(level_iid)
        }
        LevelSelection::Iid(level_iid) => level_iid.clone(),
        LevelSelection::Uid(_) => panic!("uid grabbing for levels is unhandled as of yet"),
        LevelSelection::Indices(_) => {
            panic!("unable too handle multiple level spawning hero spawners as of yet")
        }
    };

    for event in level_spawn_events.read() {
        let existing_hero = already_spawned_hero.single_mut();
        if let LevelEvent::Transformed(iid) = event {
            if iid != &level {
                continue;
            }
            let hero_spots: Vec<&GlobalTransform> = hero_spots.iter().collect();
            if registry.characters.heroes.is_empty() {
                error!("no heroes too pick from");
            }
            if hero_spots.is_empty() {
                error!("no hero spots too put heroes");
            }

            info!("preparing heroes and focusing camera");
            let hero_spots_iter = hero_spots.iter();

            info!("placing heroes");
            populate_hero_spots(&registry, existing_hero, hero_spots_iter, &mut commands);

            let hero_spots_amnt = hero_spots.len() as f32;
            let sum_hero_spots: Vec2 = hero_spots.iter().map(|f| f.translation().truncate()).sum();
            let avg = sum_hero_spots / hero_spots_amnt;

            info!("focusing camera on all heroes");

            // TODO: is extending with z right?
            camera_query.0.translation = avg.extend(camera_query.0.translation.z);

            // TODO: rethink the camera scale system
            match *camera_query.1 {
                Projection::Orthographic(ref mut orthographic_projection) => {
                    orthographic_projection.scale = 0.6
                }
                _ => return,
            }
        }
    }
}

/// fills hero slots with selectable heroes
fn populate_hero_spots(
    registry: &Res<ActorRegistry>,
    existing_hero: Result<
        (&RegistryIdentifier, bevy::prelude::Mut<Transform>),
        bevy::ecs::query::QuerySingleError,
    >,
    mut hero_spots_iter: std::slice::Iter<&GlobalTransform>,
    commands: &mut Commands,
) {
    // TODO: swap this around for better expandability?
    registry
        .characters
        .heroes
        .values()
        .filter(|f| {
            if let Ok((a, _)) = existing_hero {
                *a != f.identifier
            } else {
                true
            }
        })
        .for_each(|bundle| {
            let Some(spot) = hero_spots_iter.next() else {
                error!("no more hero spots");
                return;
            };

            use std::fmt::Debug;

            // An observer listener that changes the target entity's color.
            fn send_select_player_event_on<E: Debug + Clone + Reflect>()
            -> impl Fn(Trigger<E>, EventWriter<SelectThisHeroForPlayer>, Query<&PlayerSelectedHero>)
            {
                move |trigger,
                      mut ew: EventWriter<SelectThisHeroForPlayer>,
                      other_heroes: Query<&PlayerSelectedHero>| {
                    if other_heroes.is_empty() {
                        warn!("selectable player was clicked");
                        ew.write(SelectThisHeroForPlayer(trigger.target()));
                    }
                }
            }

            commands
                .spawn((
                    bundle.clone(),
                    Pickable::default(),
                    Aspen2dPhysicsBundle::default_character(),
                    Transform::from_translation(
                        spot.translation().truncate().extend(ACTOR_Z_INDEX),
                    ),
                ))
                .observe(send_select_player_event_on::<Pointer<Pressed>>());
        });

    if existing_hero.is_ok() {
        let (_id, mut position) = existing_hero.unwrap();
        let new_spot = hero_spots_iter.next();
        if let Some(new_spot) = new_spot {
            warn!("moving existing hero too unoccupied hero spot");
            position.translation = new_spot.translation().truncate().extend(ACTOR_Z_INDEX);
        } else {
            warn!("no empty hero spot was found");
        }
    }
}

// TODO: find all uses of cmds.spawn(()) and add cleanup component
// cleanup component should be a system that querys for a specific DespawnComponent and despawns all entitys in the query
// DespawnWhenStateIs(Option<S: States/State>)
/// despawn all entities that should be cleaned up on restart
fn despawn_hideout(
    mut commands: Commands,
    characters_not_player: Query<Entity, (With<CharacterMoveState>, Without<PlayerSelectedHero>)>,
    weapons: Query<Entity, (With<AttackDamage>, Without<ChildOf>)>,
    hideout: Query<(Entity, &LevelSet), With<HideoutTag>>,
) {
    for (hideout, levelset) in &hideout {
        commands.entity(hideout).despawn();
    }

    for ent in &weapons {
        commands.entity(ent).despawn();
    }
    for ent in &characters_not_player {
        commands.entity(ent).despawn();
    }
}

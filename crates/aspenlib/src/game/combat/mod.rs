use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_rapier2d::{
    geometry::SolverFlags,
    pipeline::{BevyPhysicsHooks, PairFilterContextView},
};

use crate::{
    game::{
        attributes_stats::{CharacterStats, DamageQueue},
        characters::player::PlayerSelectedHero,
        combat::unarmed::EventAttackUnarmed,
        game_world::components::{ActorTeleportEvent, TpTriggerEffect},
        items::weapons::{
            components::{WeaponDescriptor, WeaponHolder},
            EventAttackWeapon,
        },
    },
    AppState,
};

/// handles attacks from characters without weapons
pub mod unarmed;

/// game combat functionality
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(unarmed::UnArmedPlugin);

        app.add_event::<EventRequestAttack>()
            .insert_resource(CurrentRunInformation::default())
            .insert_resource(PlayerSaveInformation::default());

        app.add_systems(
            PreUpdate,
            apply_damage_system.run_if(in_state(AppState::PlayingGame)),
        );

        app.add_systems(
            Update,
            (
                delegate_attack_events.run_if(on_event::<EventRequestAttack>()),
                handle_death_system,
            )
                .run_if(in_state(AppState::PlayingGame)),
        );
    }
}

// TODO: have damaged characters use particle effect or red tint when damaged
/// applys
#[allow(clippy::type_complexity)]
fn apply_damage_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut damaged_characters: Query<
        (&mut CharacterStats, Entity, &DamageQueue),
        Changed<DamageQueue>,
    >,
    player_controlled: Query<&PlayerSelectedHero>,
) {
    for (mut character_stats, character, damage_queue) in &mut damaged_characters {
        for damage in damage_queue.iter_queue() {
            if character_stats.get_current_health() <= 0.0 {
                return;
            }
            if player_controlled.get(character).is_ok() {
                game_info.player_physical_damage_taken += damage.physical.0;
            } else {
                game_info.enemy_physical_damage_taken += damage.physical.0;
            }
            character_stats.apply_damage(*damage);
        }
    }
}

/// gathers entitys that have damage and despawns them if have no remaining health
#[allow(clippy::type_complexity)]
fn handle_death_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut cmds: Commands,
    mut damaged_query: Query<
        (
            Entity,
            &mut CharacterStats,
            &mut Transform,
            Option<&PlayerSelectedHero>,
        ),
        Changed<CharacterStats>,
    >,
    mut teleport_event: EventWriter<ActorTeleportEvent>,
) {
    for (ent, mut stats, _transform, player_control) in &mut damaged_query {
        if stats.get_current_health() <= 0.0 {
            if player_control.is_some() {
                info!("player died, moving player");
                // player is entity that died
                stats.set_health(150.0);
                game_info.player_deaths += 1;

                teleport_event.send(ActorTeleportEvent {
                    tp_type: TpTriggerEffect::Event("StartDungeonGen".to_string()),
                    target: Some(ent),
                    sender: Some(ent),
                });
                continue;
            }

            // entity that died is not player
            error!("despawning entity");
            game_info.enemies_deaths += 1;
            cmds.entity(ent).despawn_recursive();
        }
    }
}

/// triggers weapon attacks if weapon weapon exists
fn delegate_attack_events(
    mut attack_events: EventReader<EventRequestAttack>,
    mut weapon_attack_events: EventWriter<EventAttackWeapon>,
    mut unarmed_attack_events: EventWriter<EventAttackUnarmed>,
    weapon_query: Query<(&WeaponDescriptor, &WeaponHolder), With<Parent>>,
) {
    for attack_request in attack_events.read() {
        match attack_request.direction {
            AttackDirection::FromWeapon(weapon_id) => {
                let Ok((_weapon_info, _weapon_holder)) = weapon_query.get(weapon_id) else {
                    warn!("attack event received but weapon is missing important components");
                    continue;
                };

                weapon_attack_events.send(EventAttackWeapon {
                    requester: attack_request.requester,
                    weapon: weapon_id,
                });
            }
            AttackDirection::FromVector(attack_direction) => {
                unarmed_attack_events.send(EventAttackUnarmed {
                    requester: attack_request.requester,
                    direction: attack_direction,
                });
            }
        };
    }
}

/// character wanted attack
#[derive(Debug, Event)]
pub struct EventRequestAttack {
    /// who is using the weapon
    pub requester: Entity,
    /// what weapon is this attacker using
    pub direction: AttackDirection,
}

/// how too get direction this attack request is towards
#[derive(Debug)]
pub enum AttackDirection {
    /// weapon attack direction is collected from weapons rotation
    FromWeapon(Entity),
    /// weapon attack direction is calculated from a target position
    FromVector(Vec2),
}

/// information tracked for current run
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct CurrentRunInformation {
    /// damage dealt by player this run
    pub enemy_physical_damage_taken: f32,
    /// damage dealt too player this run
    pub player_physical_damage_taken: f32,
    /// enemies killed by player this run
    pub enemies_deaths: i32,
    /// times player has died
    pub player_deaths: i32,
    /// amount of damage enemy's have fired that hit player and didn't get counted
    pub enemy_damage_sent: f32,
    /// amount of damage player have fired that hit enemy and didn't get counted
    pub player_damage_sent: f32,
}

//TODO: save this too file, load from file when rebooting game
/// information tracked for player save state
#[derive(Debug, Clone, Copy, Default, Resource)]
pub struct PlayerSaveInformation {
    /// damage player has cause with this save
    pub all_time_damage: f32,
    /// amount of times player has finishes a run
    pub runs_completed: i32,
    /// amount of times play has started a run
    pub runs_started: i32,
    /// amount of money player has earned
    pub player_money: i32,
    /// total amount of player deaths
    pub total_deaths: i32,
    /// total amonut of items player has collected
    pub items_got: i32,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub struct BulletOwnerFilter(pub Entity);

// A custom filter that allows contacts/intersections only between rigid-bodies
// with the same CustomFilterTag component value.
// Note that using collision groups would be a more efficient way of doing
// this, but we use custom filters instead for demonstration purpose.
#[derive(SystemParam)]
pub struct SameUserDataFilter<'w, 's> {
    tags: Query<'w, 's, &'static BulletOwnerFilter>,
}

impl BevyPhysicsHooks for SameUserDataFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        if let Some(a_filter) = self.tags.get(context.collider1()).ok()
            && let Some(b_filter) = self.tags.get(context.collider2()).ok()
        {
            if a_filter.0 == b_filter.0 {
                // this bullet was requested by opposite entitity
                // dont 'hit' it.
                return None;
            }
        }

        Some(SolverFlags::COMPUTE_IMPULSES)
    }
}

pub mod unarmed;

use bevy::prelude::*;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        attributes_stats::{CharacterStats, DamageQueue},
        characters::player::PlayerSelectedHero,
        items::weapons::{
            components::{WeaponDescriptor, WeaponHolder},
            EventAttackWeapon,
        },
    },
    AppState,
};

/// game combat functionality
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EventRequestAttack>()
            .insert_resource(CurrentRunInformation::default())
            .insert_resource(PlayerSaveInformation::default());

        app.add_systems(
            PreUpdate,
            apply_damage_system.run_if(state_exists_and_equals(AppState::PlayingGame)),
        );

        app.add_systems(
            Update,
            (
                delegate_attack_events.run_if(on_event::<EventRequestAttack>()),
                handle_death_system,
            )
                .run_if(state_exists_and_equals(AppState::PlayingGame)),
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
) {
    for (ent, mut stats, mut transform, player_control) in &mut damaged_query {
        if stats.get_current_health() <= 0.0 {
            if player_control.is_some() {
                error!("player died, moving player");
                // player is entity that died
                stats.set_health(150.0);
                *transform = Transform::from_translation(Vec3::new(0.0, 0.0, ACTOR_Z_INDEX));
                game_info.player_deaths += 1;
            } else {
                // entity that died is not player
                error!("despawning entity");
                game_info.enemies_deaths += 1;
                cmds.entity(ent).despawn_recursive();
            }
        }
    }
}

/// triggers weapon attacks if weapon weapon exists
fn delegate_attack_events(
    mut attack_events: EventReader<EventRequestAttack>,
    mut weapon_attack_events: EventWriter<EventAttackWeapon>,
    weapon_query: Query<(&WeaponDescriptor, &WeaponHolder), With<Parent>>,
) {
    for attack_request in attack_events.read() {
        if attack_request.weapon.is_some() {
            let Ok((_weapon_info, _weapon_holder)) =
                weapon_query.get(attack_request.weapon.expect("check ok"))
            else {
                warn!("attack event received but weapon is missing important components");
                continue;
            };
            weapon_attack_events.send(EventAttackWeapon {
                requester: attack_request.requester,
                weapon: attack_request.weapon.unwrap(),
            });
        } else {
            // TODO: implement fist attack
            warn!("no weapon, assuming melee attack");
        }
    }
}

/// character wanted attack
#[derive(Debug, Event)]
pub struct EventRequestAttack {
    /// who is using the weapon
    pub requester: Entity,
    /// what weapon is this attacker using
    pub weapon: Option<Entity>,
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

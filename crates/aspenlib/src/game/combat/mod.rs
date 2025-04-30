use avian2d::prelude::CollisionStarted;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    game::{
        combat::unarmed::EventAttackUnarmed,
        items::weapons::{
            components::{WeaponDescriptor, WeaponHolder},
            EventAttackWeapon,
        },
    },
    utilities::EntityCreator,
    AppStage,
};

pub mod damage;
/// handles attacks from characters without weapons
pub mod unarmed;

/// game combat functionality
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(unarmed::UnArmedPlugin);

        app.add_event::<EventRequestAttack>();

        app.add_systems(
            PreUpdate,
            damage::apply_damage_system.run_if(in_state(AppStage::Running)),
        );

        app.add_systems(
            Update,
            (
                damage::handle_death_system,
                damage::projectile_hits.run_if(on_event::<CollisionStarted>),
                delegate_attack_events.run_if(on_event::<EventRequestAttack>),
            )
                .run_if(in_state(AppStage::Running)),
        );
    }
}

/// triggers weapon attacks if weapon weapon exists
fn delegate_attack_events(
    mut attack_events: EventReader<EventRequestAttack>,
    mut weapon_attack_events: EventWriter<EventAttackWeapon>,
    mut unarmed_attack_events: EventWriter<EventAttackUnarmed>,
    weapon_query: Query<(&WeaponDescriptor, &WeaponHolder), With<ChildOf>>,
) {
    for attack_request in attack_events.read() {
        match attack_request.direction {
            AttackDirection::FromWeapon(weapon_id) => {
                let Ok((_weapon_info, _weapon_holder)) = weapon_query.get(weapon_id) else {
                    warn!("attack event received but weapon is missing important components");
                    continue;
                };

                weapon_attack_events.write(EventAttackWeapon {
                    requester: attack_request.requester,
                    weapon: weapon_id,
                });
            }
            AttackDirection::FromVector(attack_direction) => {
                unarmed_attack_events.write(EventAttackUnarmed {
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

/// A custom filter that ignores contacts if both contact entities share the same '`EntityCreator`'
#[derive(SystemParam)]
pub struct SameUserDataFilter<'w, 's> {
    /// tags for filtering
    tags: Query<'w, 's, &'static EntityCreator>,
}

// TODO: make this work for avian
// impl BevyPhysicsHooks for SameUserDataFilter<'_, '_> {
//     fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
//         if let Some(a_filter) = self.tags.get(context.collider1()).ok()
//             && let Some(b_filter) = self.tags.get(context.collider2()).ok()
//             && a_filter.0 == b_filter.0
//         {
//             // this bullet was requested by opposite entitity
//             // dont 'hit' it.
//             return None;
//         }

//         Some(SolverFlags::COMPUTE_IMPULSES)
//     }
// }

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::game::actors::{
    attributes_stats::{DamageQueue, ProjectileStats},
    components::ActorColliderType,
};

/// detects projectile hits on player, adds hits too Player
pub fn projectile_hits(
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_queue_query: Query<&mut DamageQueue>,
    parented_collider_query: Query<(Entity, &Parent, &ActorColliderType), With<Collider>>,
    projectile_info: Query<&ProjectileStats>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(a, b, flags) = event {
            if flags.contains(CollisionEventFlags::SENSOR) {
                return;
            }

            let hit_actor = {
                let mut character_colliders = parented_collider_query
                    .iter()
                    .filter(|(_, _, at)| at == &&ActorColliderType::Character);
                character_colliders
                    .find(|f| f.0 == *b || f.0 == *a)
                    .map(|f| f.1.get())
            };

            let hitting_projectile = {
                let mut projectile_colliders = parented_collider_query
                    .iter()
                    .filter(|(_, _, at)| at == &&ActorColliderType::Projectile);
                projectile_colliders
                    .find(|f| f.0 == *b || f.0 == *a)
                    .map(|f| f.1.get())
            };

            if let Some(projectile) = hitting_projectile {
                info!("projectile hit detected");
                if let Some(actor) = hit_actor {
                    let Ok(stats) = projectile_info.get(projectile) else {
                        return;
                    };
                    let Ok(mut damage_queue) = damage_queue_query.get_mut(actor) else {
                        return;
                    };

                    damage_queue.push_damage(stats.damage);
                }
                // projectile hit something other than player
                cmds.entity(projectile).despawn_recursive();
            }
        }
    }
}

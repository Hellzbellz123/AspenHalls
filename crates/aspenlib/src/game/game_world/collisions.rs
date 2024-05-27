use bevy::prelude::*;
use bevy_ecs_ldtk::TileEnumTags;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody, Rot, Vect};

use crate::consts::AspenCollisionLayer;

/// tiles that can collide get this
#[derive(Clone, Debug, Bundle, Default)]
pub struct LdtkTileCollider {
    /// name of collider
    pub name: Name,
    /// entity has physics
    pub rigidbody: RigidBody,
    /// collision shape
    pub collision_shape: Collider,
    /// what too collide with
    pub collision_group: CollisionGroups,
}

// TODO:
// maybe make this a system the registers a bundle?
/// checks tile enum tag for collider tag, creates shape for collider, passes too `insert_collider`, tag is then removed from `tile_enum_tags`
#[allow(clippy::too_many_lines)]
pub fn handle_and_removed_collider_tag(
    tag: &str,
    cmds: &mut Commands,
    entity: Entity,
    tag_info: &mut Mut<TileEnumTags>,
) -> bool {
    // 90 degrees radian
    let degrees = std::f32::consts::FRAC_PI_2;

    let tag_was_handled = match tag {
        "CollideUp" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, -12.), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideDown" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideLeft" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideRight" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideCornerLR" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideCornerUR" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideCornerLL" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideCornerUL" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideInnerUL" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(-12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideInnerLL" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(-12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideInnerUR" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideInnerLR" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "DoubleWallVertical" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, 4.0), degrees, Collider::cuboid(16.0, 4.0)),
                (Vec2::new(-12.0, 4.0), degrees, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "DoubleWallHorizontal" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, 4.0), 0.0, Collider::cuboid(16.0, 4.0)),
                (Vec2::new(-12.0, 4.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        "CollideInnerWall" | "CollideOuterWall" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, 14.0), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_tile_collider(cmds, entity, shape, tag);
            true
        }
        _ => false,
    };
    if tag_was_handled {
        tag_info.tags.retain(|f| f != tag);
    }
    tag_was_handled
}

/// inserts collider onto passed entity, collides with everything
fn insert_tile_collider(
    commands: &mut Commands,
    entity: Entity,
    shape: Vec<(Vec2, f32, Collider)>,
    tag: &str,
) {
    commands.entity(entity).insert((LdtkTileCollider {
        name: Name::new(tag.to_owned()),
        rigidbody: RigidBody::Fixed,
        collision_shape: Collider::compound(shape),
        collision_group: CollisionGroups {
            memberships: AspenCollisionLayer::WORLD,
            filters: Group::ALL,
        },
    },));
}

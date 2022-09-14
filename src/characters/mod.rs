use bevy::{prelude::Vec2, sprite::collide_aabb::Collision};

pub mod player;

#[must_use] pub fn collide_2(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;

    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Some(Collision::Left), b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Some(Collision::Right), a_min.x - b_max.x)
        } else {
            (None, 0.0)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Some(Collision::Bottom), b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Some(Collision::Top), a_min.y - b_max.y)
        } else {
            (None, 0.0)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth < x_depth {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => None,
        }
    } else {
        None
    }
}

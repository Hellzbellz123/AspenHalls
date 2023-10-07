use bevy::{math::vec3, prelude::*};

pub trait Vec3Ext {
    fn move_towards(&self, target: &Vec3, max_dist: f32) -> Option<Vec3>;
}

pub trait Vec3ExtMut {
    fn move_towards(&mut self, target: &Vec3, max_dist: f32) -> bool;
}

fn move_towards(curr: &Vec3, target: &Vec3, max_dist: f32) -> Option<Vec3> {
    let to_x = target.x - curr.x;
    let to_y = target.y - curr.y;
    let to_z = target.z - curr.z;

    let sqdist = to_x * to_x + to_y * to_y + to_z * to_z;
    if sqdist == 0. || (max_dist >= 0. && sqdist <= max_dist * max_dist) {
        return None;
    }

    let dist = sqdist.sqrt();

    Some(vec3(
        curr.x + to_x / dist * max_dist,
        curr.y + to_y / dist * max_dist,
        curr.z + to_z / dist * max_dist,
    ))
}

impl Vec3Ext for Transform {
    fn move_towards(&self, target: &Vec3, max_dist: f32) -> Option<Vec3> {
        move_towards(&self.translation, target, max_dist)
    }
}
impl<'a> Vec3ExtMut for Mut<'a, Transform> {
    fn move_towards(&mut self, target: &Vec3, max_dist: f32) -> bool {
        let Some(pos) = move_towards(&self.translation, target, max_dist) else {
            return true;
        };

        self.translation = pos;
        false
    }
}

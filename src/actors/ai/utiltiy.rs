use bevy::prelude::*;

use crate::components::actors::general::Player;

/// find closest entity too (arg1: Entity) With<Player>.
#[allow(dead_code)]
fn find_closest_player(
    player: &Query<&Transform, With<Player>>,
    actor_position: &Transform,
) -> Transform {
    *(player
        .iter()
        .min_by(|a, b| {
            let da = (a.translation - actor_position.translation).length_squared();
            let db = (b.translation - actor_position.translation).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("to players"))
}

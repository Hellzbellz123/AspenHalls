use crate::game::actors::components::Player;
use bevy::prelude::*;

/// find closest entity too (arg1: Entity) With<Player>.
#[allow(dead_code)]
fn find_closest_player(
    player: &Query<&Transform, With<Player>>,
    actor_position: &Transform,
) -> Transform {
    *(player
        .iter()
        .min_by(|a, b| {
            let da = (a.translation.truncate()
                - actor_position.translation.truncate())
            .length_squared();
            let db = (b.translation.truncate()
                - actor_position.translation.truncate())
            .length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("to players"))
}

use bevy::prelude::*;

use crate::prelude::{engine, game::action_maps};

/// find closest entity too controlled actor.
#[allow(dead_code)]
fn find_closest_player(
    player: &Query<&Transform, With<engine::ActionState<action_maps::Gameplay>>>,
    actor_position: &Transform,
) -> Transform {
    *(player
        .iter()
        .min_by(|a, b| {
            let da =
                (a.translation.truncate() - actor_position.translation.truncate()).length_squared();
            let db =
                (b.translation.truncate() - actor_position.translation.truncate()).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("to players"))
}

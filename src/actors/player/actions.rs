use bevy::prelude::*;

use leafwing_input_manager::prelude::ActionState as leafwingActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::{
        general::Player,
        spawners::{EnemyType, SpawnEvent},
    },
};

pub fn spawn_skeleton_button(
    mut eventwriter: EventWriter<SpawnEvent>,
    query_action_state: Query<&leafwingActionState<PlayerBindables>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::DebugF1) {
            debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
            let player_transform = player_query.single().0;
            let _direction: Vec3 = player_transform.translation.normalize_or_zero();

            eventwriter.send(SpawnEvent {
                enemy_to_spawn: EnemyType::Skeleton,
                spawn_position: (player_transform.translation
                    + Vec3 {
                        x: 36.0,
                        y: 36.0,
                        z: 0.0,
                    }),
                spawn_count: 1,
            })
        };
    }
}

use bevy::prelude::*;

use bevy_mouse_tracking_plugin::MousePosWorld;
use leafwing_input_manager::prelude::ActionState as leafwingActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::{
        general::Player,
        spawners::{EnemyType, SpawnEnemyEvent},
    },
    utilities::game::ACTOR_LAYER,
};

pub fn spawn_skeleton_button(
    mut eventwriter: EventWriter<SpawnEnemyEvent>,
    mouse: Res<MousePosWorld>,
    query_action_state: Query<&leafwingActionState<PlayerBindables>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::DebugF1) {
            debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
            let player_transform = player_query.single().0.translation.truncate();
            let direction: Vec2 = (player_transform - Vec2::new(mouse.x, mouse.y))
                .abs()
                .normalize_or_zero();

            eventwriter.send(SpawnEnemyEvent {
                enemy_to_spawn: EnemyType::Skeleton,
                spawn_position: (player_transform + (direction)).extend(ACTOR_LAYER),
                spawn_count: 1,
            })
        };
    }
}

// .with_system(skeleton::utilities::spawn_skeleton_button)

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

        if actions.just_released(PlayerBindables::Heal) {
            info!("Pressed devtest button");

            if !player_query.is_empty() {
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
}

// pub fn on_shoot(
//     mut commands: Commands,
//     timeinfo: Res<TimeInfo>,
//     time: Res<Time>,
//     assets: ResMut<PlayerTextureHandles>,
//     player_query: Query<&Transform, With<Player>>,
//     mut query: Query<(&Transform, &mut AIAttackTimer), With<AIEnemy>>,
// ) {
//     // let rconstraints = RotationConstraints::allow();

//     if !timeinfo.game_paused {
//         if let Ok(player_transform) = player_query.get_single() {
//             for (transform, mut attacking) in query.iter_mut() {
//                 // Only shoot when the cooldown is over
//                 if !attacking.is_attacking
//                     || !attacking.timer.tick(time.delta()).just_finished()
//                 {
//                     continue;
//                 }

//                 let direction: Vec3 = player_transform.translation.normalize_or_zero();

//                 // Make sure that the projectiles spawn outside of the body so that it doesn't collide
//                 let beyond_body_diff = direction * 36.;
//                 let mut new_transform = *transform;
//                 new_transform.translation = transform.translation + beyond_body_diff;

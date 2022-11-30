use crate::{actors::enemies::skeleton::actions::on_shoot, game::GameStage};
use bevy::prelude::{App, Plugin, SystemSet};

pub mod skeleton;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(on_shoot)
                .with_system(update_enemy_graphics),
        )
        .add_system_set(SystemSet::on_enter(GameStage::Playing));
    }
}

use bevy::{
    prelude::{Entity, Query, ResMut, Vec2, With},
    sprite::TextureAtlasSprite,
};
use bevy_rapier2d::prelude::*;

use crate::{
    components::actors::{ai::AIEnemy, animation::FacingDirection, general::ActorState},
    game::TimeInfo,
};

pub fn update_enemy_graphics(
    timeinfo: ResMut<TimeInfo>,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut ActorState,
        &mut TextureAtlasSprite,
        Entity,
        With<AIEnemy>,
    )>,
) {
    if !timeinfo.game_paused {
        enemy_query.for_each_mut(|(velocity, mut enemystate, mut sprite, _ent, _)| {
            if velocity.linvel == Vec2::ZERO {
                enemystate.facing = FacingDirection::Idle;
            } else if velocity.linvel.x > 5.0 {
                sprite.flip_x = false;
                enemystate.facing = FacingDirection::Right;
            } else if velocity.linvel.x < -5.0 {
                sprite.flip_x = true;
                enemystate.facing = FacingDirection::Left;
            } else if velocity.linvel.y < -5.0 {
                enemystate.facing = FacingDirection::Down;
            } else if velocity.linvel.y > 2.0 {
                enemystate.facing = FacingDirection::Up;
            }
        })
    }
}

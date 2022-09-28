use crate::game::GameStage;
use bevy::prelude::{App, Bundle, Commands, Plugin, SystemSet, *};
use rand::prelude::*;

mod skeleton;
mod zombie;

const MAX_ENEMIES: i32 = 20;
const CAN_SPAWN: bool = true;

#[derive(Bundle)]
struct EnemyBundle {
    label: Name,
    #[bundle]
    sprite: SpriteBundle,
}

fn on_enter(mut cmd: Commands) {
    // info!("this only runs when switching to gamestage::playing, setup enemys here")
    let mut rng = rand::thread_rng();
    if CAN_SPAWN {
        for _enemy in 0..MAX_ENEMIES {
            cmd.spawn_bundle(EnemyBundle {
                label: Name::new("SkeletonMelee".to_string()),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(50.0, 100.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: rng.gen_range(0.0..2195.0),
                            y: rng.gen_range(0.0..1460.0),
                            z: 8.0,
                        },
                        ..default()
                    },
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
            });
        }
    }
}

fn on_update() {
    // info!("this runs every frame in gamestage::playing \"sorta\" ");
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(on_enter))
            .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(on_update));
    }
}

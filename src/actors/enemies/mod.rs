use crate::{
    game::GameStage,
    loading::assets::EnemyTextureHandles,
    utilities::game::{PhysicsLayers, PLAYER_SIZE, TILE_SIZE},
};
use bevy::prelude::{App, Plugin, SystemSet, *};
use big_brain::BigBrainPlugin;
use heron::{CollisionShape, PhysicMaterial, RotationConstraints, Velocity};
use rand::prelude::*;

use crate::actors::{
    components::{Aggroable, Enemy},
    enemies::skeleton::SkeletonBundle,
    RigidBodyBundle,
};

use super::animation::AnimationSheet;

pub mod shaman_ai;
pub mod skeleton;

const MAX_ENEMIES: i32 = 10;

fn on_enter(mut commands: Commands, enemyassets: Res<EnemyTextureHandles>) {
    let mut rng = rand::thread_rng();

    commands
        .spawn()
        .insert(Name::new("EnemyContainer")) //this "EntityContainer" should eventually be expanded too choose enemies and spawn them in and too setup hp and ai.
        .insert_bundle(SpatialBundle {
            visibility: Visibility::visible(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .with_children(|parent| {
            for _ in 0..MAX_ENEMIES {
                parent
                    .spawn_bundle(SkeletonBundle {
                        name: Name::new("Skeleton"),
                        actortype: Enemy,
                        actorstate: super::ActorState {
                            speed: 100.0,
                            sprint_available: false,
                            facing: super::animation::FacingDirection::Idle,
                            just_moved: false,
                        },
                        animation_state: crate::actors::animation::AnimState {
                            timer: Timer::from_seconds(0.2, true),
                            current_frames: vec![0, 1, 2, 3, 4],
                            current_frame: 0,
                        },
                        available_animations: AnimationSheet {
                            handle: enemyassets.skele_full_sheet.clone(),
                            idle_animation: [0, 1, 2, 3, 4],
                            down_animation: [5, 6, 7, 8, 9],
                            up_animation: [10, 11, 12, 13, 14],
                            right_animation: [15, 16, 17, 18, 19],
                        },
                        sprite: SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                                ..default()
                            },
                            texture_atlas: enemyassets.skele_full_sheet.clone(),
                            transform: Transform::from_xyz(
                                rng.gen_range(-470.0..520.0),
                                rng.gen_range(2818.0..3805.0),
                                8.0,
                            ),
                            ..default()
                        },
                        rigidbody: RigidBodyBundle {
                            rigidbody: heron::RigidBody::Dynamic,
                            velocity: Velocity::default(),
                            rconstraints: RotationConstraints::lock(),
                            collision_layers: PhysicsLayers::Enemy.layers(),
                            physicsmat: PhysicMaterial {
                                restitution: 0.1,
                                density: 100.0,
                                friction: 0.5,
                            },
                        },
                        aggroable: Aggroable { distance: 5.0 },
                    })
                    .with_children(|skele_parent| {
                        skele_parent
                            .spawn()
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0, 0.0),
                                border_radius: None,
                            })
                            .insert(Transform::from_translation(Vec3::new(0., -24., 0.)));
                    });
            }
        });
    info!("this only runs when switching to gamestage::playing, setup enemys here")
}

fn on_update() {
    // info!("this runs every frame in gamestage::playing \"sorta\" ");
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(on_enter))
        app.add_plugin(BigBrainPlugin)
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(on_update)
                    .with_system(skeleton::utilities::spawn_skeleton_button),
            )
            .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(on_enter));
    }
}

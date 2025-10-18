use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationRepeat, AseAnimation, Aseprite};

use crate::{
    game::characters::components::{CharacterMoveState, CurrentMovement, MoveDirection},
    utilities::vector_to_cardinal_direction,
};

// TODO: redo player animations to be based on where the mouse cursor is pointing, not player velocity
// this will probably look better and makes the player animations look a bit less funky

/// plays animations for all actors with ([`AnimState`], [`AnimationSheet`], [`TextureAtlasSprite`])
pub struct AnimationsPlugin;

/// different gun animations
pub struct GunAnimations;

/// different character animations
pub struct CharacterAnimations;

impl GunAnimations {
    /// gun still movement
    pub const IDLE: &str = "idle";
    // /// gun vibrate with movement anim
    // pub const WIGGLE: &str = "wiggle";
    /// gun fire animation index
    pub const FIRE: &str = "fire";
    /// gun reload animation index
    pub const RELOAD: &str = "reload";
}

impl CharacterAnimations {
    /// character idle animation index
    pub const IDLE: &str = "idle";
    /// character walk down animation index
    pub const WALK_SOUTH: &str = "walk_south";
    /// character walk up animation index
    pub const WALK_NORTH: &str = "walk_north";
    /// character walk horizontal animation index
    pub const WALK_EAST: &str = "walk_east";
}

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventAnimationChange>();
        app.add_systems(
            Update,
            (handle_animation_changes, change_character_animations),
        );
    }
}

/// updates character animation when move status changes
fn change_character_animations(
    mut change_events: EventWriter<EventAnimationChange>,
    mut characters: Query<
        (Entity, &CharacterMoveState, &LinearVelocity),
        Changed<CharacterMoveState>,
    >,
    mut sprite_query: Query<&mut Sprite>,
) {
    for (character, move_state, velocity) in &mut characters {
        let move_status = &move_state.move_status.0;

        // we are ignoring whats on the character because the diagnal directions are not important here
        // use pi4?
        let move_direction = vector_to_cardinal_direction(**velocity);

        match move_status {
            CurrentMovement::None => {
                change_events.write(EventAnimationChange {
                    anim_handle: vec![CharacterAnimations::IDLE],
                    actor: character,
                });
            }
            _ => match move_direction {
                MoveDirection::South => {
                    change_events.write(EventAnimationChange {
                        anim_handle: vec![CharacterAnimations::WALK_SOUTH],
                        actor: character,
                    });
                }
                MoveDirection::North => {
                    change_events.write(EventAnimationChange {
                        anim_handle: vec![CharacterAnimations::WALK_NORTH],
                        actor: character,
                    });
                }
                MoveDirection::East => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;
                    change_events.write(EventAnimationChange {
                        anim_handle: vec![CharacterAnimations::WALK_EAST],
                        actor: character,
                    });
                }
                MoveDirection::West => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = true;

                    change_events.write(EventAnimationChange {
                        anim_handle: vec![CharacterAnimations::WALK_EAST],
                        actor: character,
                    });
                }
                _ => panic!("should not have got this direction from vec_to_pi4"),
            },
        }
    }
}

/// updates actors animations
fn handle_animation_changes(
    mut change_events: EventReader<EventAnimationChange>,
    mut animateable: Query<&mut AseAnimation>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for event in change_events.read() {
        let Ok(mut animator) = animateable.get_mut(event.actor) else {
            error!("animation event requested for entity that did not have required animation components");
            continue;
        };

        if event.anim_handle.len() == 1
            && let Some(tag) = event.anim_handle.first()
        {
            let Some(aseprite_file) = aseprites.get(&animator.aseprite) else {
                warn!("sprite sheet should exist for this actor");
                continue;
            };

            if !aseprite_file.tags.contains_key(&(*tag).to_string()) {
                warn!("animation id does not exist in spritesheet");
                continue;
            }

            animator.animation.tag = Some((*tag).to_string());
        } else if event.anim_handle.len() > 1 {
            animator.animation.clear_queue();
            animator.animation.tag = None;
            animator.animation.repeat = AnimationRepeat::Count(1);
            animator.animation.playing = true;
            event.anim_handle.iter().enumerate().for_each(|(idx, tag)| {
                animator.animation.queue.push_back((
                    (*tag).to_string(),
                    if idx == event.anim_handle.len() - 1 {
                        AnimationRepeat::Loop
                    } else {
                        AnimationRepeat::Count(0)
                    },
                ));
            });
        }
    }

    // idle animators without animation
    for mut animator in &mut animateable {
        if animator.animation.tag.is_some() {
            continue;
        }
        // TODO: should we check if idle exists?
        animator.animation.tag = Some("idle".to_string());
    }
}

/// update actors animation
#[derive(Debug, Event)]
pub struct EventAnimationChange {
    /// animation too set
    pub anim_handle: Vec<&'static str>,
    /// what actor too change animation on
    pub actor: Entity,
}

// TODO: use this for animation system
// pub struct ActorAnimation {
//     tag: String,
//     speed: f32,
//     repeat: AnimationRepeat,
//     queue: VecDeque<ActorAnimation>,
// }

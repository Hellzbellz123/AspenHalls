use bevy::prelude::*;
use bevy_asepritesheet::{
    animator::SpriteAnimator,
    prelude::{AnimHandle, Spritesheet},
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    game::characters::components::{CharacterMoveState, CurrentMovement, MoveDirection},
    utilities::vector_to_pi4,
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
    // pub const IDLE: usize = 0;
    // pub const WIGGLE: usize = 1;
    /// gun fire animation index
    pub const FIRE: usize = 2;
    /// gun reload animation index
    pub const RELOAD: usize = 3;
}

impl CharacterAnimations {
    /// character idle animation index
    pub const IDLE: usize = 0;
    /// character walk down animation index
    pub const WALK_DOWN: usize = 1;
    /// character walk up animation index
    pub const WALK_UP: usize = 2;
    /// character walk horizontal animation index
    pub const WALK_RIGHT: usize = 3;
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
    mut characters: Query<(Entity, &CharacterMoveState, &Velocity), Changed<CharacterMoveState>>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
) {
    for (character, move_state, velocity) in &mut characters {
        let move_status = &move_state.move_status.0;
        let move_direction = vector_to_pi4(velocity.linvel.normalize());
        match move_status {
            CurrentMovement::None => change_events.send(EventAnimationChange {
                anim_handle: AnimHandle::from_index(CharacterAnimations::IDLE),
                actor: character,
            }),
            _ => match move_direction {
                MoveDirection::South => change_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_DOWN),
                    actor: character,
                }),
                MoveDirection::North => change_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_UP),
                    actor: character,
                }),
                MoveDirection::East => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;
                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_RIGHT),
                        actor: character,
                    });
                }
                MoveDirection::West => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = true;

                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_RIGHT),
                        actor: character,
                    });
                }
                MoveDirection::NorthEast => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;

                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_RIGHT),
                        actor: character,
                    });
                },
                MoveDirection::SouthEast => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;

                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_RIGHT),
                        actor: character,
                    });
                },
                MoveDirection::NorthWest => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;

                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_UP),
                        actor: character,
                    });
                },
                MoveDirection::SouthWest => {
                    let mut sprite = sprite_query.get_mut(character).expect("msg");
                    sprite.flip_x = false;

                    change_events.send(EventAnimationChange {
                        anim_handle: AnimHandle::from_index(CharacterAnimations::WALK_DOWN),
                        actor: character,
                    });
                },
            },
        }
    }
}

/// updates actors animations
fn handle_animation_changes(
    mut change_events: EventReader<EventAnimationChange>,
    mut animateable: Query<(&mut SpriteAnimator, &Handle<Spritesheet>)>,
    sprite_sheets: Res<Assets<Spritesheet>>,
) {
    for event in change_events.read() {
        let Ok((mut animator, sheet_handle)) = animateable.get_mut(event.actor) else {
            return;
        };

        let sprite_sheet = sprite_sheets
            .get(sheet_handle)
            .expect("sprite sheet should exist for this actor");

        let anim_time = sprite_sheet
            .get_anim(&event.anim_handle)
            .expect("anim id does not exist")
            .total_time();
        if animator.is_cur_anim(event.anim_handle) && animator.cur_time() < anim_time
            || animator.cur_anim().unwrap_or(AnimHandle::from_index(0)) == event.anim_handle
        {
            continue;
        }
        animator.set_anim(event.anim_handle);
    }
}

/// update actors animation
#[derive(Debug, Event)]
pub struct EventAnimationChange {
    /// animation too set
    pub anim_handle: AnimHandle,
    /// what actor too change animation on
    pub actor: Entity,
}

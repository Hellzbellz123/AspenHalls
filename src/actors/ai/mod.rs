use bevy::prelude::*;
use big_brain::thinker::Actor;

use crate::game::GameStage;

use self::skeleton_ai::StupidAiPlugin;

pub mod skeleton_ai;
pub mod utiltiy;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StupidAiPlugin)
            .add_system(parent_brains_to_container.in_set(OnUpdate(GameStage::PlayingGame)));
    }
}

#[derive(Debug, Component)]
struct BigBrainContainerTag;

/// this feels like a dirty hack but it totally works!!!
/// ALL HAIL THE ENTITY HIERARCHY!!!
fn parent_brains_to_container(
    mut commands: Commands,
    brain_query: Query<Entity, (With<Actor>, Without<Parent>)>,
    brain_container_query: Query<Entity, With<BigBrainContainerTag>>,
) {
    if brain_query.is_empty() {
        return;
    }

    if brain_container_query.is_empty() {
        commands.spawn((BigBrainContainerTag, Name::new("BigBrainContainer")));
        return;
    }

    brain_query.for_each(|thinker| {
        commands
            .entity(brain_container_query.single())
            .push_children(&[thinker]);
    });
}

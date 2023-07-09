use bevy::prelude::*;
use big_brain::thinker::Actor;

use crate::game::GameStage;

use self::stupid_ai::StupidAiPlugin;

/// ai components
pub mod components;
/// stupid ai stuff
pub mod stupid_ai;
/// util functions
pub mod utiltiy;

/// handles different AI classes
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StupidAiPlugin)
            .add_system(parent_brains_to_container.in_set(OnUpdate(GameStage::PlayingGame)));
    }
}

/// entity tag for all big_brain entitys too be parented too
/// clean heirarchy plz
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

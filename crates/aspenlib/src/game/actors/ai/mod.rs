use bevy::prelude::*;
use big_brain::{thinker::Actor, BigBrainPlugin};

use self::stupid_ai::StupidAiPlugin;

/// ai components
pub mod components;
/// stupid ai stuff
pub mod stupid_ai;
/// util functions
pub mod utility;

/// handles different AI classes
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BigBrainPlugin::new(Update), StupidAiPlugin))
            .add_systems(Update, parent_brains_to_container);
    }
}

/// entity tag for all `big_brain` entity's too be parented too
/// clean hierarchy plz
#[derive(Debug, Component)]
struct BigBrainContainerTag;

//TODO: make this system insert `InspectorIgnore` component on each
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

    for entity in &brain_query {
        commands
            .entity(brain_container_query.single())
            .push_children(&[entity]);
    }
}

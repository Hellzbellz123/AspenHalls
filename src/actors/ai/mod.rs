use bevy::prelude::*;

use self::skeleton_ai::SkeletonAiPlugin;

pub mod skeleton_ai;
pub mod utiltiy;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SkeletonAiPlugin);
    }
}

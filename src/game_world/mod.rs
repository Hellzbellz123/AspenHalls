pub struct MapSystem;

impl Plugin for MapSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugin(LdtkPlugin);
    }
}

use bevy::prelude::{App, Plugin};

use self::window_icon::set_window_icon;

pub mod window_icon;

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(RPCPlugin(RPCConfig {
        //     app_id: 425407036495495169,
        //     show_time: true,
        // }))
        // .add_startup_system(update_presence)
        app.add_startup_system(set_window_icon);
    }
}

// fn update_presence(mut state: ResMut<ActivityState>) {
//     state.instance = Some(true);
//     state.details = Some("Playing VanillaCoffee".to_string());
//     state.state = Some("Just a Lone Developer".to_string());
// }

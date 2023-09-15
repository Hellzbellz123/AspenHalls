use bevy::prelude::*;

use self::components::{PauseMenuRoot, StartMenuRoot, UiRoot};
use crate::{game::AppStage, loading::splashscreen::OnlySplashScreen, utilities};

/// common ui components
pub mod components;
/// pause menu systems
pub mod pause_menu;
/// start menu systems
pub mod start_menu;

/// currently active menu
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum RequestedMenu {
    /// no menu spawned
    #[default]
    None,
    /// start menu
    Start,
    /// pause menu
    Pause,
    /// settings menu
    Settings,
}

/// ui plugin
pub struct BevyUiPlugin;

impl Plugin for BevyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<RequestedMenu>().add_systems(
            Update,
            (
                control_menu_state,
                spawn_ui_root.run_if(state_exists_and_equals(AppStage::Loading).and_then(run_once())),
                // start menu systems
                start_menu::build.run_if(state_exists_and_equals(RequestedMenu::Start).and_then(run_once())),
                start_menu::button_system.run_if(in_state(RequestedMenu::Start)),
                utilities::despawn_with::<StartMenuRoot>.run_if(state_exists_and_equals(RequestedMenu::None)),
                // pause menu systems
                pause_menu::build.run_if(state_exists_and_equals(RequestedMenu::Pause).and_then(run_once())),
                pause_menu::button_system.run_if(in_state(RequestedMenu::Pause)),
                utilities::despawn_with::<PauseMenuRoot>.run_if(state_exists_and_equals(RequestedMenu::None))
            ),
        );
    }
}

/// updates menu state based on game stage
fn control_menu_state(mut cmds: Commands, game_state: Res<State<AppStage>>) {
    if game_state.is_changed() {
        match game_state.get() {
            AppStage::StartMenu => cmds.insert_resource(NextState(Some(RequestedMenu::Start))),
            AppStage::PauseMenu => cmds.insert_resource(NextState(Some(RequestedMenu::Pause))),
            AppStage::PlayingGame => cmds.insert_resource(NextState(Some(RequestedMenu::None))),
            _ => {}
        }
    }
}

/// spawns ui root
fn spawn_ui_root(mut cmds: Commands) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceAround,
                min_height: Val::Percent(100.0),
                min_width: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Global(0),
            ..default()
        },
        Name::new("Ui"),
        UiRoot,
    ));
}

/// no interaction button color
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/// button hovered color
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
/// button pressed color
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

//     parent.spawn((
//         TextBundle::from_section(
//             "Scrolling list",
//             TextStyle {
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 font_size: 25.,
//                 color: Color::WHITE,
//             },
//         )
//         .with_style(Style {
//             size: Size::height(Val::Px(25.)),
//             ..default()
//         }),
//         Label,
//     ));
//     // List with hidden overflow
//     parent
//         .spawn(NodeBundle {
//             style: Style {
//                 flex_direction: FlexDirection::Column,
//                 align_self: AlignSelf::Stretch,
//                 size: Size::height(Val::Percent(50.0)),
//                 overflow: Overflow::Hidden,
//                 ..default()
//             },
//             background_color: Color::rgb(0.10, 0.10, 0.10).into(),
//             ..default()
//         })
//         .with_children(|parent| {
//             // Moving panel
//             parent
//                 .spawn((
//                     NodeBundle {
//                         style: Style {
//                             flex_direction: FlexDirection::Column,
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     ScrollingList::default(),
//                     AccessibilityNode(NodeBuilder::new(Role::List)),
//                 ))
//                 .with_children(|parent| {
//                     // List items
//                     for i in 0..30 {
//                         parent.spawn((
//                             TextBundle::from_section(
//                                 format!("Item {i}"),
//                                 TextStyle {
//                                     font: asset_server
//                                         .load("fonts/FiraSans-Bold.ttf"),
//                                     font_size: 20.,
//                                     color: Color::WHITE,
//                                 },
//                             ),
//                             Label,
//                             AccessibilityNode(NodeBuilder::new(Role::ListItem)),
//                         ));
//                     }
//                 });
//         });
// });

// #[derive(Component, Default)]
// struct ScrollingList {
//     position: f32,
// }

// fn mouse_scroll(
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
//     query_node: Query<&Node>,
// ) {
//     for mouse_wheel_event in mouse_wheel_events.iter() {
//         for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
//             let items_height = list_node.size().y;
//             let container_height = query_node.get(parent.get()).unwrap().size().y;

//             let max_scroll = (items_height - container_height).max(0.);

//             let dy = match mouse_wheel_event.unit {
//                 MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
//                 MouseScrollUnit::Pixel => mouse_wheel_event.y,
//             };

//             scrolling_list.position += dy;
//             scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
//             style.position.top = Val::Px(scrolling_list.position);
//         }
//     }
// }

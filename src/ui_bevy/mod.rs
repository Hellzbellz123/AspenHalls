use crate::{
    components::{OnSplashScreen, SplashTimer},
    game::GameStage,
};
use bevy::{asset::LoadState, prelude::*};

//builds menus for vanillacoffee, both ingame and main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            // .add_plugin(menu::settings::SettingsPlugin)
            //     .add_plugin(menu::game::InGameMenuPlugin);
            .add_system_set(
                SystemSet::on_update(GameStage::Menu).with_system(pass_to_play::<OnSplashScreen>),
            );
        // .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(draw_mainmenu))
        // .add_system_set(SystemSet::on_exit(GameStage::Menu).with_system(destroy_menu))
        // .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(pause_game));
    }
}

fn pass_to_play<T: Component>(
    asset_server: ResMut<AssetServer>,

    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameStage>>,
) {
    let img: Handle<Image> = asset_server.load("splash/splashL.png");
    let mut state_pushed;

    let imgloadstate = asset_server.get_load_state(img);

    if imgloadstate == LoadState::Loaded
        && timer.tick(time.delta()).finished()
        && game_state.current() == &GameStage::Menu
    {
        state_pushed = false;

        // info!("splash asset loaded");
        // game_state.set(GameStage::Menu).unwrap(); //TODO:change back too menu when updated too new kayakui version
        info!("pushing playing state too state stack");
        if !state_pushed {
            game_state
                .push(GameStage::Playing)
                .expect("couldnt push state to stack");

            state_pushed = true;

            for entity in to_despawn.iter() {
                info!("despawning entity: {:#?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        } else if state_pushed {
            info!(" do nothing?")
        }
    }
}

// A Settings example
// This example mimics in-game settings with player and sound controls.
// It also adds some types to simulate gamepad and keyboard settings.
// It also shows how to get from the Settings to the game and back.
// Due to the way Bevy handles GameStates (which will soon be rewritten),
// composing menus and games looks a bit convoluted.
pub mod menu {
    use crate::game::GameStage;
    use bevy::{ecs::schedule::ShouldRun, prelude::*};

    impl GameStage {
        fn is_game(state: Res<State<GameStage>>) -> ShouldRun {
            (state.current() == &GameStage::Playing).into()
        }

        fn is_menu(state: Res<State<GameStage>>) -> ShouldRun {
            (state.current() == &GameStage::Menu).into()
        }
    }

    pub mod settings {
        use bevy::{prelude::*, utils::HashMap};
        use bevy_quickmenu::{
            style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuState, QuickMenuPlugin,
            ScreenTrait,
        };

        use crate::{game::GameStage, loading::assets::PlayerTextureHandles};
        /// This custom event can be emitted by the action handler (below) in order to
        /// process actions with access to the bevy ECS
        #[derive(Debug)]
        enum MyEvent {
            CloseSettings,
        }

        /// This state represents the UI. Mutations to this state (via `MenuState::state_mut`)
        /// cause a re-render of the menu UI
        #[derive(Debug, Clone)]
        struct CustomState {
            sound_on: bool,
            gamepads: Vec<(Gamepad, String)>,
            controls: HashMap<usize, ControlDevice>,
            logo: Handle<Image>,
        }

        pub struct SettingsPlugin;

        impl Plugin for SettingsPlugin {
            fn build(&self, app: &mut App) {
                app
                    // Register a event that can be called from your action handler
                    .add_event::<MyEvent>()
                    // The plugin
                    .add_plugin(QuickMenuPlugin::<CustomState, Actions, Screens>::new())
                    // Some systems
                    .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(setup_system))
                    .add_system_set(
                        SystemSet::new()
                            .with_run_criteria(GameStage::is_menu)
                            .with_system(event_reader)
                            .with_system(update_gamepads_system),
                    );
            }
        }

        fn setup_system(mut commands: Commands, assets: ResMut<PlayerTextureHandles>) {
            // Create a default stylesheet. You can customize these as you wish
            let sheet = Stylesheet::default().with_background(BackgroundColor(Color::BLACK));

            commands.insert_resource(MenuState::new(
                CustomState {
                    sound_on: true,
                    gamepads: Vec::new(),
                    controls: [
                        (0, ControlDevice::keyboard1()),
                        (1, ControlDevice::keyboard2()),
                        (2, ControlDevice::keyboard3()),
                        (3, ControlDevice::keyboard4()),
                    ]
                    .into(),
                    logo: assets.rex_attack.clone(),
                },
                Screens::Root,
                Some(sheet),
            ))
        }

        /// Whenever a new gamepad connects, get the known gamepads and their names
        /// into our state
        fn update_gamepads_system(
            gamepads: Res<Gamepads>,
            menu_state: Option<ResMut<MenuState<CustomState, Actions, Screens>>>,
        ) {
            let Some(mut menu_state) = menu_state else {
            return
        };
            let gamepads = gamepads
                .iter()
                .map(|p| {
                    (
                        p,
                        gamepads
                            .name(p)
                            .map(std::borrow::ToOwned::to_owned)
                            .unwrap_or_default(),
                    )
                })
                .collect();
            if menu_state.state().gamepads != gamepads {
                menu_state.state_mut().gamepads = gamepads;
            }
        }

        /// The possible actions in our settings
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
        enum Actions {
            Close,
            SoundOn,
            SoundOff,
            Control(usize, ControlDevice),
        }

        /// Handle the possible actions
        impl ActionTrait for Actions {
            type State = CustomState;
            type Event = MyEvent;
            fn handle(&self, state: &mut CustomState, event_writer: &mut EventWriter<MyEvent>) {
                match self {
                    Actions::Close => event_writer.send(MyEvent::CloseSettings),
                    Actions::SoundOn => state.sound_on = true,
                    Actions::SoundOff => state.sound_on = false,
                    Actions::Control(p, d) => {
                        state.controls.insert(*p, *d);
                    }
                }
            }
        }

        /// All possible screens in our settings
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
        enum Screens {
            Root,
            Controls,
            Sound,
            Player(usize),
        }

        impl ScreenTrait for Screens {
            type Action = Actions;
            fn resolve(&self, state: &CustomState) -> Menu<Actions, Screens, CustomState> {
                match self {
                    Screens::Root => root_menu(state),
                    Screens::Controls => controls_menu(state),
                    Screens::Sound => sound_menu(state),
                    Screens::Player(p) => player_controls_menu(state, *p),
                }
            }
        }

        /// The `root` menu that is displayed first
        fn root_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
            Menu::new(
                "root",
                vec![
                    MenuItem::image(state.logo.clone()),
                    MenuItem::headline("Menu"),
                    MenuItem::action("Start", Actions::Close),
                    MenuItem::screen("Sound", Screens::Sound).with_icon(MenuIcon::Sound),
                    MenuItem::screen("Controls", Screens::Controls).with_icon(MenuIcon::Controls),
                ],
            )
        }

        /// This is displayed if the user selects `Sound` in the `root_menu`
        fn sound_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
            Menu::new(
                "sound",
                vec![
                    MenuItem::label("Toggles sound and music"),
                    MenuItem::action("On", Actions::SoundOn).checked(state.sound_on),
                    MenuItem::action("Off", Actions::SoundOff).checked(!state.sound_on),
                ],
            )
        }

        /// This is displayed if the user selects `Controls` in the `root_menu`
        fn controls_menu(state: &CustomState) -> Menu<Actions, Screens, CustomState> {
            let mut players: Vec<usize> = state.controls.keys().copied().collect();
            players.sort_unstable();
            Menu::new(
                "controls",
                players
                    .into_iter()
                    .map(|player| {
                        MenuItem::screen(format!("Player {player}"), Screens::Player(player))
                    })
                    .collect(),
            )
        }

        /// This is displayed if the user selects a player in the `controls_menu`
        fn player_controls_menu(
            state: &CustomState,
            player: usize,
        ) -> Menu<Actions, Screens, CustomState> {
            let selected_control = state.controls[&player];
            // Get the Keyboards
            let mut entries: Vec<_> = vec![
                ControlDevice::keyboard1(),
                ControlDevice::keyboard2(),
                ControlDevice::keyboard3(),
                ControlDevice::keyboard4(),
            ]
            .iter()
            .map(|kb| {
                MenuItem::action(kb.to_string(), Actions::Control(player, *kb))
                    .checked(kb.id() == selected_control.id())
            })
            .collect();

            // Get the GamePads
            for (pad, title) in &state.gamepads {
                let device = ControlDevice::Gamepad { gamepad_id: pad.id };
                entries.push(
                    MenuItem::action(title, Actions::Control(player, device))
                        .checked(device.id() == selected_control.id()),
                )
            }

            Menu::new("players", entries)
        }

        /// This allows to react to actions with custom bevy resources or eventwriters or queries.
        /// In this example we use it to close the menu
        fn event_reader(
            mut commands: Commands,
            mut event_reader: EventReader<MyEvent>,
            mut state: ResMut<State<GameStage>>,
        ) {
            for event in event_reader.iter() {
                match event {
                    MyEvent::CloseSettings => {
                        bevy_quickmenu::cleanup(&mut commands);
                        state.push(GameStage::Playing).unwrap();
                    }
                }
            }
        }

        // Abstractions over control devices

        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
        pub enum ControlDevice {
            Gamepad {
                gamepad_id: usize,
            },
            Keyboard {
                title: &'static str,
                description: &'static str,
                keyboard_id: usize,
                left: KeyCode,
                right: KeyCode,
                action: KeyCode,
            },
        }

        impl ControlDevice {
            #[must_use]
            pub fn id(&self) -> usize {
                match self {
                    ControlDevice::Gamepad { gamepad_id, .. } => *gamepad_id,
                    ControlDevice::Keyboard { keyboard_id, .. } => *keyboard_id,
                }
            }
        }

        impl std::fmt::Display for ControlDevice {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ControlDevice::Gamepad { gamepad_id } => {
                        f.write_fmt(format_args!("Gamepad {gamepad_id}",))
                    }
                    ControlDevice::Keyboard { title, .. } => f.write_fmt(format_args!("{title}")),
                }
            }
        }

        impl ControlDevice {
            #[must_use]
            pub fn keyboard1() -> ControlDevice {
                ControlDevice::Keyboard {
                    title: "Keyboard 1",
                    description: "Left / Right + M",
                    keyboard_id: 42001,
                    left: KeyCode::Left,
                    right: KeyCode::Right,
                    action: KeyCode::M,
                }
            }

            #[must_use]
            pub fn keyboard2() -> ControlDevice {
                ControlDevice::Keyboard {
                    title: "Keyboard 2",
                    description: "A / D + B",
                    keyboard_id: 42002,
                    left: KeyCode::A,
                    right: KeyCode::D,
                    action: KeyCode::B,
                }
            }
            #[must_use]
            pub fn keyboard3() -> ControlDevice {
                ControlDevice::Keyboard {
                    title: "Keyboard 3",
                    description: "I / O + K",
                    keyboard_id: 42003,
                    left: KeyCode::I,
                    right: KeyCode::O,
                    action: KeyCode::K,
                }
            }
            #[must_use]
            pub fn keyboard4() -> ControlDevice {
                ControlDevice::Keyboard {
                    title: "Keyboard 4",
                    description: "T / Y + H",
                    keyboard_id: 42004,
                    left: KeyCode::T,
                    right: KeyCode::Y,
                    action: KeyCode::H,
                }
            }
        }
    }

    // Replicate a simple game to show how to go back to the
    // menu screen and show it again
    pub mod game {
        use bevy::prelude::*;

        use crate::{game::GameStage, loading::assets::FontHandles};

        pub struct InGameMenuPlugin;
        impl Plugin for InGameMenuPlugin {
            fn build(&self, app: &mut App) {
                app.add_system_set(
                    SystemSet::on_enter(GameStage::Playing).with_system(setup_system),
                )
                .add_system(detect_close_system.with_run_criteria(GameStage::is_game))
                .run();
            }
        }

        #[derive(Component)]
        struct GameComponent;

        fn setup_system(mut commands: Commands, font_assets: Res<FontHandles>) {
            commands
                .spawn((TextBundle::from_section(
                    "Return Key to go back to menu",
                    TextStyle {
                        font: font_assets.fantasque_sans_ttf.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(60.0),
                        left: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                }),))
                .insert(GameComponent);
        }

        fn detect_close_system(
            // mut commands: Commands,
            keyboard_input: Res<Input<KeyCode>>,
            mut state: ResMut<State<GameStage>>,
            // game_items: Query<Entity, With<GameComponent>>,
        ) {
            if keyboard_input.just_pressed(KeyCode::Return) {
                state.push(GameStage::Menu).unwrap();
            }
        }
    }
}

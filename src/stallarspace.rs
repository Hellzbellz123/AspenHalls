use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::keyboard::ElementState,
    input::mouse::MouseButtonInput,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

struct Solid {}
const WALL_WIDTH: f32 = 30.0;
const WALL_COLOR: Color = Color::rgb(1.8, 1.2, 1.2);
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

const CHAR_PATH: &str = "assets/GIMP FIGURES/char_1-Sheet.png";

fn map_setup(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = match windows.get_primary() {
        Some(a) => Vec2::new(a.width as f32, a.height as f32),
        None => Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
    };

    println!("Width: {} Height: {}", window.x(), window.y());
    commands
        .spawn(SpriteComponents {
            material: materials.add(WALL_COLOR.into()),
            translation: Translation(Vec3::new(window.x() / 2.0, 0.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(WALL_WIDTH, window.y() + 40.0),
            },
            ..Default::default()
        })
        .with(Solid {});

    commands
        .spawn(SpriteComponents {
            material: materials.add(WALL_COLOR.into()),
            translation: Translation(Vec3::new(-1.0 * window.x() / 2.0, 0.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(WALL_WIDTH, window.y() + 40.0),
            },
            ..Default::default()
        })
        .with(Solid {});

    commands
        .spawn(SpriteComponents {
            material: materials.add(WALL_COLOR.into()),
            translation: Translation(Vec3::new(0.0, -1.0 * window.y() / 2.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(window.x(), WALL_WIDTH),
            },
            ..Default::default()
        })
        .with(Solid {});

    commands
        .spawn(SpriteComponents {
            material: materials.add(WALL_COLOR.into()),
            translation: Translation(Vec3::new(0.0, window.y() / 2.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(window.x(), WALL_WIDTH),
            },
            ..Default::default()
        })
        .with(Solid {});
}

fn get_available_location(
    outside_loc: Vec2,
    outside_size: Vec2,
    inside_loc: Vec2,
    inside_size: Vec2,
    target_size: Vec2,
) -> Vec2 {
    let outside_x_bound = outside_loc.x() / 2.0 - outside_size.x() / 2.0 - target_size.x();
    let inside_x_bound = inside_loc.x() / 2.0 + inside_size.x() / 2.0 + target_size.x();
    let outside_y_bound = outside_loc.y() / 2.0 - outside_size.y() / 2.0 - target_size.y();
    let inside_y_bound = inside_loc.y() / 2.0 - inside_size.x() / 2.0 + target_size.y();

    let mut rng = rand::thread_rng();
    let right_rand = rng.gen_range(inside_x_bound, outside_x_bound);
    let left_rand = rng.gen_range(-1.0 * outside_x_bound, -1.0 * inside_x_bound);
    let top_rand = rng.gen_range(inside_y_bound, outside_y_bound);
    let bottom_rand = rng.gen_range(-1.0 * outside_y_bound, -1.0 * inside_y_bound);

    match rng.gen_range(1, 5) {
        1 => Vec2::new(right_rand, top_rand),
        2 => Vec2::new(left_rand, top_rand),
        3 => Vec2::new(left_rand, bottom_rand),
        4 => Vec2::new(right_rand, bottom_rand),
        _ => panic!("Number Generated that wasn't accounted for"),
    }
}

fn _a_is_within_b(a_loc: Vec2, a_size: Vec2, b_loc: Vec2, b_size: Vec2) -> bool {
    let a_left = a_loc.x() - a_size.x() / 2.0;
    let a_right = a_loc.x() + a_size.x() / 2.0;
    let a_top = a_loc.y() + a_size.y() / 2.0;
    let a_bottom = a_loc.y() - a_size.y() / 2.0;

    let b_left = b_loc.x() - b_size.x() / 2.0;
    let b_right = b_loc.x() + b_size.x() / 2.0;
    let b_top = b_loc.y() + b_size.y() / 2.0;
    let b_bottom = b_loc.y() - b_size.y() / 2.0;

    if a_left >= b_left && a_right <= b_right && a_top <= b_top && a_bottom >= b_bottom {
        println!("Hello");
        return true;
    }
    false
}

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(goal_setup.system())
            .add_system(check_goal_system.system());
    }
}

struct Goal(usize);
fn goal_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let _window = match windows.get_primary() {
        Some(a) => Vec2::new(a.width as f32, a.height as f32),
        None => Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
    };
    // creates location of goal
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            translation: Translation(Vec3::new(-200.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(50.0, 50.0),
            },
            ..Default::default()
        })
        .with(Goal(0));
}

fn check_goal_system(
    //textures: ResMut<Assets<TextureAtlas>>,
    mut goal_query: Query<(&mut Goal, &Sprite, &mut Translation)>,
    mut player_query: Query<(&MainDude, &Handle<TextureAtlas>, &mut Translation)>,
) {
    let _window = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    for (mut score, goal_sprite, mut goal_trans) in &mut goal_query.iter() {
        for (_, _texture_handle, mut player_trans) in &mut player_query.iter() {
            let collision = collide(
                player_trans.0,
                Vec2::new(GUYWIDTH * SCALE_FACTOR, GUYHEIGHT * SCALE_FACTOR),
                goal_trans.0,
                goal_sprite.size,
            );
            if let Some(_collision) = collision {
                score.0 += 1;
                println!("Score is: {}", score.0);
                println!("On the Green");
                let new_loc = get_available_location(
                    Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                    Vec2::new(WALL_WIDTH, WALL_WIDTH),
                    Vec2::new(0.0, -50.0),
                    Vec2::new(150.0, 150.0),
                    Vec2::new(GUYWIDTH * SCALE_FACTOR, GUYHEIGHT * SCALE_FACTOR),
                );
                let new_loc2 = get_available_location(
                    Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                    Vec2::new(WALL_WIDTH, WALL_WIDTH),
                    Vec2::new(0.0, -50.0),
                    Vec2::new(150.0, 150.0),
                    goal_sprite.size,
                );
                println!("New Player Loc: {:?}", new_loc);
                *player_trans.x_mut() = new_loc.x();
                *player_trans.y_mut() = new_loc.y();

                println!("New Goal Loc: {:?}", new_loc2);
                *goal_trans.x_mut() = new_loc2.x();
                *goal_trans.y_mut() = new_loc2.y();
            }
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(map_setup.system());
    }
}

struct MainDude {
    speed: f32,
    target: Option<Vec2>,
}
const GUYSPEED: f32 = 20.0;
const SCALE_FACTOR: f32 = 3.0;
const GUYHEIGHT: f32 = 14.0;
const GUYWIDTH: f32 = 10.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(char_setup.system())
            .add_system(animate_sprite_system.system())
            .add_system(char_collision_system.system())
            .add_system(position_mouse_click_system.system());
        //.add_system(color_click_system.system());
    }
}

fn char_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load_sync(&mut textures, CHAR_PATH).unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    //asset_server.watch_for_changes().unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Camera and Main Character
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            translation: Translation(Vec3::new(-300.0, -300.0, 0.1)),
            scale: Scale(SCALE_FACTOR),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05, true))
        .with(MainDude {
            speed: GUYSPEED,
            target: None,
        });
}

fn animate_sprite_system(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &mut Translation,
        &MainDude,
    )>,
) {
    for (timer, mut _sprite, mut translation, maindude) in &mut query.iter() {
        if timer.finished {
            let _window = match windows.get_primary() {
                Some(a) => a,
                None => panic!("No Window!!! How is this Possible!!!"),
            };

            if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
                *translation.x_mut() -= maindude.speed;
            }
            if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
                *translation.x_mut() += maindude.speed;
            }
            if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
                *translation.y_mut() += maindude.speed;
            }
            if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
                *translation.y_mut() -= maindude.speed;
            }
        }
    }
}

fn char_collision_system(
    mut query_player: Query<(&MainDude, &mut TextureAtlasSprite, &mut Translation)>,
    mut query_walls: Query<(&Solid, &Sprite, &mut Translation)>,
) {
    for (_, _char_sprite, mut char_loc) in &mut query_player.iter() {
        for (_, wall_sprite, wall_loc) in &mut query_walls.iter() {
            let collision = collide(
                char_loc.0,
                Vec2::new(GUYWIDTH * SCALE_FACTOR, GUYHEIGHT * SCALE_FACTOR),
                wall_loc.0,
                wall_sprite.size,
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => {
                        *char_loc.x_mut() = wall_loc.0.x() - wall_sprite.size.x() / 2.0 - GUYWIDTH;
                    }
                    Collision::Right => {
                        *char_loc.x_mut() = wall_loc.0.x() + wall_sprite.size.x() / 2.0 + GUYWIDTH;
                    }
                    Collision::Top => {
                        *char_loc.y_mut() = wall_loc.0.y() + GUYHEIGHT + wall_sprite.size.y() / 2.0;
                    }
                    Collision::Bottom => {
                        *char_loc.y_mut() = wall_loc.0.y() - wall_sprite.size.y() / 2.0 - GUYHEIGHT
                    }
                }
            }
        }
    }
}

fn on_click_system(
    mut state: ResMut<State>,
    mouse_pos: Res<MouseLoc>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mut maindude: Mut<MainDude>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("event: {:?} position: {:?}", event, mouse_pos.0);
        if let ElementState::Pressed = event.state {
            maindude.target = Some(mouse_pos.0);
        }
    }
}

pub fn collide_2(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;

    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    if a_min.x() < b_max.x()
        && a_max.x() > b_min.x()
        && a_min.y() < b_max.y()
        && a_max.y() > b_min.y()
    {
        let (x_collision, x_depth) =
            if a_min.x() < b_min.x() && a_max.x() > b_min.x() && a_max.x() < b_max.x() {
                (Some(Collision::Left), b_min.x() - a_max.x())
            } else if a_min.x() > b_min.x() && a_min.x() < b_max.x() && a_max.x() > b_max.x() {
                (Some(Collision::Right), a_min.x() - b_max.x())
            } else {
                (None, 0.0)
            };

        let (y_collision, y_depth) =
            if a_min.y() < b_min.y() && a_max.y() > b_min.y() && a_max.y() < b_max.y() {
                (Some(Collision::Bottom), b_min.y() - a_max.y())
            } else if a_min.y() > b_min.y() && a_min.y() < b_max.y() && a_max.y() > b_max.y() {
                (Some(Collision::Top), a_min.y() - b_max.y())
            } else {
                (None, 0.0)
            };

        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth < x_depth {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => None,
        }
    } else {
        None
    }
}

fn position_mouse_click_system(
    time: Res<Time>,
    mut timer: ResMut<MouseTimer>,
    mut state: ResMut<State>,
    mut mouse_pos: ResMut<MouseLoc>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mut maindude: Mut<MainDude>,
    _: &TextureAtlasSprite,
    mut player_trans: Mut<Translation>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        if let Some(click_pos) = maindude.target {
            println!("Target: {}", click_pos);
            let click = Vec2::new(
                click_pos.x() - WINDOW_WIDTH as f32 / 2.0,
                click_pos.y() - WINDOW_HEIGHT as f32 / 2.0,
            );

            println!(
                "{} {} {} {}",
                player_trans.0.truncate(),
                Vec2::new(100.0, 100.0),
                click,
                Vec2::new(100.0, 100.0),
            );
            let collision = collide_2(
                player_trans.0.truncate(),
                Vec2::new(GUYWIDTH * SCALE_FACTOR, GUYHEIGHT * SCALE_FACTOR),
                click,
                Vec2::new(100.0, 100.0),
            );

            if let Some(_) = collision {
                println!("Is within");
                maindude.target = None;
            } else {
                if click.x() > player_trans.x() {
                    *player_trans.x_mut() += maindude.speed;
                } else {
                    *player_trans.x_mut() -= maindude.speed;
                }
                if click.y() > player_trans.y() {
                    *player_trans.y_mut() += maindude.speed;
                } else {
                    *player_trans.y_mut() -= maindude.speed;
                }
            }
        }
    }
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct MouseLoc(Vec2);

fn mouse_movement_updating_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut state: ResMut<State>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse_pos.0 = event.position;
    }
}

fn general_setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

struct MouseTimer(Timer);
pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.5, 0.20, 0.80)))
            .init_resource::<State>()
            .add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
            .add_resource(MouseTimer(Timer::from_seconds(0.1, true)))
            .add_startup_system(general_setup.system())
            .add_system(mouse_movement_updating_system.system())
            .add_system(on_click_system.system());
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Stellar Space".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(GeneralPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(GoalPlugin)
        .run();
}

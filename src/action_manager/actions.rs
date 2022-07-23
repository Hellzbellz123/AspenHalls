use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GameActions {
    Right,
    Left,
    Down,
    Up,

    Horizontal,
    Vertical,

    Climb,
    Dash,
    Sprint,
    Pause,
    // Heal,
    // Menus,
}

// impl GameActions {
//     // Lists like this can be very useful for quickly matching subsets of actions
//     pub const DIRECTIONS: [Self; 4] = [
//         GameActions::Up,
//         GameActions::Down,
//         GameActions::Left,
//         GameActions::Right,
//     ];

//     pub fn direction(self) -> Option<Direction> {
//         match self {
//             GameActions::Up => Some(Direction::NORTH),
//             GameActions::Down => Some(Direction::SOUTH),
//             GameActions::Left => Some(Direction::EAST),
//             GameActions::Right => Some(Direction::WEST),
//             _ => None,
//         }
//     }
// }

use bevy_console::ConsoleCommand;
use clap::Parser;

/// spawn weapon [`WeaponType`] x amount of times using `SpawnWeaponEvent`
#[derive(ConsoleCommand, Parser, Debug)]
#[command(name = "spawnweapon")] //, author, version, about, long_about = None)]
pub struct SpawnWeaponCommand {
    /// type of w to spawn
    pub weapon_type: String,
    pub loc_x: Option<i64>,
    /// y transform
    pub loc_y: Option<i64>,
    /// Number of times to spawn
    pub amount: Option<i32>,
    /// spawn at/near player
    #[arg(short = '@', long = "at_player")]
    pub atplayer: Option<bool>,
}

///  spawns enemy [`EnemyType`] x amount of times using `SpawnEnemyEvent`
#[derive(ConsoleCommand, Parser)]
#[command(name = "spawnenemy")]
pub struct SpawnEnemyCommand {
    /// type of thing to spawn
    pub enemy_type: String,
    /// x transform
    pub loc_x: Option<i64>,
    /// y transform
    pub loc_y: Option<i64>,
    /// Number of times to spawn
    pub amount: Option<i32>,
    /// spawn at/near player
    pub atplayer: Option<bool>,
}

/// Teleports the Player to x y coords
#[derive(ConsoleCommand, Parser)]
#[command(name = "teleport")]
pub struct TeleportPlayerCommand {
    pub loc_x: i64,
    pub loc_y: i64,
}

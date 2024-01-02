use bevy::math::Vec2;
use bevy_console::ConsoleCommand;
use clap::{Error, Parser};

use crate::loading::registry::RegistryIdentifier;

///  spawns requested actor amount of times
#[derive(Debug, ConsoleCommand, Parser)]
#[command(name = "spawn")]
pub struct SpawnActorCommand {
    /// type of thing
    pub actor_type: CommandSpawnType,
    /// thing too spawn
    pub identifier: RegistryIdentifier,
    /// spawn position
    pub position: Option<CommandPosition>,
    /// spawn at/near player
    pub where_spawn: Option<CommandTarget>,
    /// Number of times to spawn
    pub amount: Option<i32>,
}

/// Teleports the character too x y coords
#[derive(ConsoleCommand, Parser)]
#[command(name = "teleport")]
pub struct TeleportCharacterCommand {
    /// where too teleport too
    pub pos: CommandPosition,
    /// Teleport Target
    /// - @p : targets player
    /// - @n : targets nearest character
    /// - @e : targets everyone
    pub who: Option<CommandTarget>,
}

//######## COMMAND ARGS ########//
/// x,y location for comand
#[derive(Debug, Clone, Copy)]
pub struct CommandPosition(pub f32, pub f32);

/// who should this command target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandTarget {
    /// target player
    Player,
    /// target nearest entity
    Nearest,
    /// target all targetable entities
    Everyone,
}

/// what should this command spawn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSpawnType {
    /// spawn item
    Item,
    /// spawn npc
    Npc,
}

//######## ARG IMPL ########//
impl std::str::FromStr for CommandSpawnType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "npc" | "creep" => Ok(Self::Npc),
            "weapon" | "item" => Ok(Self::Item),
            _ => Err(Error::new(clap::error::ErrorKind::ValueValidation)),
        }
    }
}

impl std::str::FromStr for CommandTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "@p" => Ok(Self::Player),
            "@n" => Ok(Self::Nearest),
            "@e" => Ok(Self::Everyone),
            _ => Err(Error::new(clap::error::ErrorKind::ValueValidation)),
        }
    }
}

impl std::str::FromStr for CommandPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("PARSE_VEC: {s}");
        // Remove leading and trailing whitespaces
        let s = s.trim();

        // Check if the string starts with '(' and ends with ')'
        if s.starts_with('(') && s.ends_with(')') {
            // Extract the content between '(' and ')' and split it into components
            let content = &s[1..s.len() - 1];
            let components: Vec<&str> = content.split(',').collect();

            // Ensure there are exactly two components
            if components.len() == 2 {
                // Parse the components into f64 values
                let Ok(x) = components[0].trim().parse::<f32>() else {
                    return Err(Error::new(clap::error::ErrorKind::InvalidValue));
                };
                let Ok(y) = components[1].trim().parse::<f32>() else {
                    return Err(Error::new(clap::error::ErrorKind::InvalidValue));
                };
                // Return the Vec2
                Ok(Self(x, y))
            } else {
                Err(Error::new(clap::error::ErrorKind::TooManyValues))
            }
        } else {
            // Extract the content between '(' and ')' and split it into components
            let content = &s.trim();
            let components: Vec<&str> = content.split(',').collect();

            // Ensure there are exactly two components
            if components.len() == 2 {
                // Parse the components into f64 values
                let Ok(x) = components[0].trim().parse::<f32>() else {
                    return Err(Error::new(clap::error::ErrorKind::InvalidValue));
                };
                let Ok(y) = components[1].trim().parse::<f32>() else {
                    return Err(Error::new(clap::error::ErrorKind::InvalidValue));
                };
                // Return the Vec2
                Ok(Self(x, y))
            } else {
                Err(Error::new(clap::error::ErrorKind::InvalidSubcommand))
            }
        }
    }
}

impl From<Vec2> for CommandPosition {
    fn from(value: Vec2) -> Self {
        Self(value.x, value.y)
    }
}

impl From<CommandPosition> for Vec2 {
    fn from(value: CommandPosition) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

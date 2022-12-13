use std::{error::Error, fmt};

/// this is generally commandconsole only
/// An error returned when parsing a `EnemyType` using [`from_str`] fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnemyTypeError;

impl fmt::Display for ParseEnemyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid EnemyType, Possible values are [Skeleton, Slime]".fmt(f)
    }
}

impl Error for ParseEnemyTypeError {
    fn description(&self) -> &str {
        "failed to parse bool"
    }
}

/// this is generally commandconsole only
/// An error returned when parsing a `WeaponType` using [`from_str`] fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWeaponTypeError;

impl fmt::Display for ParseWeaponTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid WeaponType, Possible values are [Pistol, SmallSMG]".fmt(f)
    }
}

impl Error for ParseWeaponTypeError {
    fn description(&self) -> &str {
        "failed to parse string"
    }
}

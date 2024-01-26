/// holds blade style weapon plugin
mod blade;
/// holds flail style weapon plugin
mod flail;
/// holds gun style weapon plugin
mod gun;

pub use gun::{GunShootEvent, GunWeaponsPlugin, format_gun_animations};

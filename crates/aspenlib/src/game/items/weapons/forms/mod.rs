/// holds blade style weapon plugin
mod blade;
/// holds flail style weapon plugin
mod flail;
/// holds gun style weapon plugin
mod gun;

pub use gun::{create_bullet, format_gun_animations, GunShootEvent, GunWeaponsPlugin};

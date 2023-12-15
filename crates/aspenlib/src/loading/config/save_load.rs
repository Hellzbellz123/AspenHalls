use crate::loading::config::ConfigFile;
use std::path::Path;

/// saves a `ConfigFile` at `&Path`
/// will panic if config file cant be deserialized into a string,
/// or if it cant write a file
///
/// # Panics
/// this function will panic if it cant serialize `app_settings`
/// OR
/// if `std::fs::write` encounters an error writing too `settings_path`
pub fn save_settings(app_settings: &ConfigFile, settings_path: &Path) {
    println!("Saving AppSettings, this overwrites current settings");
    let serde_cfg = match toml::to_string(&app_settings) {
        Ok(cfg) => {
            println!("Successfully converted config file");
            cfg
        }
        Err(error) => {
            eprintln!("Couldn't convert game settings too `String`: {error}");
            return;
        }
    };
    match std::fs::write(settings_path, serde_cfg) {
        Ok(()) => println!(
            "Successfully saved settings too {}",
            settings_path.display()
        ),
        Err(e) => eprintln!(
            "There was an error saving config too {}: {}",
            settings_path.display(),
            e
        ),
    };
}

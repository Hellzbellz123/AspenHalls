use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        interface::{
            menus::{EventType, PausePlayEvent},
            InterfaceRoot,
        },
        AppStage,
    },
    loading::config::SoundSettings,
};

/// Set up the main menu
pub fn setup_menu(app: &mut App) {
    app.add_systems(
        OnEnter(AppStage::StartMenu),
        (
            SettingsMenu::create.run_if(not(any_with_component::<SettingsMenu>())),
            // SettingsMenu::hide.run_if(any_with_component::<SettingsMenu>()),
        ),
    )
    .add_systems(
        Update,
        update_menu_volumes.run_if(any_with_component::<SettingsMenu>()),
    );
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct SettingsMenu;

impl SettingsMenu {
    /// Create the settings menu menu
    fn create(
        root: Res<InterfaceRoot>,
        mut elements: Elements,
        mut commands: Commands,
        sound_settings: Res<SoundSettings>,
    ) {
        commands.entity(**root).insert(Self);
        let (current_ambience, current_sound, current_master, current_music) = (
            sound_settings.ambience_volume as f32,
            sound_settings.sound_volume as f32,
            sound_settings.master_volume as f32,
            sound_settings.music_volume as f32,
        );

        let master_slider = commands
            .spawn(TempMasterVolume(sound_settings.master_volume as f32))
            .id();
        let actor_slider = commands
            .spawn(TempSoundVolume(sound_settings.sound_volume as f32))
            .id();
        let ambience_slider = commands
            .spawn(TempAmbienceVolume(sound_settings.ambience_volume as f32))
            .id();
        let music_slider = commands
            .spawn(TempMusicVolume(sound_settings.music_volume as f32))
            .id();

        elements.select(".interface-root").add_child(eml! {
            <div c:settings-menu-root c:hidden>
            <div c:settings-cfg-box>
                    <span c:settings-title> "Settings Menu" </span>
                    <span c:sound_slider>
                        "Master Volume"
                        <slider {master_slider}
                        s:width="100px" s:margin-left="10%"
                        mode="horizontal" minimum=0.0 value=current_master maximum=1.0
                        bind:value=to!(master_slider, TempMasterVolume:0)
                        bind:value=from!(master_slider, TempMasterVolume:0)
                        />
                    </span>
                    <span c:sound_slider>
                        "Actor Volume"
                        <slider {actor_slider}
                        s:width="100px" s:margin-left="10%"
                        mode="horizontal" minimum=0.0 value=current_sound maximum=1.0
                        bind:value=to!(actor_slider, TempSoundVolume:0)
                        bind:value=from!(actor_slider, TempSoundVolume:0)
                        />
                    </span>
                    <span c:sound_slider>
                        "Ambience Volume"
                        <slider {ambience_slider}
                        s:width="100px" s:margin-left="10%"
                        mode="horizontal" minimum=0.0 value=current_ambience maximum=1.0
                        bind:value=to!(ambience_slider, TempAmbienceVolume:0)
                        bind:value=from!(ambience_slider, TempAmbienceVolume:0)
                        />
                    </span>
                    <span c:sound_slider>
                        "Music Volume"
                        <slider {music_slider}
                        s:width="100px" s:margin-left="10%"
                        mode="horizontal" minimum=0.0 value=current_music maximum=1.0
                        bind:value=to!(music_slider, TempMusicVolume:0)
                        bind:value=from!(music_slider, TempMusicVolume:0)
                        />
                    </span>
                    <span c:option-container> "toggle button" <button c:button c:toggle> "[]"</button>  </span>
                    <span c:option-container> "toggle button" <button c:button c:toggle> "[]"</button>  </span>
                </div>
                <div c:settings-buttons-bottom>
                    <button c:button on:press=|ctx| { ctx.send_event(PausePlayEvent(EventType::Previous)) }>
                        "Back Too StartMenu"
                    </button>
                    <div c:menu-version>
                        "VanillaCoffee v"
                        { env!("CARGO_PKG_VERSION") }
                    </div>
                    <div c:menu-disclaimer>
                        "ALPHA SOFTWARE - USE AT YOUR OWN RISK"
                    </div>
                </div>
            </div>
        });
    }

    /// Show the main menu
    #[allow(dead_code)]
    pub fn show(mut elements: Elements) {
        elements
            .select("div.settings-menu-root")
            .remove_class("hidden");
    }

    /// Hide the main menu
    #[allow(dead_code)]
    pub fn hide(mut elements: Elements) {
        elements
            .select("div.settings-menu-root")
            .add_class("hidden");
    }

    /// Function to handle button clicks
    #[allow(dead_code)]
    fn click_button(ctx: &mut EventContext<impl Event>, query: &str) {
        ctx.select("div.settings-menu-root").add_class("hidden");
        ctx.select(query).remove_class("hidden");
    }
}

/// mirror component for `MasterVolume`
#[derive(Debug, Component, Default)]
struct TempMasterVolume(f32);

/// mirror component for `AmbienceVolume`
#[derive(Debug, Component, Default)]
struct TempAmbienceVolume(f32);

/// mirror component for `SoundVolume`
#[derive(Debug, Component, Default)]
struct TempSoundVolume(f32);

/// mirror component for `MusicVolume`
#[derive(Debug, Component, Default)]
struct TempMusicVolume(f32);

/// updates settings resource if mirror components change
fn update_menu_volumes(
    mut sound_settings: ResMut<SoundSettings>,
    master_vol: Query<&TempMasterVolume, Changed<TempMasterVolume>>,
    sound_vol: Query<&TempSoundVolume, Changed<TempSoundVolume>>,
    ambience_vol: Query<&TempAmbienceVolume, Changed<TempAmbienceVolume>>,
    music_vol: Query<&TempMusicVolume, Changed<TempMusicVolume>>,
) {
    let error_margin = 0.01;
    for vol in &master_vol {
        if (sound_settings.master_volume as f32 - vol.0).abs() > error_margin {
            sound_settings.master_volume = f64::from(vol.0);
        }
    }

    for vol in &sound_vol {
        if (sound_settings.sound_volume as f32 - vol.0).abs() > error_margin {
            sound_settings.sound_volume = f64::from(vol.0);
        }
    }

    for vol in &ambience_vol {
        if (sound_settings.ambience_volume as f32 - vol.0).abs() > error_margin {
            sound_settings.ambience_volume = f64::from(vol.0);
        }
    }

    for vol in &music_vol {
        if (sound_settings.music_volume as f32 - vol.0).abs() > error_margin {
            sound_settings.music_volume = f64::from(vol.0);
        }
    }
}

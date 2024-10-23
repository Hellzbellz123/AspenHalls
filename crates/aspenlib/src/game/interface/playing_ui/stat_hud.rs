use crate::game::{attributes_stats::CharacterStats, characters::player::PlayerSelectedHero};
use bevy::prelude::*;

/// create player hud / vitals holder
pub fn create_player_hud(playing_ui_parts: &mut ChildBuilder) {
    playing_ui_parts
        .spawn((
            Name::new("PlayerHud"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Row,
                    align_self: AlignSelf::FlexStart,
                    justify_content: JustifyContent::SpaceBetween,
                    height: Val::Percent(35.0),
                    width: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Percent(5.0),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Px(5.0),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|hud_parts| {
            create_hero_portrait(hud_parts);
            create_vitals_hud(hud_parts);
        });
}

/// create player portrait widget
fn create_hero_portrait(hud_parts: &mut ChildBuilder) {
    hud_parts
        .spawn((
            Name::new("PortraitOuter"),
            Outline {
                width: Val::Px(5.0),
                offset: Val::Px(0.0),
                color: super::colors::OUTLINE,
            },
            NodeBundle {
                background_color: BackgroundColor(super::colors::BACKDARK),
                style: Style {
                    width: Val::Percent(35.0),
                    height: Val::Percent(100.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|picture| {
            picture.spawn((
                Name::new("PlayerPortrait"),
                UiPlayerPortrait,
                TextureAtlas {
                    layout: Handle::default(),
                    index: 0,
                },
                ImageBundle {
                    style: Style {
                        width: Val::Percent(200.0),
                        height: Val::Percent(200.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

#[derive(Debug, Component)]
/// marker component for ui portrait widget
pub struct UiPlayerPortrait;

/// vitals hud widget
fn create_vitals_hud(hud_parts: &mut ChildBuilder) {
    hud_parts
        .spawn((
            Name::new("StatsContainer"),
            Outline {
                width: Val::Px(3.0),
                offset: Val::default(),
                color: super::colors::OUTLINE,
            },
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    width: Val::Percent(58.0),
                    height: Val::Percent(95.0),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(super::colors::BACKLIGHT),
                ..default()
            },
        ))
        .with_children(|stat_bars| {
            statbar_widget(
                stat_bars,
                StatBar::HEALTH,
                "Health",
                25.0,
                super::colors::HPEMPTY,
                super::colors::HPFULL,
            );
            statbar_widget(
                stat_bars,
                StatBar::ENERGY,
                "Energy",
                20.0,
                super::colors::UTILITYEMPTY,
                super::colors::MANAFULL,
            );
        });
}

/// creates a statbar widget inside a node
pub fn statbar_widget(
    stat_bars: &mut ChildBuilder,
    bar_type: StatBar,
    title: &str,
    height: f32,
    background: Color,
    foreground: Color,
) {
    let text_name = format!("{title}Text");
    let container_name = format!("{title}BarContainer");
    stat_bars.spawn((
        Name::new(text_name),
        TextBundle::from_section(
            title,
            TextStyle {
                font_size: 18.0,
                ..default()
            },
        ),
    ));
    stat_bars
        .spawn((
            Name::new(container_name),
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        top: Val::Auto,
                        bottom: Val::Auto,
                        ..default()
                    },
                    align_self: AlignSelf::Center,
                    width: Val::Percent(95.0),
                    height: Val::Percent(height),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(background),
                ..default()
            },
        ))
        .with_children(|bar_parts| {
            bar_parts.spawn((
                bar_type,
                Name::new("HealthBar"),
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(44.0),
                        ..default()
                    },
                    background_color: BackgroundColor(foreground),
                    ..default()
                },
            ));
        });
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
/// ui stat bar data component
pub struct StatBar {
    /// what resource this bar tracks
    resource: BarResource,
    /// max value of this bar
    max: f32,
    /// current value of this bar
    current: f32,
}

#[derive(Debug, Reflect, Default)]
/// type of resource statbar widget displays
pub enum BarResource {
    /// bar displays hp values
    #[default]
    Health,
    /// bar displays energy values
    Energy,
}

impl StatBar {
    /// 1/1 energy bar
    pub const ENERGY: Self = Self {
        resource: BarResource::Energy,
        max: 1.0,
        current: 1.0,
    };

    /// 1/1 health bar
    pub const HEALTH: Self = Self {
        resource: BarResource::Health,
        max: 1.0,
        current: 1.0,
    };
}

/// modifys player portrait too currently selected player
/// only runs if portrait handle is not player sprite atlas
#[allow(clippy::type_complexity)]
pub fn update_player_portrait(
    player_query: Query<(&TextureAtlas, &Handle<Image>), With<PlayerSelectedHero>>,
    mut player_portrait: Query<
        (&mut UiImage, &mut TextureAtlas),
        (With<UiPlayerPortrait>, Without<PlayerSelectedHero>),
    >,
) {
    let (mut portrait_image, mut portrait_atlas) = player_portrait.single_mut();
    let (player_atlas, player_image) = player_query.single();

    if portrait_image.texture != *player_image {
        portrait_image.texture = player_image.clone_weak();
        portrait_atlas.layout = player_atlas.layout.clone_weak();
    }
}

/// updates statbars with character stats values
pub fn update_player_hp_bar(
    player_query: Query<(Entity, &CharacterStats), With<PlayerSelectedHero>>,
    mut bar_query: Query<(&mut StatBar, &mut Style)>,
) {
    let Ok((_, stats)) = player_query.get_single() else {
        warn!("no player stats too update player stats ui with");
        return;
    };

    for (mut bar_value, mut style) in &mut bar_query {
        let (max, current) = match bar_value.resource {
            BarResource::Health => (stats.attrs().max_hp, stats.health),
            BarResource::Energy => (stats.attrs().max_mana, stats.mana),
        };
        bar_value.max = max;
        bar_value.current = current;

        let percentage = (current / max) * 100.0;
        let clamped_percentage = percentage.clamp(0.0, 100.0);

        style.width = Val::Percent(clamped_percentage);
    }
}

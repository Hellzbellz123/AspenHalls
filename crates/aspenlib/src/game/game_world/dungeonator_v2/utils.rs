use std::collections::VecDeque;

use bevy::{
    ecs::system::Res,
    log::{info, warn},
    math::{Rect, Vec2},
};

use rand::{
    prelude::{Rng, ThreadRng},
    seq::IteratorRandom,
};

use crate::{
    consts::TILE_SIZE,
    game::game_world::dungeonator_v2::components::{
        DungeonRoomDatabase, DungeonSettings, RoomLevel, RoomPreset,
    },
};

/// gets ANY random preset from `presets`
pub fn get_random_preset(presets: &[RoomPreset]) -> Option<&RoomPreset> {
    let mut rng = ThreadRng::default();

    presets.iter().choose(&mut rng)
}

/// get random preset that matches `level` from `presets`
pub fn get_leveled_preset<'a>(
    presets: &'a [RoomPreset],
    _level: &'a RoomLevel,
) -> Option<&'a RoomPreset> {
    let mut rng = ThreadRng::default();

    presets
        .iter()
        // TODO ADD LEVELED START/END rooms
        // .filter(|f| f.descriptor.level == *level)
        .choose(&mut rng)
}

/// chooses selected amount of rooms for each room class
pub fn choose_filler_presets<'a>(
    settings: &'a DungeonSettings,
    room_database: &'a Res<'a, DungeonRoomDatabase>,
) -> VecDeque<&'a RoomPreset> {
    let mut chosen_presets: VecDeque<&RoomPreset> = VecDeque::new();
    let room_cfg: &super::components::RoomDistribution = &settings.distribution;

    // TODO: choose presets more smartly
    // if room size is large add bigger size presets
    // if small prefer smaller rooms
    for _ in 0..room_cfg.small_short {
        if !room_database.small_short_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.small_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.small_long {
        if !room_database.small_long_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.small_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.medium_short {
        if !room_database.medium_short_rooms.is_empty() {
            chosen_presets
                .push_front(get_random_preset(&room_database.medium_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.medium_long {
        if !room_database.medium_long_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.medium_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.large_short {
        if !room_database.large_short_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.large_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.large_long {
        if !room_database.large_long_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.large_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.huge_short {
        if !room_database.huge_short_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.huge_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.huge_long {
        if !room_database.huge_long_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.huge_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.special {
        if !room_database.special_rooms.is_empty() {
            chosen_presets.push_front(get_random_preset(&room_database.special_rooms).unwrap());
        }
    }
    chosen_presets
}

#[allow(clippy::redundant_else)]
/// Creates randomly positioned `Rect` that doesnt overlap any `Rect` in `occupied_positions`
///
/// configured with `DungeonSettings`
pub fn random_room_positon(
    filled_positions: &[Rect],
    room_size: Vec2,
    settings: &DungeonSettings,
) -> Rect {
    let mut rng = ThreadRng::default();
    let mut attempt_count = 0;
    let max_attempts = 100;

    let px_size_x = settings.size.x as f32 * TILE_SIZE;
    let px_size_y = settings.size.y as f32 * TILE_SIZE;

    let mut expanding_halfsize_x = px_size_x / 2.0;
    let mut expanding_halfsize_y = px_size_y / 2.0;

    loop {
        if attempt_count >= max_attempts {
            info!("expanding dungeon map range");
            attempt_count = 0;
            expanding_halfsize_x *= 1.1;
            expanding_halfsize_y *= 1.1;
        }

        let x = rng.gen_range(-expanding_halfsize_x..expanding_halfsize_x); // let start = Vec2::new(center.x - expanding_halfsize, center.y - expanding_halfsize);
        let y = rng.gen_range(-expanding_halfsize_y..expanding_halfsize_y); // let end = Vec2::new(center.x + expanding_halfsize, center.y + expanding_halfsize );

        let Vec2 {
            x: width,
            y: height,
        } = room_size;

        let valid_origins: Vec<Rect> = vec![
            Rect::from_center_size(Vec2 { x, y }, room_size),
            Rect::new(x, y, x - width, y - height),
            Rect::new(x, y, x + width, y - height),
            Rect::new(x, y, x - width, y + height),
            Rect::new(x, y, x + width, y + height),
        ];

        // test if test_rect has no intersections with currently spawned recs
        if filled_positions
            .iter()
            .all(|f| valid_origins.iter().any(|o| o.intersect(*f).is_empty()))
        {
            if let Some(rect) = valid_origins.iter().find(|new| {
                filled_positions
                    .iter()
                    .all(|filled| filled.intersect(**new).is_empty())
            }) {
                return rect.to_owned();
            };
        };
        warn!("bad position. restarting loop!");
        attempt_count += 1;
    }
}

use bevy::{
    ecs::{system::Res, world::Mut},
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
pub fn get_random_preset(presets: &[RoomPreset]) -> Option<RoomPreset> {
    let mut rng = ThreadRng::default();

    presets.iter().choose(&mut rng).cloned()
}

/// get random preset that matches `level` from `presets`
pub fn get_leveled_preset(presets: &[RoomPreset], level: &RoomLevel) -> Option<RoomPreset> {
    let mut rng = ThreadRng::default();

    presets
        .iter()
        .filter(|f| f.descriptor.level == *level)
        .choose(&mut rng)
        .cloned()
}

pub fn choose_center_presets(
    settings: &Mut<DungeonSettings>,
    room_database: &Res<DungeonRoomDatabase>,
) -> Vec<RoomPreset> {
    let mut chosen_presets: Vec<RoomPreset> = Vec::new();
    let room_cfg = &settings.room_amount;

    for _ in 0..room_cfg.small_short {
        if !room_database.small_short_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.small_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.small_long {
        if !room_database.small_long_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.small_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.medium_short {
        if !room_database.medium_short_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.medium_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.medium_long {
        if !room_database.medium_long_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.medium_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.large_short {
        if !room_database.large_short_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.large_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.large_long {
        if !room_database.large_long_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.large_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.huge_short {
        if !room_database.huge_short_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.huge_short_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.huge_long {
        if !room_database.huge_long_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.huge_long_rooms).unwrap());
        }
    }

    for _ in 0..room_cfg.special {
        if !room_database.special_rooms.is_empty() {
            chosen_presets.push(get_random_preset(&room_database.special_rooms).unwrap());
        }
    }
    chosen_presets
}

pub fn rects_too_display(filled_positions: &[Rect]) -> Vec<String> {
    let filled_pos_vec = filled_positions
        .iter()
        .map(|f| f.min)
        .map(|f| format!("Vec2{{{},{}}}", f.x, f.y))
        .collect::<Vec<String>>();
    filled_pos_vec
}

#[allow(clippy::redundant_else)]
/// Creates randomly positioned `Rect` that doesnt overlap any `Rect` in `occupied_positions`
///
/// configured with `DungeonSettings`
pub fn random_room_positon(
    filled_positions: &[Rect],
    size: Vec2,
    settings: &DungeonSettings,
) -> Rect {
    let mut expanding_range = settings.map_halfsize;
    let mut rng = ThreadRng::default();
    let mut attempt_count = 0;
    let max_attempts = 100;

    info!(
        "occupied positions: {:?}",
        rects_too_display(filled_positions)
    );
    info!("room size: {}", size);

    loop {
        if attempt_count >= max_attempts {
            info!("expanding testing range");
            attempt_count = 0;
            expanding_range *= 1.2;
        }

        let half = settings.tiles_between_rooms as f32 * TILE_SIZE / 2.0;
        let x = (rng.gen_range(-expanding_range..expanding_range) / TILE_SIZE).round() * TILE_SIZE;
        let y = (rng.gen_range(-expanding_range..expanding_range) / TILE_SIZE).round() * TILE_SIZE;
        let (width, height) = (size.x, size.y);
        info!("new room position: X: {}, Y: {}", x, y);

        let new_5 = Rect::from_center_size(Vec2 { x, y }, size);
        let new_4 = Rect::new(x, y, x - width, y - height); // top left origin
        let new_3 = Rect::new(x, y, x + width, y - height); // top right origin
        let new_2 = Rect::new(x, y, x - width, y + height); // bottom left origin
        let new_1 = Rect::new(x, y, x + width, y + height); // bottom right origin

        let mut valid_origins: Vec<Rect> = Vec::with_capacity(5);
        valid_origins.push(new_1.clone());
        valid_origins.push(new_2.clone());
        valid_origins.push(new_3.clone());
        valid_origins.push(new_4.clone());
        valid_origins.push(new_5.clone());

        // test if test_rect has no intersections with currently spawned recs
        if filled_positions
            .iter()
            .all(|f| valid_origins.iter().any(|o| o.intersect(*f).is_empty()))
        {
            if let Some(rect) = valid_origins.iter().find(|new| {
                let new_place = new.inset(half);
                filled_positions.iter().all(|filled| {
                    let filled = filled.inset(half);
                    filled.intersect(new_place).is_empty()
                })
            }) {
                return rect.to_owned();
            };
        };
        warn!("bad position. restarting loop!");
        attempt_count += 1;
        continue;
    }
}

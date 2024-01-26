use bevy_asepritesheet::prelude::{Spritesheet, AnimEndAction};

pub fn format_character_animations(sheet: &mut Spritesheet) {
    let handle_idle = sheet.get_anim_handle("idle");
    let handle_north = sheet.get_anim_handle("walk_south");
    let handle_south = sheet.get_anim_handle("walk_north");
    let handle_east = sheet.get_anim_handle("walk_east");

    if let Ok(anim_idle) = sheet.get_anim_mut(&handle_idle) {
        anim_idle.end_action = AnimEndAction::Loop;
    }
    if let Ok(anim_walk_north) = sheet.get_anim_mut(&handle_north) {
        anim_walk_north.end_action = AnimEndAction::Next(handle_idle);
    }
    if let Ok(anim_walk_south) = sheet.get_anim_mut(&handle_south) {
        anim_walk_south.end_action = AnimEndAction::Next(handle_idle);
    }
    if let Ok(anim_walk_east) = sheet.get_anim_mut(&handle_east) {
        anim_walk_east.end_action = AnimEndAction::Next(handle_idle);
    }
}
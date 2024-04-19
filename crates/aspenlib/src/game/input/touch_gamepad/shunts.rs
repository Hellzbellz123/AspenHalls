#![allow(clippy::type_complexity)]

use crate::game::input::{
    action_maps,
    touch_gamepad::{
        HealTag, InteractionTag, PauseTag, SwapWeaponTag, TouchStickBinding, ZoomInTag, ZoomOutTag,
    },
};
use bevy::prelude::*;
use bevy_touch_stick::TouchStick;
use leafwing_input_manager::prelude::ActionState;

/// press zoom out action if shunt is touched
pub fn touch_zoom_out(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<ZoomOutTag>)>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            actions.press(&action_maps::Gameplay::ZoomOut);
        }
    }
}

/// press zoom in action if shunt is touched
pub fn touch_zoom_in(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<ZoomInTag>)>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            actions.press(&action_maps::Gameplay::ZoomIn);
        }
    }
}

/// press pause action if shunt is touched
pub fn touch_pause_game(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<PauseTag>)>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            debug!("pause shunt triggered");
            actions.press(&action_maps::Gameplay::Pause);
        }
    }
}

/// press heal action if shunt is touched
pub fn touch_heal(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<HealTag>)>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            debug!("Heal shunt triggered");
            actions.press(&action_maps::Gameplay::Heal);
        }
    }
}

/// presses cycle weapon action if shunt is touched
pub fn touch_cycle_weapon(
    interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<SwapWeaponTag>),
    >,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            debug!("Swap shunt triggered");
            actions.press(&action_maps::Gameplay::CycleWeapon);
        }
    }
}

/// links UI interact button too `Gameplay::Interact` action
#[allow(clippy::type_complexity)]
pub fn touch_interaction_button(
    interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<InteractionTag>),
    >,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &interaction_query {
        if *interaction == Interaction::Pressed {
            debug!("Interact shunt triggered");
            actions.press(&action_maps::Gameplay::Interact);
        }
    }
}

/// triggers player sprint action if touch joystick is dragged past threshold
pub fn touch_trigger_sprint(
    sticks: Query<&TouchStick<TouchStickBinding>, Changed<TouchStick<TouchStickBinding>>>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    let stick = sticks
        .iter()
        .find(|f| f.id == TouchStickBinding::MoveTouchInput)
        .expect("always exists at this point");

    if stick.value.abs().max_element() >= 0.85 {
        // debug!("touch too press Sprint");
        actions.press(&action_maps::Gameplay::Sprint);
    }
}

/// triggers player shoot action if touch joystick is dragged past threshold
pub fn touch_trigger_attack(
    sticks: Query<&TouchStick<TouchStickBinding>, Changed<TouchStick<TouchStickBinding>>>,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    let stick = sticks
        .iter()
        .find(|f| f.id == TouchStickBinding::LookTouchInput)
        .expect("always exists at this point");

    if stick.value.abs().max_element() >= 0.85 {
        debug!("touch too press Shoot");
        actions.press(&action_maps::Gameplay::Attack);
    }
}

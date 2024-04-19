use bevy::prelude::*;
use bevy_touch_stick::{
    TouchStick, TouchStickGamepadMapping, TouchStickPlugin, TouchStickType, TouchStickUiBundle,
    TouchStickUiKnob, TouchStickUiOutline,
};
use leafwing_input_manager::prelude::ActionState;
use std::hash::Hash;

use crate::{
    game::{
        input::{
            action_maps,
            touch_gamepad::{InteractionButtonTag, TouchStickBinding},
            AspenInputSystemSet,
        },
        interface::InterfaceRoot,
        AppState,
    },
    loading::assets::{AspenInitHandles, AspenTouchHandles},
};

/// links UI interact button too `Gameplay::Interact` action
#[allow(clippy::type_complexity)]
pub fn touch_interaction_button(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            With<InteractionButtonTag>,
        ),
    >,
    mut actions: ResMut<ActionState<action_maps::Gameplay>>,
) {
    for (interaction, _) in &mut interaction_query {
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
pub fn touch_trigger_shoot(
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

use bevy::{math::vec2, prelude::*};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::{
    components::actors::{bundles::RigidBodyBundle, general::Player},
    game::GameStage,
    utilities::game::SystemLabels,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(setup_weapon_sockets)
                .with_system(rotate_player_weapon)
                .after(SystemLabels::Spawn),
        );
    }
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub tag: WeaponTag,
    pub weaponstats: WeaponStats,
    pub damagetype: DamageType,
    pub rigidbodybundle: RigidBodyBundle,
    pub spritesheetbundle: SpriteSheetBundle,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponTag {
    /// weapon slot weapon is currently in, None if not attached to player
    pub stored_weapon_slot: Option<i8>,
    /// weapons parent
    pub parent: Option<Entity>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct CurrentEquippedWeapon;

#[derive(Debug, Clone, Copy, Component)]
pub enum DamageType {
    KineticRanged,
    KineticMelee,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponStats {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponSocket {
    pub currently_equipped: Option<i8>,
    pub weapon_slots: i8,
    pub attached_weapon: Option<Entity>,
}

fn setup_weapon_sockets(
    mut cmds: Commands,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    mut weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform),
        (Without<Parent>, Without<Player>),
    >,
) {
    if !player_query.is_empty() {
        let (playerentity, mut weaponsocket_on_player, ptransform) = player_query.single_mut();
        if weaponsocket_on_player.attached_weapon.is_none() {
            for (weapon, mut weapontag, mut wtransform) in weapon_query.iter_mut() {
                let distance = (ptransform.translation - wtransform.translation)
                    .length()
                    .abs();
                if distance < 50.0 {
                    info!("parenting weapon: {:?} to player", weapon);
                    cmds.entity(playerentity).push_children(&[weapon]);
                    weapontag.parent = Some(playerentity);
                    weaponsocket_on_player.attached_weapon = Some(weapon);
                    wtransform.translation = Vec3::ZERO;
                    cmds.entity(weapon).insert(CurrentEquippedWeapon);
                } else {
                    info!("no weapon in range");
                };
            }
        }
    }
}

fn rotate_player_weapon(
    mouse: Res<MousePosWorld>,
    // mut player_query: Query<&mut Transform, With<Player>>,
    mut weapon_query: Query<
        (&mut WeaponTag, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentEquippedWeapon>, Without<Player>),
    >, // query weapon with a parent.
) {
    // mousepos is already worldspace
    if !weapon_query.is_empty() {
        for (wtag, wgtransform, mut wtransform) in weapon_query.iter_mut() {
            if wtag.parent.is_some() {
                let mousepos = vec2(mouse.x, mouse.y);
                let weaponpos: Vec2 = wtransform.translation.truncate();

                let aimdirection: Vec2 = (mousepos - weaponpos).normalize_or_zero();

                let aimangle = aimdirection.x.atan2(aimdirection.y);

                let global_wtrans = wgtransform.compute_transform().translation.truncate();
                let anglerad = global_wtrans.angle_between(mousepos);
                info!("{anglerad}");
                let rotation = Quat::from_rotation_z(aimangle);
                wtransform.rotation = rotation //rotation * PI; // = wtransform.rotation.lerp(rotation, 0.5);
            }
        }
    }
}

// check if if the weapon is supposed to be visible
fn weapon_visiblity_system(
    _cmds: Commands,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    mut weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform, &mut Visibility),
        (With<Parent>, Without<Player>),
    >, // query weapons
) {
    let (_pent, pweaponsocket, _ptransform) = player_query.single_mut();
    for (_wentity, wtag, _wtransform, mut wvisiblity) in weapon_query.iter_mut() {
        if wtag.stored_weapon_slot == pweaponsocket.currently_equipped {
            wvisiblity.is_visible = true;
        } else {
            wvisiblity.is_visible = false;
        }
    }
}

fn update_equipped_weapon(
    _cmds: Commands,
    _player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    _weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform),
        (Without<Parent>, Without<Player>),
    >,
) {
}

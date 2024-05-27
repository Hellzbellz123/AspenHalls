#![allow(clippy::unnecessary_struct_initialization)]

use bevy::{
    log::{error, warn},
    math::Vec2,
    prelude::{Name, Timer, TimerMode},
};
use bevy_ecs_ldtk::{
    ldtk::ReferenceToAnEntityInstance,
    prelude::{EntityInstance, LdtkFields},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider};

use crate::{
    game::game_world::components::{
        CharacterSpawner, HeroLocation, PlayerStartLocation, SpawnerTimer, SpawnerWave, Teleporter,
        TpTriggerEffect, WeaponSpawner,
    },
    loading::registry::RegistryIdentifier,
};

/// creates `PlayerStartLocation` from start location `EntityInstance`
pub const fn start_location_from_instance(instance: &EntityInstance) -> PlayerStartLocation {
    PlayerStartLocation {
        size: Vec2::new(instance.width as f32, instance.height as f32),
    }
}

/// creates a `Collider` from `EntityInstance` width/height
pub fn teleporter_collider_from_instance(instance: &EntityInstance) -> Collider {
    Collider::cuboid((instance.width / 4) as f32, (instance.height / 4) as f32)
}

/// creates `Teleporter` from `EntityInstance`
pub fn teleporter_from_instance(instance: &EntityInstance) -> Teleporter {
    let tp_type = decipher_teleport_type(instance).unwrap_or_else(|| {
        warn!("couldnt get teleporter action");
        let default = ReferenceToAnEntityInstance::default();
        TpTriggerEffect::Local(default)
    });

    Teleporter {
        active: true,
        effect: tp_type,
    }
}

/// creates `Name` from `EntityInstance.identifier`
pub fn name_from_instance(instance: &EntityInstance) -> Name {
    Name::new(instance.identifier.clone())
}

/// creates Spawner from spawner `EntityInstance`
pub fn spawner_wave_from_instance(entity_instance: &EntityInstance) -> SpawnerWave {
    let identifiers_too_spawn = get_spawn_identifiers(entity_instance);

    SpawnerWave {
        too_spawn: identifiers_too_spawn,
        spawned_entities: Vec::new(),
    }
}

/// creates Spawner from spawner `EntityInstance`
pub fn character_spawner_from_instance(entity_instance: &EntityInstance) -> CharacterSpawner {
    let spawn_wave_refs = entity_instance
        .get_maybe_entity_refs_field("CharacterWaves")
        .expect("spawners should have a CharacterWaves field")
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<ReferenceToAnEntityInstance>>();

    let &should_spawn_infinite = entity_instance
        .get_bool_field("RandomWave")
        .expect("spawner should have 'RandomWave' field");

    let got_max_ents = entity_instance
        .get_maybe_int_field("MaxEnemies")
        .expect("Spawner should ALWAYS have MaxEnemies field")
        // TODO: else size of max wave
        .unwrap_or(5);

    CharacterSpawner {
        //TODO: WAVE entity references
        waves: spawn_wave_refs,
        spawn_radius: entity_instance.width as f32,
        max_spawned: got_max_ents,
        spawned_characters: Vec::new(),
        random_wave: should_spawn_infinite,
    }
}

/// creates Spawner from spawner `EntityInstance`
pub fn weapon_spawner_from_instance(entity_instance: &EntityInstance) -> WeaponSpawner {
    let wanted_weapons = get_spawn_identifiers(entity_instance);

    let &interacted_only = entity_instance
        .get_bool_field("Interacted")
        .expect("weapon spawner should ALWAYS have 'Interacted' field");

    WeaponSpawner {
        wanted_weapons,
        interacted_only,
        used: false,
        debug: false,
    }
}

/// creates a hero location from an `EntityInstance`
pub fn hero_location_from_instance(entity_instance: &EntityInstance) -> HeroLocation {
    let hero_id = entity_instance
        .get_maybe_string_field("SpawnIdentifier")
        .expect("hero locations should have 'SpawnIdentifier' field")
        .clone()
        .map(|f| RegistryIdentifier(f));

    let &in_dungeon_hireable = entity_instance
        .get_bool_field("InDungeonHireable")
        .expect("hero locations should have 'InDungeonHireable' field");

    HeroLocation {
        hero_id,
        in_dungeon_hireable,
    }
}

/// creates a timer from spawner `EntityInstance`
pub fn spawn_timer_from_instance(entity_instance: &EntityInstance) -> SpawnerTimer {
    let ldtk_ent_duration = entity_instance
        .get_maybe_float_field("WaitTime")
        .expect("spawners should have a 'WaitTime' field")
        .unwrap_or(5.0);

    SpawnerTimer(Timer::from_seconds(ldtk_ent_duration, TimerMode::Repeating))
}

/// creates `ActiveEvents` from `EntityInstance`
pub const fn events_from_instance(_: &EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

/// gets list of `RegistryIdentifiers` from ldtk entity instance
pub fn get_spawn_identifiers(instance: &EntityInstance) -> Vec<RegistryIdentifier> {
    let strings = instance
        .get_maybe_strings_field("SpawnIdentifiers")
        .expect("Boss Area should ALWAYS have a DungeonBosses field")
        .iter()
        .filter_map(std::clone::Clone::clone)
        .map(RegistryIdentifier::from)
        .collect::<Vec<RegistryIdentifier>>();
    strings
}

/// creates `TpTriggerEffect` from data defined on teleporter `EntityInstance`
fn decipher_teleport_type(instance: &EntityInstance) -> Option<TpTriggerEffect> {
    let Ok(tp_type) = instance.get_enum_field("TeleporterType") else {
        return None;
    };

    match tp_type.as_str() {
        "Event" => instance
            .get_string_field("TeleporterAction")
            .map_or(None, |action| Some(TpTriggerEffect::Event(action.clone()))),
        "Local" => instance
            .get_entity_ref_field("TeleportLocalRef")
            .map_or(None, |local| Some(TpTriggerEffect::Local(local.clone()))),
        "Global" => {
            let Ok(val) = instance
                .get_maybe_floats_field("TeleportGlobalRef")
                .inspect_err(|e| error!("error getting TPType::Global from spawner instance: {e}"))
            else {
                return None;
            };
            let vec = vec2_from_float_array(val).expect("msg");
            Some(TpTriggerEffect::Global(vec))
        }
        unknown => {
            error!(
                "encountered unknown TPType for spawner instance: {}",
                unknown
            );
            None
        }
    }
}

/// creates Vec2 from array of f32s
fn vec2_from_float_array(option_array: &[Option<f32>]) -> Option<Vec2> {
    match option_array.len().cmp(&2) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Less => {
            warn!("array has 1 element, cant make Vec2");
            return None;
        }
        std::cmp::Ordering::Greater => {
            warn!("array has more than 2 elements, cant make Vec2");
            return None;
        }
    }

    // Extract values from the array using pattern matching
    let x = option_array[0].expect("unreachable expect?");
    let y = option_array[1].expect("unreachable expect?");

    Some(Vec2 { x, y })
}

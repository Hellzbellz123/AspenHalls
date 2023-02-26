// use std::default;
use bevy::prelude::*;

#[derive(Component)]
pub enum TestTowerType {
    Starter {
        Power: f32,
        Radius: f32,
    },
    Bigger {
        Power: f32,
        Radius: f32,
    },
    Enbiggened {
        Power: f32,
        Radius: f32,
    },
}

impl Default for TestTowerType {
    fn default() -> Self { TestTowerType::Starter { Power: 0.5, Radius: 1. } }
}

fn create_tower(mut commands: Commands)
{
    commands.spawn(TestTowerType::default());
    commands.spawn(TestTowerType::Starter { Power: 1., Radius: 1. });
}
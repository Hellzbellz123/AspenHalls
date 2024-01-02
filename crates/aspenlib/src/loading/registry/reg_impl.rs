use crate::loading::registry::{ActorRegistry, RegistryIdentifier};
use std::fmt::Debug;

impl From<String> for RegistryIdentifier {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Debug for ActorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec_creep = &self
            .characters
            .creeps
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        let vec_heroes = &self
            .characters
            .heroes
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        let vec_items = &self
            .items
            .weapons
            .keys()
            .collect::<Vec<&RegistryIdentifier>>();
        f.debug_struct("ActorRegistry")
            .field("creeps", vec_creep)
            .field("heroes", vec_heroes)
            .field("weapons", vec_items)
            .finish()
    }
}

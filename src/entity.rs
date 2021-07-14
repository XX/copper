use bevy::prelude::Entity;
use dashmap::DashMap;
use lazy_static::lazy_static;

pub use self::fetch::*;

pub mod fetch;

lazy_static! {
    static ref ENTITY_STORAGE: DashMap<String, Entity> = DashMap::new();
}

pub fn store_entity(name: impl Into<String>, entity: Entity) {
    ENTITY_STORAGE.insert(name.into(), entity);
}

pub fn find_entity(name: impl AsRef<str>) -> Option<Entity> {
    ENTITY_STORAGE.get(name.as_ref()).map(|value| *value)
}

use std::collections::HashSet;

use bevy::prelude::{Bundle, Commands, Component, Entity, EventReader, PbrBundle, Query, With};

use crate::{PbrState, SpawnedNode, TypedNode};

#[derive(Copy, Clone, Component)]
pub struct FinalType;

#[derive(Bundle)]
pub struct Final {
    _type_marker: FinalType,
    #[bundle]
    pub pbr: PbrBundle,
    pub pbr_state: PbrState,
}

impl Final {
    pub fn new() -> Self {
        Self {
            _type_marker: FinalType,
            pbr: Default::default(),
            pbr_state: Default::default(),
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> SpawnedNode {
        let id = commands.spawn_bundle(self).id();
        SpawnedNode { id }
    }
}

impl Default for Final {
    fn default() -> Self {
        Self::new()
    }
}

impl TypedNode for Final {
    type Type = FinalType;
}

pub struct UpdateEvent(pub Entity);

#[derive(Default, Component)]
pub struct Finals(pub HashSet<Entity>);

pub fn final_update(mut events: EventReader<UpdateEvent>, mut query: Query<&mut PbrState, With<FinalType>>) {
    for UpdateEvent(entity_id) in events.iter() {
        match query.get_mut(*entity_id) {
            Ok(mut state) => *state = PbrState::NotCalculated,
            Err(err) => eprintln!("Final does not exist for id = {}: {:?}", entity_id.to_bits(), err),
        }
    }
}

use std::any::Any;

use bevy::prelude::{BuildChildren, Commands, Component, Entity};

pub use self::{material::*, r#box::*, r#final::*, selection_group::*};
use crate::{store_entity, ProcessObject};

pub mod r#box;
pub mod r#final;
pub mod material;
pub mod selection_group;

#[derive(Copy, Clone)]
pub struct SpawnedNode {
    pub id: Entity,
}

impl SpawnedNode {
    pub fn save(self, name: impl Into<String>) -> Self {
        store_entity(name, self.id);
        self
    }

    pub fn inputs(self, commands: &mut Commands, inputs: &[Entity]) -> Self {
        commands.entity(self.id).push_children(inputs);
        self
    }
}

pub trait CommonNode: Send + Sync + Any {
    fn process(&self, object: &mut ProcessObject);
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Component)]
pub struct Node(pub std::boxed::Box<dyn CommonNode>);

impl CommonNode for Node {
    fn process(&self, object: &mut ProcessObject) {
        self.0.process(object)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self.0.as_any_ref()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.0.as_any_mut()
    }
}

pub trait TypedNode {
    type Type: Send + Sync + 'static;
}

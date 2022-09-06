use std::any::Any;

use bevy::prelude::{Commands, Component, StandardMaterial};

use crate::{node::Finals, CommonNode, Node, ProcessObject, SpawnedNode, TypedNode};

#[derive(Copy, Clone, Component)]
pub struct MaterialType;

impl TypedNode for Material {
    type Type = MaterialType;
}

#[derive(Default)]
pub struct Material {
    pub material: StandardMaterial,
}

impl Material {
    pub fn new(material: impl Into<StandardMaterial>) -> Self {
        Self {
            material: material.into(),
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> SpawnedNode {
        let id = commands
            .spawn_bundle((MaterialType, Node(Box::new(self)), Finals::default()))
            .id();
        SpawnedNode { id }
    }
}

impl CommonNode for Material {
    fn process(&self, object: &mut ProcessObject) {
        object.materials.push(self.material.clone());
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

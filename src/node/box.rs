use std::any::Any;

use bevy::prelude::{shape, Commands, Component, Mesh, Transform, Vec3};

use crate::{node::Finals, CommonNode, Node, ProcessObject, SpawnedNode, TypedNode};

#[derive(Copy, Clone, Component)]
pub struct BoxType;

impl TypedNode for Box {
    type Type = BoxType;
}

pub struct Box {
    pub length: Vec3,
    pub transform: Transform,
}

impl Box {
    pub fn new(x_length: f32, y_length: f32, z_length: f32) -> Self {
        Self {
            length: Vec3::new(x_length, y_length, z_length),
            transform: Default::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> SpawnedNode {
        let id = commands
            .spawn_bundle((BoxType, Node(std::boxed::Box::new(self)), Finals::default()))
            .id();
        SpawnedNode { id }
    }
}

impl Default for Box {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl CommonNode for Box {
    fn process(&self, object: &mut ProcessObject) {
        object
            .meshes
            .push(Mesh::from(shape::Box::new(self.length.x, self.length.y, self.length.z)));
        object.transform = Some(
            object
                .transform
                .as_ref()
                .map(|exist_transform| exist_transform.mul_transform(self.transform))
                .unwrap_or(self.transform),
        );
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

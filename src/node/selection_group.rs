use std::any::Any;

use bevy::{
    prelude::{Commands, Component, Mesh, Vec3},
    render::mesh::VertexAttributeValues,
};
use smallvec::SmallVec;

use crate::{node::Finals, CommonNode, Node, ProcessObject, SpawnedNode, TypedNode};

#[derive(Copy, Clone, Component)]
pub struct SelectionGroupType;

impl TypedNode for SelectionGroup {
    type Type = SelectionGroupType;
}

pub struct Selection {
    pub mesh: usize,
    pub indices: Vec<u32>,
}

pub enum Selector {
    ByNormal(ByNormalSelector),
}

pub struct ByNormalSelector {
    pub direction: Vec3,
    pub spread_angle: f32,
}

impl ByNormalSelector {
    pub fn is_selected(&self, normal_x: f32, normal_y: f32, normal_z: f32) -> bool {
        let angle = ((self.direction.x * normal_x + self.direction.y * normal_y + self.direction.z * normal_z)
            / self.direction.length())
        .acos();
        angle <= self.spread_angle
    }
}

#[derive(Default)]
pub struct SelectionGroup {
    pub name: String,
    pub selectors: SmallVec<[Selector; 1]>,
}

impl SelectionGroup {
    pub fn new(name: impl Into<String>, selectors: impl Into<SmallVec<[Selector; 1]>>) -> Self {
        Self {
            name: name.into(),
            selectors: selectors.into(),
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> SpawnedNode {
        let id = commands
            .spawn_bundle((SelectionGroupType, Node(Box::new(self)), Finals::default()))
            .id();
        SpawnedNode { id }
    }
}

impl CommonNode for SelectionGroup {
    fn process(&self, object: &mut ProcessObject) {
        for (idx, mesh) in object.meshes.iter().enumerate() {
            let mut selection_indices = Vec::new();

            for selector in &self.selectors {
                match selector {
                    Selector::ByNormal(selector) => {
                        if let Some(VertexAttributeValues::Float32x3(normals)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
                        {
                            for (idx, normal) in normals.iter().enumerate() {
                                if selector.is_selected(normal[0], normal[1], normal[2]) {
                                    selection_indices.push(idx as u32);
                                }
                            }
                        }
                    },
                }
            }

            if !selection_indices.is_empty() {
                object.selections.entry(self.name.clone()).or_default().push(Selection {
                    mesh: idx,
                    indices: selection_indices,
                });
            }
        }
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

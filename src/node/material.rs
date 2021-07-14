use std::any::Any;

use bevy::prelude::{Commands, StandardMaterial};

use crate::{node::Finals, CommonNode, Node, ProcessObject, SpawnedNode, TypedNode};

#[derive(Copy, Clone)]
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
        object.material.push(StandardMaterial {
            base_color: self.material.base_color,
            base_color_texture: self.material.base_color_texture.clone(),
            roughness: self.material.roughness,
            metallic: self.material.metallic,
            metallic_roughness_texture: self.material.metallic_roughness_texture.clone(),
            reflectance: self.material.reflectance,
            normal_map: self.material.normal_map.clone(),
            double_sided: self.material.double_sided,
            occlusion_texture: self.material.occlusion_texture.clone(),
            emissive: self.material.emissive,
            emissive_texture: self.material.emissive_texture.clone(),
            unlit: self.material.unlit,
        });
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

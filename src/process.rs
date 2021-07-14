use bevy::prelude::{
    Assets, Children, Commands, Entity, GlobalTransform, Mesh, PbrBundle, Query, ResMut, StandardMaterial, Transform,
    With,
};

use crate::{
    node::{FinalType, Finals},
    CommonNode, Node,
};

#[derive(Debug, Copy, Clone)]
pub enum PbrState {
    NotCalculated,
    Calculated,
}

impl Default for PbrState {
    fn default() -> Self {
        Self::NotCalculated
    }
}

impl PbrState {
    pub fn is_calculated(&self) -> bool {
        matches!(self, Self::Calculated)
    }

    pub fn need_calculate(&self) -> bool {
        !self.is_calculated()
    }
}

#[derive(Default)]
pub struct ProcessObject {
    pub mesh: Vec<Mesh>,
    pub material: Vec<StandardMaterial>,
    pub transform: Option<Transform>,
    pub global_transform: Option<GlobalTransform>,
}

impl ProcessObject {
    pub fn into_pbr(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> PbrBundle {
        let ProcessObject {
            mesh,
            material,
            transform,
            global_transform,
        } = self;
        let mut pbr = PbrBundle::default();

        if let Some(mesh) = mesh.into_iter().next() {
            pbr.mesh = meshes.add(mesh);
        }

        if let Some(material) = material.into_iter().next() {
            pbr.material = materials.add(material);
        }

        if let Some(transform) = transform {
            pbr.transform = transform;
        }

        if let Some(global_transform) = global_transform {
            pbr.global_transform = global_transform;
        }
        pbr
    }
}

pub fn finalize(
    mut commands: Commands,
    final_query: Query<(Entity, &PbrState, &Children), With<FinalType>>,
    mut node_query: Query<(&Node, &mut Finals, Option<&Children>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (final_id, pbr_state, inputs) in final_query.iter() {
        if pbr_state.need_calculate() {
            let mut final_entity = commands.entity(final_id);
            let mut object = ProcessObject::default();

            for input_id in inputs.iter() {
                process_input(final_id, *input_id, &mut node_query, &mut object);
            }

            final_entity
                .insert_bundle(object.into_pbr(&mut meshes, &mut materials))
                .insert(PbrState::Calculated);
        }
    }
}

fn process_input(
    final_id: Entity,
    input_id: Entity,
    query: &mut Query<(&Node, &mut Finals, Option<&Children>)>,
    object: &mut ProcessObject,
) {
    let mut finals = query.get_component_mut::<Finals>(input_id).unwrap();
    if !finals.0.contains(&final_id) {
        finals.0.insert(final_id);
    }

    let inputs = query.get_component::<Children>(input_id);
    if let Ok(inputs) = inputs {
        for input_id in inputs.clone().iter() {
            process_input(final_id, *input_id, query, object);
        }
    }

    let node = query.get_component::<Node>(input_id).unwrap();
    node.process(object);
}

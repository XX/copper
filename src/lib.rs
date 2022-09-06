use bevy::prelude::{App, Plugin};

pub use self::{
    entity::*,
    node::{CommonNode, Node, SpawnedNode, TypedNode},
    process::*,
};

pub mod entity;
pub mod node;
pub mod process;

pub struct CopperPlugin;

impl Plugin for CopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<node::UpdateEvent>()
            .add_system(process::finalize)
            .add_system(node::final_update);
    }
}

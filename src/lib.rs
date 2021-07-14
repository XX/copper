use bevy::prelude::{AppBuilder, IntoSystem, Plugin};

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
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<node::UpdateEvent>()
            .add_system(process::finalize.system())
            .add_system(node::final_update.system());
    }
}

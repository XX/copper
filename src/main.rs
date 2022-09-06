use bevy::{
    prelude::{
        shape, App, Assets, Color, Commands, DefaultPlugins, EventWriter, Mesh, Msaa, PbrBundle, PointLightBundle,
        ResMut, StandardMaterial, Transform,
    },
    window::close_on_esc,
};
use copper::{
    node::{self, UpdateEvent},
    CopperPlugin, FetchMutQuery, FetchingNode,
};

use crate::{camera::PanOrbitCameraPlugin, diagnostic::DiagnosticPlugin};

mod camera;
mod diagnostic;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(CopperPlugin)
        .add_event::<UpdateEvent>()
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let box1 = node::Box::default()
        .with_transform(Transform::from_xyz(0.0, 0.5, 0.0))
        .spawn(&mut commands)
        .save("Box1");

    let mat1 = node::Material::new(Color::rgb(0.8, 0.7, 0.6))
        .spawn(&mut commands)
        .save("Mat1")
        .inputs(&mut commands, &[box1.id]);

    node::Final::new()
        .spawn(&mut commands)
        .save("Final1")
        .inputs(&mut commands, &[mat1.id]);
}

fn update(finals_updater: EventWriter<UpdateEvent>, mut query: FetchMutQuery<node::Box>) {
    let mut box1 = node::Box::find_mut("Box1", &mut query, finals_updater).unwrap();
    box1.transform.translation.z += 0.001;
}

pub enum PrimitiveType {
    Polygon,
    PolygonMesh,
    Mesh,
    Nurbs,
    Bezier,
    Points,
}

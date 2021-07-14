use bevy::{
    input::system::exit_on_esc_system,
    prelude::{
        shape, App, Assets, Color, Commands, DefaultPlugins, EventWriter, IntoSystem, LightBundle, Mesh, Msaa,
        PbrBundle, PerspectiveCameraBundle, ResMut, StandardMaterial, Transform, Vec3,
    },
};
use bevy_fly_camera::FlyCamera;
use copper::{
    node::{self, UpdateEvent},
    CopperPlugin, FetchMutQuery, FetchingNode,
};

use crate::{camera::PanOrbitCameraPlugin, diagnostic::DiagnosticPlugin};

mod camera;
mod diagnostic;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(CopperPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_camera.system())
        .add_system(exit_on_esc_system.system())
        .add_system(update.system())
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let _radius = translation.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera {
            sensitivity: 10.0,
            ..Default::default()
        });
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(LightBundle {
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
        .add_inputs(&mut commands, &[box1.id]);

    node::Final::new()
        .spawn(&mut commands)
        .save("Final1")
        .add_inputs(&mut commands, &[mat1.id]);
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

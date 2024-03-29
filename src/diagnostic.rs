use bevy::app::{App, Plugin};

pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
        // app.add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default());
        // app.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default());
        // app.add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<
        //     bevy::render::texture::Texture,
        // >::default());
    }
}

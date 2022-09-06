use bevy::prelude::{Input, KeyCode, Plugin, ResMut};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};

use crate::verlet::point::VerletPoint;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<VerletPoint>()
        .add_system(toggle_inspector);
    }
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Space) {
        window_params.enabled = !window_params.enabled
    }
}

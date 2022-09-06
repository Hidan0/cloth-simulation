use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;

use super::{
    point::update_points,
    stick::{draw_lines, update_verlet_sticks},
};

pub struct VerletPlugin;

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_points)
            .add_plugin(DebugLinesPlugin::default())
            .add_system(draw_lines)
            .add_system(update_verlet_sticks);
    }
}

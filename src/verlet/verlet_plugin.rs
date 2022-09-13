use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_prototype_debug_lines::DebugLinesPlugin;

use super::{
    point::{update_points, VerletPoint},
    stick::{draw_lines, update_verlet_sticks},
};

pub struct VerletPlugin;

pub const CUTTING_RADIUS: f32 = 3.;

impl Plugin for VerletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_points)
            .insert_resource(IsMouseCutting(false))
            .add_plugin(DebugLinesPlugin::default())
            .add_system(cloth_cutting)
            .add_system(draw_lines)
            .add_system(update_verlet_sticks);
    }
}

struct IsMouseCutting(bool);

fn cloth_cutting(
    q: Query<(Entity, &Transform), With<VerletPoint>>,
    mut cmds: Commands,
    mut is_mouse_cutting: ResMut<IsMouseCutting>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut cursormv_evr: EventReader<CursorMoved>,
    win: Res<Windows>,
) {
    for btn_ev in mousebtn_evr.iter() {
        match btn_ev.state {
            bevy::input::ButtonState::Pressed => {
                if btn_ev.button.eq(&MouseButton::Left) {
                    is_mouse_cutting.0 = true;
                }
            }
            bevy::input::ButtonState::Released => {
                if btn_ev.button.eq(&MouseButton::Left) {
                    is_mouse_cutting.0 = false;
                }
            }
        }
    }

    let wnd = win.get_primary().unwrap();
    for cursor_ev in cursormv_evr.iter() {
        if is_mouse_cutting.0 {
            let cursor_pos = Vec3::new(
                cursor_ev.position.x - (wnd.width() / 2.),
                cursor_ev.position.y - (wnd.height() / 2.),
                0.,
            );

            q.iter().for_each(|(e, t)| {
                if t.translation.distance(cursor_pos) < CUTTING_RADIUS {
                    cmds.entity(e).despawn();
                }
            });
        }
    }
}

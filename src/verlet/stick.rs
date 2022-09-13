use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use super::point::{LockedVerletPoint, VerletPoint};

#[derive(Component)]
pub struct VerletStick {
    pub from: Entity,
    pub to: Entity,
    pub lenght: f32,
}

pub fn update_verlet_sticks(
    sticks: Query<(Entity, &VerletStick)>,
    mut cmds: Commands,
    mut points: Query<(&mut Transform, Option<&LockedVerletPoint>), With<VerletPoint>>,
) {
    for (e, stick) in &sticks {
        let res = points.get_many_mut([stick.from, stick.to]);
        match res {
            Ok([(mut from_t, from_lock), (mut to_t, to_lock)]) => {
                let (is_locked_from, is_locked_to) = (from_lock.is_some(), to_lock.is_some());
                if is_locked_from && is_locked_to {
                    continue;
                }

                let diff = from_t.translation - to_t.translation;
                let distance = from_t.translation.distance(to_t.translation);

                if distance > 10. * stick.lenght {
                    cmds.entity(e).despawn();
                } else {
                    let diff_factor = (stick.lenght - distance) / distance;
                    let offset = diff * diff_factor;
                    if !is_locked_from {
                        from_t.translation += if is_locked_to {
                            offset
                        } else {
                            offset * Vec3::splat(0.5)
                        }
                    }

                    if !is_locked_to {
                        to_t.translation -= if is_locked_from {
                            offset
                        } else {
                            offset * Vec3::splat(0.5)
                        }
                    }
                }
            }
            Err(_) => cmds.entity(e).despawn(),
        }

        // let [(mut from_t, from_lock), (mut to_t, to_lock)] =
        // points.get_many_mut([stick.from, stick.to]).unwrap();
    }
}

pub fn draw_lines(
    mut lines: ResMut<DebugLines>,
    sticks: Query<&VerletStick>,
    points: Query<&Transform, With<VerletPoint>>,
) {
    for stick in &sticks {
        let res = points.get_many([stick.from, stick.to]);
        match res {
            Ok([from_t, to_t]) => lines.line(from_t.translation, to_t.translation, 0.),
            Err(_) => {}
        }
    }
}

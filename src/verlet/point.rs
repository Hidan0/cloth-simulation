use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const GRAVITY: Vec3 = Vec3::new(0., -400., 0.);
const DRAG: f32 = 0.001;

#[derive(Component, Inspectable, Clone)]
pub struct VerletPoint {
    pub(crate) prev_pos: Option<Vec3>,
    pub(crate) mass: f32,
}

#[derive(Component, Inspectable)]
pub struct LockedVerletPoint;

pub fn update_points(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut VerletPoint), Without<LockedVerletPoint>>,
) {
    for (mut t, mut vp) in &mut q {
        let prev_pos = vp.prev_pos.unwrap_or(t.translation);
        vp.prev_pos = Some(t.translation);

        let acceleration = GRAVITY / Vec3::splat(vp.mass);
        t.translation = t.translation
            + (t.translation - prev_pos) * Vec3::splat(1.0 - DRAG)
            + acceleration
                * Vec3::splat(time.delta_seconds() * time.delta_seconds())
                * Vec3::splat(1.0 - DRAG);
    }
}

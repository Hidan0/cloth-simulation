mod debug;
mod verlet;

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*, window::PresentMode};
use debug::DebugPlugin;
use verlet::{
    point::{LockedVerletPoint, VerletPoint},
    stick::VerletStick,
    verlet_plugin,
};

const CLEAR_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const RESOLUTION: f32 = 16. / 9.;

fn main() {
    let w_height = 720.;

    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: RESOLUTION * w_height,
            height: w_height,
            title: "Cloth simulation test".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(verlet_plugin::VerletPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_dot)
        .run();
}

fn spawn_camera(mut cmds: Commands) {
    let camera = Camera2dBundle::default();
    cmds.spawn_bundle(camera);
}

fn spawn_dot(mut cmds: Commands) {
    let start_pos = Vec3::new(-500., 300., 0.);
    let mass = 9.5;
    let spacing = 5.;
    let lenght = 5.;
    let width = 160;
    let height = 80;

    let mut pts: Vec<Entity> = vec![];

    for y in 0..=height {
        for x in 0..=width {
            let pt = cmds
                .spawn()
                .insert(VerletPoint {
                    prev_pos: None,
                    mass,
                })
                .insert_bundle(sprite_bundle(
                    Color::BLUE,
                    start_pos + Vec3::new(x as f32 * spacing, y as f32 * spacing * -1., 0.),
                ))
                .id();

            if x != 0 {
                cmds.spawn().insert(VerletStick {
                    from: pts.get(pts.len() - 1).unwrap().clone(),
                    to: pt,
                    lenght,
                });
            }

            if y != 0 {
                cmds.spawn().insert(VerletStick {
                    from: pts.get(x + (y - 1) * (width + 1)).unwrap().clone(),
                    to: pt,
                    lenght,
                });
            }

            if y == 0 && x % 4 == 0 {
                cmds.entity(pt)
                    .insert(LockedVerletPoint)
                    .remove_bundle::<SpriteBundle>()
                    .insert_bundle(sprite_bundle(
                        Color::RED,
                        start_pos + Vec3::new(x as f32 * spacing, y as f32 * spacing * -1., 0.),
                    ));
            }

            pts.push(pt);
        }
    }
}

fn sprite_bundle(color: Color, pos: Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::splat(10.)),
            ..Default::default()
        },
        transform: Transform::from_translation(pos),
        ..Default::default()
    }
}

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallaxation2d::prelude::*;

// This example demonstrates how to set up and use the `bevy_parallaxation2d`
// crate in a Bevy application, including camera movement and parallax layers.

const CAMERA_MOVE_SPEED: f32 = 5.0;
const CAMERA_HEIGHT: f32 = 180.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("29ADFF").unwrap()))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            ParallaxPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scaling_mode: ScalingMode::FixedVertical(CAMERA_HEIGHT),
                ..default()
            },
            ..default()
        })
        .insert(ParallaxCamera);

    commands.spawn_batch(vec![
        ParallaxLayer {
            image: "mountains_background.png",
            depth: 84.0.into(),
            ..default()
        },
        ParallaxLayer {
            image: "back_trees_background.png",
            depth: 70.0.into(),
            ..default()
        },
        ParallaxLayer {
            image: "trees_background.png",
            depth: 55.0.into(),
            ..default()
        },
        ParallaxLayer {
            image: "bushes_background.png",
            depth: 40.0.into(),
            ..default()
        },
    ]);
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("There should only be one Camera2d");

    let left = input.pressed(KeyCode::ArrowLeft) as u32 as f32;
    let right = input.pressed(KeyCode::ArrowRight) as u32 as f32;
    let up = input.pressed(KeyCode::ArrowUp) as u32 as f32;
    let down = input.pressed(KeyCode::ArrowDown) as u32 as f32;
    let move_direction = Vec2::new(right - left, up - down) * CAMERA_MOVE_SPEED;

    camera_transform.translation += move_direction.extend(0.0);
}

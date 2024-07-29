# `bevy_parallaxation2d`

Crate providing simple 2D parallax layers in Bevy.

## Features
* **`ParallaxPlugin`** - Plugin required for the parallax functionality.
* **`ParallaxCamera`** - Component for marking the parallax camera. **Only one camera can use parallax layers**.
* **`ParallaxLayer`** - Component for creating a parallax layer.
* **`ParallaxFlags`** - Bit flags for defining attributes of a parallax layer.

## Examples
This is a simple example of how to use the crate.
```rust, no_run
use bevy::prelude::*;

// Import `bevy_parallaxation2d`
use bevy_parallaxation2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ParallaxPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn parallax camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCamera);

    // Spawn parallax layers
    commands.spawn_batch(vec![
        ParallaxLayer {
            image: "main_background.png",
            depth: 80.0.into(),
            flags: ParallaxFlags::REPEAT_X_AXIS | ParallaxFlags::REPEAT_Y_AXIS,
            ..default()
        },
        ParallaxLayer {
            image: "foreground.png",
            depth: (-5.0).into(),
            ..default()
        },
    ]);
}
```

This crate features an example you can run with
```ps
cargo run --example mountains
```

## Compatibility
| bevy | bevy_parallaxation2d |
|------|----------------------|
| 0.13 | 0.1                  |

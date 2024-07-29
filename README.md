## `bevy_parallaxation2d`

<div align="center">
    
  [![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/wilzet/bevy_parallaxation2d)
  [![Crates.io](https://img.shields.io/crates/v/bevy_parallaxation2d.svg)](https://crates.io/crates/bevy_parallaxation2d)
  [![Downloads](https://img.shields.io/crates/d/bevy_parallaxation2d.svg)](https://crates.io/crates/bevy_parallaxation2d)
  [![CI](https://github.com/wilzet/bevy_parallaxation2d/workflows/CI/badge.svg)](https://github.com/wilzet/bevy_parallaxation2d/actions)
</div>

Crate providing simple 2D parallax layers in Bevy.

## Features
* **`ParallaxPlugin`** - Plugin required for the parallax functionality.
* **`ParallaxCamera`** - Component for marking the parallax camera.
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
        // Use parallax plugin
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

This repository features an example you can run with
```ps
cargo run --example mountains
```

## Compatibility
| bevy | bevy_parallaxation2d |
|------|----------------------|
| 0.13 | 0.1                  |

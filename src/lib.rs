//! ## `bevy_parallaxation2d`
//! Crate providing simple 2D parallax layers in Bevy.
//!
//! ## In this crate:
//! * **[`ParallaxPlugin`](crate::plugin::ParallaxPlugin)** - Plugin required for the parallax functionality.
//! * **[`ParallaxCamera`](crate::components::ParallaxCamera)** - Component for marking the parallax camera.
//! * **[`ParallaxLayer`](crate::components::ParallaxLayer)** - Component for creating a parallax layer.
//! * **[`ParallaxFlags`](crate::flags::ParallaxFlags)** - Bit flags for defining attributes of a parallax layer.
//!
//! ## Examples
//! ```no_run
//! use bevy::prelude::*;
//!
//! // Import the `bevy_parallaxation2d` crate
//! use bevy_parallaxation2d::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, ParallaxPlugin::default()))
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     // Spawn parallax camera
//!     commands
//!         .spawn(Camera2dBundle::default())
//!         .insert(ParallaxCamera);
//!
//!     // Spawn:
//!     // * Main background that repeats in both directions.
//!     // * Hills that will default to repeating horizontally.
//!     // * Foreground at the top of the screen.
//!     commands.spawn_batch(vec![
//!         ParallaxLayer {
//!             image: "main_background.png",
//!             depth: 80.0.into(),
//!             flags: ParallaxFlags::REPEAT_X_AXIS | ParallaxFlags::REPEAT_Y_AXIS,
//!             ..default()
//!         },
//!         ParallaxLayer {
//!             image: "hills_background.png",
//!             depth: 40.0.into(),
//!             ..default()
//!         },
//!         ParallaxLayer {
//!             image: "branches_foreground.png",
//!             depth: (-5.0).into(),
//!             flags: ParallaxFlags::REPEAT_X_AXIS | ParallaxFlags::OFFSET_CAMERA_TOP,
//!             ..default()
//!         },
//!     ]);
//! }
//! ```

mod commands;
mod components;
mod flags;
mod material;
mod plugin;
mod resources;
mod systems;

/// The `depth` module provides extra functions and options for creating and
/// managing [`Depth`](crate::depth::Depth) values used in parallax effects.
///
/// The recommend way to use [`Depth`](crate::depth::Depth) values is simply
/// by using `.into()` on an [`f32`] as in the example:
/// ```
/// # use bevy::prelude::default;
/// # use bevy_parallaxation2d::prelude::*;
/// let layer = ParallaxLayer {
///     image: "background.png",
///     depth: 1.0.into(),
///     ..default()
/// };
/// ```
pub mod depth;

/// The `prelude` module exports commonly used types to provide a convenient entry
/// point for users of the `bevy_parallaxation2d` crate. It includes plugins,
/// components, and bitflags necessary for implementing parallax effects.
pub mod prelude {
    pub use crate::{
        commands::ParallaxDespawnCommands,
        components::{ParallaxCamera, ParallaxLayer},
        flags::ParallaxFlags,
        plugin::ParallaxPlugin,
    };
}

/// Test the readme example
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDocTests;

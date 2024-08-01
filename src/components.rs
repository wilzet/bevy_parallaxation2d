use crate::{depth::Depth, flags::ParallaxFlags};
use bevy::prelude::{Color, Component, Vec2};

/// Marker component for the parallax camera.
///
/// **Only one camera can use parallax layers**.
///
/// ## Panics
/// The application will panic if multiple (or none) `ParallaxCamera` components are detected.
///
/// ## Examples
/// ```
/// # use bevy::prelude::{Commands, Camera2dBundle};
/// use bevy_parallaxation2d::prelude::*;
///
/// fn setup(mut commands: Commands) {
///     // Add the ParallaxCamera component to your camera entity to enable parallax effects.
///     commands.spawn(Camera2dBundle::default()).insert(ParallaxCamera);
/// }
/// ```
#[derive(Default, Component, Debug)]
pub struct ParallaxCamera;

/// Inserting this component initiates a layer in the parallax scrolling system.
///
/// ## Examples
/// ```
/// # use bevy::prelude::{default, Commands, Color, Vec2};
/// use bevy_parallaxation2d::prelude::*;
///
/// fn setup(mut commands: Commands) {
///     let layer = ParallaxLayer {
///         image: "background.png",
///         depth: 1.0.into(),
///         ..default()
///     };
///
///     commands.spawn(layer);
/// }
///
/// fn setup_extra(mut commands: Commands) {
///     let layers = vec![
///         ParallaxLayer {
///             image: "background.png",
///             depth: 5.0.into(),
///             ..default()
///         },
///         ParallaxLayer {
///             image: "foreground.png",
///             depth: (-2.0).into(),
///             flags: ParallaxFlags::REPEAT_Y_AXIS | ParallaxFlags::OFFSET_CAMERA_RIGHT,
///             ..default()
///         },
///     ];
///
///     commands.spawn_batch(layers);
/// }
/// ```
/// 
/// ## Note
/// It is not necessary to provide a `TransformBundle` to the parallax layer and if you do,
/// the initialisation process only takes into account the z-value as a depth offset without
/// affecting the depth factor of the parallax effect.
#[derive(Default, Component, Debug)]
pub struct ParallaxLayer {
    pub image: &'static str,
    /// Color tint of the parallax layer.
    pub color: Color,
    /// The depth of the parallax layer, affecting its scroll speed.
    pub depth: Depth,
    /// The initial offset of the parallax layer.
    ///
    /// The offset is used such that the layer will be centered in the camera view
    /// when the camera is at this position. The offset can be further tuned using
    /// `OFFSET_TO_CAMERA` in the [`ParallaxFlags`].
    pub offset: Vec2,
    pub flags: ParallaxFlags,
}

/// Internal data structure for parallax layer configuration.
#[derive(Component)]
pub(crate) struct ParallaxLayerData {
    pub depth: Depth,
    pub offset: Vec2,
    pub flags: ParallaxFlags,
}

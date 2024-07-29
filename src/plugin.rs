use crate::{material::*, resources::*, systems::*};
use bevy::{
    asset::load_internal_asset,
    prelude::{default, App, Handle, Plugin, PostUpdate, Shader, Update},
    sprite::Material2dPlugin,
};

/// A plugin for setting up and managing parallax layers in the application.
///
/// ## Examples
/// ```no_run
/// # use bevy::prelude::{App, DefaultPlugins, ImagePlugin, PluginGroup};
/// use bevy_parallaxation2d::prelude::*;
///
/// App::new()
///     .add_plugins((
///         DefaultPlugins.set(ImagePlugin::default_nearest()),
///         ParallaxPlugin::new(-5.0, 5.0)
///             .set_neutral_depth(2.0)
///             .set_scale(2.0),
///     ))
///     .run();
/// ```
#[derive(Default)]
pub struct ParallaxPlugin(ParallaxConfig);

pub(crate) const PARALLAX_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(15425869855826893231);

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        // Assert material is added
        load_internal_asset!(
            app,
            PARALLAX_SHADER_HANDLE,
            "parallax_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(Material2dPlugin::<ParallaxMaterial>::default());

        app.insert_resource(ParallaxContext::new(self.0))
            .init_resource::<ParallaxMesh>()
            .add_systems(
                Update,
                (
                    initial_load_parallax_layers,
                    process_new_parallax_layer_data,
                ),
            )
            .add_systems(PostUpdate, move_parallax_layers);
    }
}

impl ParallaxPlugin {
    /// Creates a new `ParallaxPlugin` with the specified near and far depths.
    ///
    /// The neutral depth gets set at the mid-point between the `near_depth` and `far_depth`.
    ///
    /// ## Panics
    /// Panics if `near_depth` is not less than `far_depth`.
    ///
    /// See also [`Depth`](crate::depth::Depth)
    #[inline]
    #[must_use]
    pub fn new(near_depth: f32, far_depth: f32) -> Self {
        if near_depth >= far_depth {
            panic!("Parallax near depth should be less than far depth.");
        }

        Self(ParallaxConfig {
            near_depth,
            neutral_depth: (near_depth + far_depth) / 2.0,
            far_depth,
            ..default()
        })
    }

    /// Sets the neutral depth for the parallax effect.
    ///
    /// At this depth there is no parallax effect for a layer, meaning the layer seem to be
    /// stationary with respect to the world space.
    #[inline]
    #[must_use]
    pub fn set_neutral_depth(mut self, neutral_depth: f32) -> Self {
        self.0.neutral_depth = neutral_depth;
        Self(self.0)
    }

    /// Sets the scale for the parallax effect.
    ///
    /// The parallax effect gets multiplied by this value.
    #[inline]
    #[must_use]
    pub fn set_scale(mut self, scale: f32) -> Self {
        self.0.scale = scale;
        Self(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_new() {
        let plugin = ParallaxPlugin::new(0.0, 1.0)
            .set_neutral_depth(-1.0)
            .set_scale(-5.0);

        assert_eq!(plugin.0.scale, -5.0);
        assert_eq!(plugin.0.near_depth, 0.0);
        assert_eq!(plugin.0.neutral_depth, -1.0);
        assert_eq!(plugin.0.far_depth, 1.0);
    }

    #[test]
    #[should_panic(expected = "Parallax near depth")]
    fn plugin_new_panic() {
        let _ = ParallaxPlugin::new(1.0, -1.0);
    }
}

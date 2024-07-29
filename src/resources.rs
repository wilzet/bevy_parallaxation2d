use bevy::{
    prelude::{Assets, FromWorld, Mesh, Rectangle, Resource, World},
    sprite::Mesh2dHandle,
};

/// Configuration for parallax effects.
#[derive(Clone, Copy)]
pub(crate) struct ParallaxConfig {
    pub scale: f32,
    pub near_depth: f32,
    /// The neutral depth plane, interpreted as the 0-plane.
    pub neutral_depth: f32,
    pub far_depth: f32,
}

impl ParallaxConfig {
    /// Converts a depth between parallax depth and world depth
    ///
    /// For the parallax depth, `neutral_depth` is interpreted as the 0-plane, a negative value
    /// is in front, and a positive value is behind. The camera depth is reversed from this so
    /// we translate it as -`depth`. We also want the depth relative to the 0-plane; resulting
    /// in `new_depth` = -`depth` + `neutral_depth`.
    #[inline]
    fn convert_depth(config: Self, depth: f32) -> f32 {
        config.neutral_depth - depth
    }
}

impl Default for ParallaxConfig {
    #[inline]
    fn default() -> Self {
        Self {
            scale: 1.0,
            near_depth: -10.0,
            neutral_depth: 0.0,
            far_depth: 100.0,
        }
    }
}

/// Context holding the parallax configuration.
#[derive(Resource)]
pub(crate) struct ParallaxContext(ParallaxConfig);

impl ParallaxContext {
    pub(crate) const DEPTH_FACTOR_MIN: f32 = 0.0;
    pub(crate) const DEPTH_FACTOR_MAX: f32 = 100.0;

    #[inline]
    #[must_use]
    pub fn new(mut config: ParallaxConfig) -> Self {
        // Adjust the near and far depths relative to the neutral depth if needed.
        if config.near_depth < config.far_depth {
            config.near_depth = ParallaxConfig::convert_depth(config, config.near_depth);
            config.far_depth = ParallaxConfig::convert_depth(config, config.far_depth);
        }

        Self(config)
    }

    /// Converts a given depth between parallax depth and world depth
    #[inline]
    #[must_use]
    pub fn convert_depth(&self, depth: f32) -> f32 {
        ParallaxConfig::convert_depth(self.0, depth)
    }

    #[inline]
    #[must_use]
    pub fn calculate_depth_factor(&self, world_depth: f32) -> f32 {
        let factor = if world_depth <= self.0.far_depth {
            Self::DEPTH_FACTOR_MIN
        } else if world_depth >= self.0.near_depth {
            Self::DEPTH_FACTOR_MAX
        } else {
            self.0.near_depth / (self.0.near_depth - world_depth)
        };

        factor * self.0.scale
    }
}

/// Mesh resource used for parallax layers.
#[derive(Resource)]
pub(crate) struct ParallaxMesh(Mesh2dHandle);

impl ParallaxMesh {
    #[inline]
    #[must_use]
    pub fn handle(&self) -> Mesh2dHandle {
        self.0.clone()
    }
}

impl FromWorld for ParallaxMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self(meshes.add(Rectangle::default()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resources_new_default_context() {
        let context = ParallaxContext::new(ParallaxConfig::default());
        assert_eq!(context.0.scale, 1.0);
        assert_eq!(context.0.near_depth, 10.0);
        assert_eq!(context.0.neutral_depth, 0.0);
        assert_eq!(context.0.far_depth, -100.0);
    }

    #[test]
    fn resources_new_context() {
        let context = ParallaxContext::new(ParallaxConfig {
            scale: -5.0,
            near_depth: 0.0,
            neutral_depth: -1.0,
            far_depth: 1.0,
        });

        assert_eq!(context.0.scale, -5.0);
        assert_eq!(context.0.near_depth, -1.0);
        assert_eq!(context.0.neutral_depth, -1.0);
        assert_eq!(context.0.far_depth, -2.0);
    }

    #[test]
    fn resources_calculate_depth_factor() {
        let context = ParallaxContext::new(ParallaxConfig {
            neutral_depth: 5.0,
            ..Default::default()
        });
        let near = context.calculate_depth_factor(15.0);
        let far = context.calculate_depth_factor(-95.0);
        let neutral = context.calculate_depth_factor(0.0);
        let twice = context.calculate_depth_factor(-15.0);
        let half = context.calculate_depth_factor(7.5);

        assert_eq!(near, ParallaxContext::DEPTH_FACTOR_MAX);
        assert_eq!(far, ParallaxContext::DEPTH_FACTOR_MIN);
        assert_eq!(neutral, 1.0);
        assert_eq!(twice, 0.5);
        assert_eq!(half, 2.0);
    }
}

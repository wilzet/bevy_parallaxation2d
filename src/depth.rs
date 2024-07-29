use crate::resources::ParallaxContext;
use std::cmp::Ordering;
use DepthType::*;

/// Represents different types of depth.
#[derive(Clone, Copy, Debug)]
enum DepthType {
    /// Indicates a user-space depth.
    Parallax(f32),
    /// The depth used in the parallax system.
    WorldWithFactor(f32, f32),
}

/// Depth of a parallax layer.
///
/// The depth is relative to the neutral depth defined in the [`ParallaxPlugin`](crate::plugin::ParallaxPlugin).
/// In parallax depth space; defining a depth value less than the neutral depth will render the layer in front,
/// while a depth value greater renders it behind the neutral depth.
///
/// To manually set a world-space depth and scroll speed factor see: [`from_world`](crate::depth::Depth::from_world).
///
/// ## Examples
/// ```
/// # use bevy_parallaxation2d::prelude::ParallaxLayer;
/// use bevy_parallaxation2d::depth::*;
///
/// let depth = 8.0.into();
/// # let _ = ParallaxLayer {
/// #     depth,
/// #     ..Default::default()
/// # };
///
/// let depth = Depth::from_parallax(5.0);
///
/// let depth = Depth::from_world(10.0, 2.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Depth(DepthType);

impl Depth {
    /// Creates a new `Depth` from a parallax depth value.
    #[inline]
    #[must_use]
    pub fn from_parallax(depth: f32) -> Self {
        Self(Parallax(depth))
    }

    /// Creates a new `Depth` from custom depth and factor values.
    ///
    /// The depth is relative to the neutral depth defined in the [`ParallaxPlugin`](crate::plugin::ParallaxPlugin).
    /// In world-space; the depth is the z-position of the parallax layer. Defining a depth
    /// value greater than the neutral depth will render the layer in front, while a depth
    /// value less renders it behind the neutral depth.
    ///
    /// The factor should be the ratio `neutral depth / depth` meaning the factor should be
    /// `1.0` at the neutral layer, `0.0` at the far depth, and `f32::MAX` or `f32::INFINITY`
    /// (or just a large value) at the near depth. There is nothing that stops you from using
    /// a negative factor which could create the effect of a rotating background.
    ///
    /// ## Important
    /// The factor does ***not*** get multiplied by the scaling factor defined in the plugin.
    #[inline]
    pub fn from_world(depth: f32, factor: f32) -> Self {
        Self(WorldWithFactor(depth, factor))
    }

    #[inline]
    #[must_use]
    pub(crate) fn depth(&self) -> f32 {
        match self.0 {
            Parallax(depth) => depth,
            WorldWithFactor(depth, _) => depth,
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn depth_factor(&self) -> Option<f32> {
        match self.0 {
            Parallax(_) => None,
            WorldWithFactor(_, factor) => Some(factor),
        }
    }

    /// Translates a parallax depth to world depth with a factor using the given context.
    #[inline]
    #[must_use]
    pub(crate) fn to_world_with_factor(self, context: &ParallaxContext) -> Self {
        match self.0 {
            Parallax(depth) => {
                let depth = context.convert_depth(depth) * context.scale();
                let factor = context.calculate_depth_factor(depth);
                Self(WorldWithFactor(depth, factor))
            }
            WorldWithFactor(_, _) => self,
        }
    }
}

impl Default for Depth {
    /// The default depth value is `0.0`
    #[inline]
    fn default() -> Self {
        Self(Parallax(0.0))
    }
}

impl From<f32> for Depth {
    #[inline]
    fn from(value: f32) -> Self {
        Self::from_parallax(value)
    }
}

impl PartialEq for Depth {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (Parallax(lhs), Parallax(rhs)) => lhs == rhs,
            (WorldWithFactor(lhs, _), WorldWithFactor(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0, other.0) {
            (Parallax(lhs), Parallax(rhs)) => lhs.partial_cmp(&rhs),
            (WorldWithFactor(lhs, _), WorldWithFactor(rhs, _)) => lhs.partial_cmp(&rhs),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ParallaxConfig;

    #[test]
    fn depth_to_world_with_factor() {
        let context = ParallaxContext::new(ParallaxConfig {
            scale: 2.0,
            neutral_depth: 5.0,
            ..Default::default()
        });

        let near = Depth::from_parallax(-10.0).to_world_with_factor(&context);
        let far = Depth::from_parallax(100.0).to_world_with_factor(&context);
        let neutral = Depth::from_parallax(5.0).to_world_with_factor(&context);
        let double = Depth::from_parallax(20.0).to_world_with_factor(&context);
        let third = Depth::default().to_world_with_factor(&context);
        let custom = Depth::from_world(-10.0, 1.0);

        assert_eq!(near.depth(), 15.0);
        assert_eq!(far.depth(), -95.0);
        assert_eq!(neutral.depth(), 0.0);
        assert_eq!(double.depth(), -15.0);
        assert_eq!(third.depth(), 5.0);
        assert_eq!(custom.depth(), -10.0);

        // Double factors from scale = 2.0
        assert_eq!(
            near.depth_factor(),
            Some(ParallaxContext::DEPTH_FACTOR_MAX * 2.0)
        );
        assert_eq!(
            far.depth_factor(),
            Some(ParallaxContext::DEPTH_FACTOR_MIN * 2.0)
        );
        assert_eq!(neutral.depth_factor(), Some(2.0));
        assert_eq!(double.depth_factor(), Some(1.0));
        assert_eq!(third.depth_factor(), Some(3.0));
        // But not custom world depth
        assert_eq!(custom.depth_factor(), Some(1.0));
    }

    #[test]
    fn depth_ordering() {
        let context = ParallaxContext::new(ParallaxConfig {
            scale: 2.0,
            neutral_depth: 5.0,
            ..Default::default()
        });

        let near_parallax: Depth = (-10.0).into();
        let far_parallax: Depth = 100.0.into();
        let neutral_parallax: Depth = 5.0.into();

        let near = near_parallax.to_world_with_factor(&context);
        let far = far_parallax.to_world_with_factor(&context);
        let neutral = neutral_parallax.to_world_with_factor(&context);

        // Not compatible
        assert_ne!(near, near_parallax);
        assert_ne!(neutral, far_parallax);
        assert!(!(near < near_parallax));
        assert!(!(far_parallax >= neutral));

        // Assert ordering for WorldWithFactor
        assert!(near == near);
        assert!(near != neutral);
        assert!(far <= far);
        assert!(neutral >= far);
        assert!(neutral < near);
        assert!(near > far);

        // Assert ordering for Parallax
        assert!(near_parallax == near_parallax);
        assert!(near_parallax != neutral_parallax);
        assert!(far_parallax >= far_parallax);
        assert!(neutral_parallax <= far_parallax);
        assert!(neutral_parallax > near_parallax);
        assert!(near_parallax < far_parallax);
    }

    #[test]
    fn depth_getters() {
        let context = ParallaxContext::new(ParallaxConfig {
            neutral_depth: 5.0,
            ..Default::default()
        });

        let depth: Depth = (-5.0).into();
        assert_eq!(depth.depth(), -5.0);
        assert_eq!(depth.depth_factor(), None);

        let depth = depth.to_world_with_factor(&context);
        assert_eq!(depth.depth(), 10.0);
        assert_eq!(depth.depth_factor(), Some(3.0));

        let depth = depth.to_world_with_factor(&context);
        assert_eq!(depth.depth(), 10.0);
        assert_eq!(depth.depth_factor(), Some(3.0));

        let depth = Depth::from_world(50.0, -10.0).to_world_with_factor(&context);
        assert_eq!(depth.depth(), 50.0);
        assert_eq!(depth.depth_factor(), Some(-10.0));
    }
}

use bitflags::bitflags;

bitflags! {
    /// Represents various parallax scrolling settings using bit flags.
    ///
    /// ## Features
    /// * Repeat - Stretches the parallax layer with correct tiling along the specified axis.
    /// * Lock - Locks the layer's translation on the specified axis.
    /// * Offset - A [`ParallaxLayer`](crate::components::ParallaxLayer) can specify an offset, flags can adjust it to the camera boundary.
    ///
    /// ## Examples
    /// ```
    /// use bevy_parallaxation2d::prelude::ParallaxFlags;
    ///
    /// let flags = ParallaxFlags::default();
    /// assert_eq!(flags, ParallaxFlags::DEFAULT);
    ///
    /// let flags = ParallaxFlags::LOCKED_Y_AXIS | ParallaxFlags::OFFSET_CAMERA_LEFT;
    /// assert!(flags.intersects(ParallaxFlags::OFFSET_CAMERA_TOP));
    /// assert!(flags.contains(ParallaxFlags::NONE | ParallaxFlags::HORIZONTAL_OFFSET));
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ParallaxFlags: u8 {
        /// No set bit flags.
        const NONE = 0;
        /// Repeats the parallax effect along the X axis.
        const REPEAT_X_AXIS = 1;
        /// Repeats the parallax effect along the Y axis.
        const REPEAT_Y_AXIS = 2;
        /// Locks the parallax effect along the X axis.
        const LOCKED_X_AXIS = 4;
        /// Locks the parallax effect along the Y axis.
        const LOCKED_Y_AXIS = 8;
        /// Offsets the parallax effect relative to the camera position.
        const OFFSET_TO_CAMERA = 16;
        /// Specifies a horizontal offset (not set for vertical, set for horizontal).
        const HORIZONTAL_OFFSET = 32; // 0 => VERTICAL, 1 => HORIZONTAL
        /// Specifies a positive offset (not set for left/bottom, set for right/top).
        const POSITIVE_OFFSET = 64; // 0 => LEFT/BOTTOM, 1 => RIGHT/TOP
        /// Offsets the parallax effect to the camera's left.
        const OFFSET_CAMERA_LEFT = ParallaxFlags::OFFSET_TO_CAMERA.bits() | ParallaxFlags::HORIZONTAL_OFFSET.bits();
        /// Offsets the parallax effect to the camera's right.
        const OFFSET_CAMERA_RIGHT = ParallaxFlags::OFFSET_CAMERA_LEFT.bits() | ParallaxFlags::POSITIVE_OFFSET.bits();
        /// Offsets the parallax effect to the camera's bottom.
        const OFFSET_CAMERA_BOTTOM = ParallaxFlags::OFFSET_TO_CAMERA.bits();
        /// Offsets the parallax effect to the camera's top.
        const OFFSET_CAMERA_TOP = ParallaxFlags::OFFSET_CAMERA_BOTTOM.bits() | ParallaxFlags::POSITIVE_OFFSET.bits();
        /// Default parallax settings with repeat along X axis and offset to the camera's bottom.
        const DEFAULT = ParallaxFlags::REPEAT_X_AXIS.bits() | ParallaxFlags::OFFSET_CAMERA_BOTTOM.bits();
    }
}

impl Default for ParallaxFlags {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

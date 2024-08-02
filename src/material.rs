use crate::plugin::PARALLAX_SHADER_HANDLE;
use bevy::{
    prelude::{Asset, Color, Handle, Image, TypePath, Vec2},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

/// Material for parallax rendering.
#[derive(AsBindGroup, Clone, Asset, TypePath)]
pub(crate) struct ParallaxMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    depth: Vec2,
    #[uniform(0)]
    offset: Vec2,
    #[uniform(0)]
    repeat_scale: Vec2,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl ParallaxMaterial {
    #[inline]
    #[must_use]
    pub fn new(image: Handle<Image>, color: Color) -> Self {
        Self {
            color,
            depth: Vec2::ZERO,
            offset: Vec2::ZERO,
            repeat_scale: Vec2::ONE,
            texture: image,
        }
    }

    #[inline]
    #[must_use]
    pub fn image_handle(&self) -> Handle<Image> {
        self.texture.clone()
    }

    #[inline]
    pub fn set_image_handle(&mut self, image: Handle<Image>) -> &mut Self {
        self.texture = image;
        self
    }

    #[inline]
    pub fn set_repeat_scale(&mut self, repeat_scale: Vec2) -> &mut Self {
        self.repeat_scale = repeat_scale;
        self
    }

    #[inline]
    pub fn set_depth(&mut self, depth: Vec2) -> &mut Self {
        self.depth = depth;
        self
    }

    #[inline]
    pub fn set_offset(&mut self, offset: Vec2) -> &mut Self {
        self.offset = offset;
        self
    }
}

impl Material2d for ParallaxMaterial {
    #[inline]
    fn fragment_shader() -> ShaderRef {
        PARALLAX_SHADER_HANDLE.into()
    }
}

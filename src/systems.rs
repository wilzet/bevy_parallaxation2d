use crate::{
    components::*,
    depth::Depth,
    flags::ParallaxFlags,
    material::ParallaxMaterial,
    resources::{ParallaxContext, ParallaxMesh},
};
use bevy::{
    prelude::{
        default, Added, AssetServer, Assets, Commands, Entity, Handle, Image,
        OrthographicProjection, Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    sprite::MaterialMesh2dBundle,
};

pub(crate) fn initial_load_parallax_layers(
    mut commands: Commands,
    mut materials: ResMut<Assets<ParallaxMaterial>>,
    new_parallax_layers_query: Query<(Entity, &ParallaxLayer), Added<ParallaxLayer>>,
    asset_server: Res<AssetServer>,
    parallax_mesh: Res<ParallaxMesh>,
) {
    for (entity, parallax) in new_parallax_layers_query.iter() {
        commands
            .entity(entity)
            .insert((
                ParallaxLayerData {
                    depth: parallax.depth,
                    offset: parallax.offset,
                    flags: parallax.flags,
                },
                MaterialMesh2dBundle {
                    mesh: parallax_mesh.handle(),
                    material: materials.add(ParallaxMaterial::new(
                        asset_server.load(parallax.image),
                        parallax.color,
                    )),
                    ..default()
                },
            ))
            .remove::<ParallaxLayer>();

        #[cfg(debug_assertions)]
        {
            use bevy::prelude::Name;
            commands.entity(entity).insert(Name::new("Parallax Layer"));
        }
    }
}

pub(crate) fn process_new_parallax_layer_data(
    mut new_parallax_layers_query: Query<
        (
            &mut Transform,
            &mut ParallaxLayerData,
            &Handle<ParallaxMaterial>,
        ),
        Added<ParallaxLayerData>,
    >,
    mut materials: ResMut<Assets<ParallaxMaterial>>,
    mut images: ResMut<Assets<Image>>,
    camera_query: Query<&OrthographicProjection, With<ParallaxCamera>>,
    parallax_context: Res<ParallaxContext>,
) {
    let camera_projection = camera_query
        .get_single()
        .expect("There should be exactly one parallax camera");

    let camera_size = camera_projection.area.half_size() * 2.0;

    for (mut transform, mut parallax, material) in new_parallax_layers_query.iter_mut() {
        let material = materials
            .get_mut(material)
            .expect("Parallax material should be loaded");

        let image = images
            .get_mut(material.image_handle())
            .expect("Image should be loaded");

        let image_dimensions = image.size_f32();

        // Set and get the world depth, unwrap safe since we just before set the factor
        parallax.depth = parallax.depth.to_world_with_factor(&parallax_context);
        let depth_factor = parallax.depth.depth_factor().unwrap();
        let mut depth_factor = Vec2::splat(depth_factor);

        // Configure texture repeat modes and dimensions
        let (tile_mode_x, scaled_image_width) =
            match parallax.flags.contains(ParallaxFlags::REPEAT_X_AXIS) {
                true => (ImageAddressMode::Repeat, camera_size.x),
                false => {
                    depth_factor.x = 0.0;
                    (ImageAddressMode::ClampToEdge, image_dimensions.x)
                }
            };
        let (tile_mode_y, scaled_image_height) =
            match parallax.flags.contains(ParallaxFlags::REPEAT_Y_AXIS) {
                true => (ImageAddressMode::Repeat, camera_size.y),
                false => {
                    depth_factor.y = 0.0;
                    (ImageAddressMode::ClampToEdge, image_dimensions.y)
                }
            };
        let scaled_image_dimensions = Vec2::new(scaled_image_width, scaled_image_height);

        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: tile_mode_x,
            address_mode_v: tile_mode_y,
            ..default()
        });

        // Compute camera offset
        let camera_translation =
            translation_with_depth_and_flags(parallax.offset, parallax.depth, parallax.flags);
        parallax.offset -= camera_translation;

        // Adjust offset relative to camera.
        if parallax.flags.contains(ParallaxFlags::OFFSET_TO_CAMERA) {
            // If the camera is centered at precisely the parallax layers spawn position, the
            // offset should be adjusted to the camera by this much: ...
            let offset = if parallax.flags.contains(ParallaxFlags::HORIZONTAL_OFFSET) {
                Vec2::X * (camera_size.x - scaled_image_dimensions.x) / 2.0
            } else {
                Vec2::Y * (camera_size.y - scaled_image_dimensions.y) / 2.0
            };

            if parallax.flags.contains(ParallaxFlags::POSITIVE_OFFSET) {
                parallax.offset += offset;
            } else {
                parallax.offset -= offset;
            }
        }

        transform.translation = parallax.offset.extend(parallax.depth.depth());
        transform.scale = scaled_image_dimensions.extend(1.0);

        material
            .set_repeat_scale(scaled_image_dimensions / image_dimensions)
            .set_depth(depth_factor / scaled_image_dimensions)
            .set_offset(parallax.offset);
    }
}

pub(crate) fn move_parallax_layers(
    mut parallax_layer_query: Query<(&mut Transform, &ParallaxLayerData), Without<ParallaxCamera>>,
    camera_query: Query<&Transform, With<ParallaxCamera>>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("There should be exactly one parallax camera");

    let camera_translation = camera_transform.translation.truncate();
    for (mut transform, parallax) in parallax_layer_query.iter_mut() {
        let translation =
            translation_with_depth_and_flags(camera_translation, parallax.depth, parallax.flags);

        transform.translation = (translation + parallax.offset).extend(parallax.depth.depth());
    }
}

#[inline]
#[must_use]
fn translation_with_depth_and_flags(
    mut translation: Vec2,
    depth: Depth,
    flags: ParallaxFlags,
) -> Vec2 {
    let Some(depth_factor) = depth.depth_factor() else {
        // No depth factor is treated as 0.0
        return Vec2::ZERO;
    };

    if flags.contains(ParallaxFlags::LOCKED_X_AXIS) {
        translation.x = 0.0;
    } else if !flags.contains(ParallaxFlags::REPEAT_X_AXIS) {
        translation.x -= translation.x * depth_factor;
    }

    if flags.contains(ParallaxFlags::LOCKED_Y_AXIS) {
        translation.y = 0.0;
    } else if !flags.contains(ParallaxFlags::REPEAT_Y_AXIS) {
        translation.y -= translation.y * depth_factor;
    }

    translation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ParallaxConfig;

    #[test]
    fn systems_translation() {
        let context = ParallaxContext::new(ParallaxConfig::default());
        let depth = Depth::from_parallax(10.0);
        let translation_0 =
            translation_with_depth_and_flags(Vec2::ZERO, depth, ParallaxFlags::NONE);

        let depth = depth.to_world_with_factor(&context);

        let translation_1 = translation_with_depth_and_flags(Vec2::ONE, depth, ParallaxFlags::NONE);
        let translation_2 =
            translation_with_depth_and_flags(Vec2::ONE, depth, ParallaxFlags::LOCKED_X_AXIS);
        let translation_3 = translation_with_depth_and_flags(
            Vec2::ONE,
            depth,
            ParallaxFlags::LOCKED_X_AXIS | ParallaxFlags::LOCKED_Y_AXIS,
        );
        let translation_4 =
            translation_with_depth_and_flags(Vec2::ONE, depth, ParallaxFlags::REPEAT_X_AXIS);

        assert_eq!(depth.depth_factor(), Some(0.5));
        assert_eq!(translation_0, Vec2::ZERO);
        assert_eq!(translation_1, Vec2::splat(0.5));
        assert_eq!(translation_2, Vec2::new(0.0, 0.5));
        assert_eq!(translation_3, Vec2::ZERO);
        assert_eq!(translation_4, Vec2::new(1.0, 0.5));
    }
}

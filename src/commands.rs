use crate::{components::ParallaxLayerData, depth::Depth};
use bevy::prelude::{Commands, Entity, World};

pub trait ParallaxDespawnCommands {
    /// Despawn the front most parallax layer
    fn despawn_front_layer(&mut self);

    /// Despawn the back most parallax layer
    fn despawn_back_layer(&mut self);
}

impl<'w, 's> ParallaxDespawnCommands for Commands<'w, 's> {
    fn despawn_front_layer(&mut self) {
        self.add(|world: &mut World| {
            let mut parallax_layers_query = world.query::<(Entity, &ParallaxLayerData)>();
            let (mut front_layer_entity, mut min_depth) = (None, Depth::from_world(f32::MIN, 1.0));
            for (entity, parallax) in parallax_layers_query.iter(world) {
                if parallax.depth > min_depth {
                    (front_layer_entity, min_depth) = (Some(entity), parallax.depth);
                }
            }

            if let Some(front_entity) = front_layer_entity {
                world.despawn(front_entity);
            }
        });
    }

    fn despawn_back_layer(&mut self) {
        self.add(|world: &mut World| {
            let mut parallax_layers_query = world.query::<(Entity, &ParallaxLayerData)>();
            let (mut back_layer_entity, mut max_depth) = (None, Depth::from_world(f32::MAX, 1.0));
            for (entity, parallax) in parallax_layers_query.iter(world) {
                if parallax.depth < max_depth {
                    (back_layer_entity, max_depth) = (Some(entity), parallax.depth);
                }
            }

            if let Some(back_entity) = back_layer_entity {
                world.despawn(back_entity);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::ParallaxFlags;
    use bevy::{ecs::system::CommandQueue, prelude::Vec2};

    #[test]
    fn commands_despawn_front_layer() {
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();

        // Spawn
        Commands::new(&mut command_queue, &world).spawn_batch(vec![
            ParallaxLayerData {
                depth: Depth::from_world(10.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
            ParallaxLayerData {
                depth: Depth::from_world(-12.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
        ]);
        command_queue.apply(&mut world);
        assert_eq!(world.entities().len(), 2);

        // Despawn front
        Commands::new(&mut command_queue, &world).despawn_front_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result, vec![-12.0]);
    }

    #[test]
    fn commands_despawn_back_layer() {
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();

        // Spawn
        Commands::new(&mut command_queue, &world).spawn_batch(vec![
            ParallaxLayerData {
                depth: Depth::from_world(10.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
            ParallaxLayerData {
                depth: Depth::from_world(-12.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
        ]);
        command_queue.apply(&mut world);
        assert_eq!(world.entities().len(), 2);

        // Despawn back
        Commands::new(&mut command_queue, &world).despawn_back_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result, vec![10.0]);
    }

    #[test]
    fn commands_despawn_alternate_layer() {
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();

        // Spawn
        Commands::new(&mut command_queue, &world).spawn_batch(vec![
            ParallaxLayerData {
                depth: Depth::from_world(10.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
            ParallaxLayerData {
                depth: Depth::from_world(-12.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
            ParallaxLayerData {
                depth: Depth::from_world(0.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
            ParallaxLayerData {
                depth: Depth::from_world(4.0, 1.0),
                offset: Vec2::ZERO,
                flags: ParallaxFlags::NONE,
            },
        ]);
        command_queue.apply(&mut world);
        assert_eq!(world.entities().len(), 4);

        // Despawn front
        Commands::new(&mut command_queue, &world).despawn_front_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 3);
        assert_eq!(world.entities().len(), 3);
        for depth in result.iter() {
            assert_eq!([0.0, -12.0, 4.0].contains(depth), true);
        }

        // Despawn back
        Commands::new(&mut command_queue, &world).despawn_back_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 2);
        assert_eq!(world.entities().len(), 2);
        for depth in result.iter() {
            assert_eq!([0.0, 4.0].contains(depth), true);
        }

        // Despawn back
        Commands::new(&mut command_queue, &world).despawn_back_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 1);
        assert_eq!(world.entities().len(), 1);
        for depth in result.iter() {
            assert_eq!([4.0].contains(depth), true);
        }

        // Despawn front
        Commands::new(&mut command_queue, &world).despawn_front_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 0);
        assert_eq!(world.entities().len(), 0);

        // Despawn nothing
        Commands::new(&mut command_queue, &world).despawn_back_layer();
        command_queue.apply(&mut world);

        let result = world
            .query::<&ParallaxLayerData>()
            .iter(&world)
            .map(|e| e.depth.depth())
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 0);
        assert_eq!(world.entities().len(), 0);
    }
}

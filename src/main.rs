mod ecs;
fn main() {}
#[cfg(test)]

mod test {
    use std::sync::{Arc, Mutex};

    use ecs::ecs::{
        component::IComponent, config::Signature, singleton::EcsSingleton, system::System,
    };
    use glam::Vec3;

    use super::*;

    #[derive(Debug)]
    struct Transform {
        position: Vec3,
    }

    impl IComponent for Transform {
        fn entity_destroyed(&mut self, _entity: ecs::ecs::config::EntityType) {
            todo!()
        }
    }

    #[derive(Debug)]
    struct RigidBody {
        force: Vec3,
    }
    struct Physics<'a, 'c> {
        singleton: &'c EcsSingleton,
        system: &'a Arc<Mutex<System>>,
    }

    impl<'a, 'c> Physics<'a, 'c> {
        pub fn new(singleton: &'c EcsSingleton, system: &'a Arc<Mutex<System>>) -> Self {
            Physics { singleton, system }
        }

        pub fn process(&mut self) {
            for sys in self.system.lock().iter() {
                for entity in sys.entities.iter() {
                    self.singleton
                        .get_component_mut::<Transform, _, ()>(*entity, |comp| {
                            comp.position.x += 1.0;
                        });

                    self.singleton
                        .get_component_mut::<RigidBody, _, ()>(*entity, |comp| {
                            comp.force.x += 1.0;
                        });
                }
            }
        }
    }

    impl IComponent for RigidBody {
        fn entity_destroyed(&mut self, _entity: ecs::ecs::config::EntityType) {
            todo!()
        }
    }

    #[test]
    pub fn test_register_entity_to_singleton() {
        let mut singleton = EcsSingleton::new();
        // created my components
        singleton.register_component::<Transform>();
        singleton.register_component::<RigidBody>();

        let entity = singleton.create_entity();

        let force = Vec3::new(1.0, 1.0, 1.0);
        singleton.add_component(entity, RigidBody { force });
        let position = Vec3::new(1.0, 0.0, 1.0);
        singleton.add_component(entity, Transform { position });
        let rigid_body = singleton.get_component::<RigidBody>(entity);

        let transform_component = singleton.get_component::<Transform>(entity);

        assert_eq!(transform_component.position, position);

        assert_eq!(rigid_body.force, force);
    }

    #[test]
    pub fn test_register_system() {
        let mut singleton = EcsSingleton::new();
        // created my components
        singleton.register_component::<Transform>();
        singleton.register_component::<RigidBody>();

        singleton.register_system::<Physics>();

        let mut signature: Signature = Default::default();
        signature.insert(singleton.get_component_type::<Transform>() as usize);
        signature.insert(singleton.get_component_type::<RigidBody>() as usize);
        singleton.set_system_signature::<Physics>(signature);

        let entity = singleton.create_entity();

        let force = Vec3::new(1.0, 1.0, 1.0);
        singleton.add_component(entity, RigidBody { force });
        let position = Vec3::new(1.0, 0.0, 1.0);
        singleton.add_component(entity, Transform { position });

        let transform_component = singleton.get_component::<Transform>(entity);
        let rigid_body = singleton.get_component::<RigidBody>(entity);

        assert_eq!(transform_component.position, position);

        assert_eq!(rigid_body.force, force);

        let gh = singleton.get_system::<Physics>().clone();
        let mut physics = Physics::new(&singleton, &gh);
        physics.process();

        let transform_component = singleton.get_component::<Transform>(entity);
        let rigid_body = singleton.get_component::<RigidBody>(entity);

        assert_eq!(transform_component.position, Vec3::new(2.0, 0.0, 1.0));

        assert_eq!(rigid_body.force, Vec3::new(2.0, 1.0, 1.0));
    }
}

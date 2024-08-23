use std::{
    any::{Any, TypeId},
    sync::{Arc, Mutex},
};

use crate::ecs::ecs::{
    component::IComponent,
    component_manager::ComponentManager,
    config::{EntityType, Signature},
    entity_manager::{EntityManager, EntityManagerResponse},
    system::System,
    system_manager::SystemManager,
};

#[derive(Debug)]
pub struct EcsSingleton {
    component_manager: Box<ComponentManager>,
    entity_manager: Box<EntityManager>,
    system_manager: Box<SystemManager>,
}
impl EcsSingleton {
    pub fn new() -> Self {
        Self {
            component_manager: Box::new(ComponentManager::new()),
            entity_manager: Box::new(EntityManager::create()),
            system_manager: Box::new(SystemManager::new()),
        }
    }
    pub fn create_entity(&mut self) -> u32 {
        self.entity_manager.create_entity()
    }

    pub fn destroy_entity(&mut self, entity: EntityType) {
        let _ = self.entity_manager.destroy_entity(entity);
        self.component_manager.entity_destroyed(entity);
        self.system_manager.entity_destroyed(entity);
    }

    pub fn register_component<T: Any>(&mut self) {
        self.component_manager.register_component::<T>();
    }

    pub fn get_component_mut<T: Any + IComponent, F, R>(&self, entity: EntityType, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        self.component_manager
            .get_component_mut::<T, F, R>(entity, f)
    }

    pub fn get_component<T: Any + IComponent>(&self, entity: EntityType) -> &T {
        self.component_manager.get_component::<T>(entity)
    }

    pub fn get_component_type<T: Any>(&mut self) -> u32 {
        let val = self.component_manager.get_component_type::<T>();
        *val
    }

    pub fn register_system<T: Any>(&mut self) -> TypeId {
        self.system_manager.register_system::<T>()
    }
    pub fn get_system<T: Any>(&mut self) -> &Arc<Mutex<System>> {
        self.system_manager.get_system::<T>()
    }
    pub fn set_system_signature<T: Any>(&mut self, signature: Signature) {
        self.system_manager.set_signatures::<T>(signature)
    }

    pub fn add_component<T: Any + IComponent>(&mut self, entity: EntityType, component: T) {
        self.component_manager.add_component::<T>(entity, component);
        let component_type = self.component_manager.get_component_type::<T>();

        let mut signature: Signature = Default::default();
        signature.insert(*component_type as usize);
        self.entity_manager.set_signature(entity, signature);

        let signature = self.entity_manager.get_signature(entity);
        self.system_manager
            .entity_signature_changed(entity, &signature);
    }

    pub fn remove_component<T: Any + IComponent>(&mut self, entity: EntityType) {
        self.component_manager.remove_component::<T>(entity);
        self.entity_manager.remove_signature(entity);
        let signature = self.entity_manager.get_signature(entity);

        self.system_manager
            .entity_signature_changed(entity, &signature)
    }
}

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::ecs::ecs::config::{EntityType, Signature};

use super::system::System;

#[derive(Debug)]
pub struct SystemManager {
    signatures: HashMap<TypeId, Signature>,
    systems: HashMap<TypeId, Arc<Mutex<System>>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn get_system<T: Any>(&self) -> &Arc<Mutex<System>> {
        let type_id = TypeId::of::<T>();
        if let Some(system) = self.systems.get(&type_id) {
            system
        } else {
            panic!("Cant retriver given system");
        }
    }
    pub fn register_system<T: Any>(&mut self) -> TypeId {
        let system_shared_value = Mutex::new(System::new());
        let arc_system = Arc::new(system_shared_value);
        let type_id = TypeId::of::<T>();

        if let None = self.systems.insert(type_id, arc_system) {
            return type_id;
        }
        panic!("Trying to add a duplicate system")
    }

    pub fn register_system_func<T: Any>(&mut self) -> TypeId {
        let system_shared_value = Mutex::new(System::new());
        let arc_system = Arc::new(system_shared_value);
        let type_id = TypeId::of::<T>();

        if let None = self.systems.insert(type_id, arc_system) {
            return type_id;
        }
        panic!("Trying to add a duplicate system")
    }

    pub fn set_signatures<T: Any>(&mut self, signature: Signature) {
        self.signatures.insert(TypeId::of::<T>(), signature);
    }

    pub fn entity_destroyed(&mut self, entity: EntityType) {
        for system in self.systems.iter_mut() {
            let (_, system_shared_ptr) = system;
            let system_shared = system_shared_ptr.clone();
            if let Ok(mut system_mutext) = system_shared.lock() {
                system_mutext.entities.remove(&entity);
            };
        }
    }
    pub fn entity_signature_changed(&mut self, entity: EntityType, signature: &Signature) {
        for system in self.systems.iter_mut() {
            let (type_id, system_shared_ptr) = system;
            if let Ok(mut system_mutext) = system_shared_ptr.clone().lock() {
                if let Some(self_signature) = self.signatures.get(&type_id) {
                    if self_signature.is_subset(signature) {
                        system_mutext.entities.insert(entity);
                    } else {
                        system_mutext.entities.remove(&entity);
                    }
                }
            }
        }
    }
}

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
        let wata = TypeId::of::<T>();

        // shared ptr for in threads, arc for thread safe
        if let None = self.systems.insert(wata, arc_system) {
            return wata;
        }
        panic!("Trying to add a duplicate system")
    }

    pub fn register_system_func<T: Any>(&mut self) -> TypeId {
        let system_shared_value = Mutex::new(System::new());
        let arc_system = Arc::new(system_shared_value);
        let wata = TypeId::of::<T>();

        // shared ptr for in threads, arc for thread safe
        if let None = self.systems.insert(wata, arc_system) {
            return wata;
        }
        panic!("Trying to add a duplicate system")
    }

    pub fn set_signatures<T: Any>(&mut self, signature: Signature) {
        self.signatures.insert(TypeId::of::<T>(), signature);
    }

    pub fn entity_destroyed(&mut self, entity: EntityType) {
        for system in self.systems.iter_mut() {
            let (_, s_p) = system;
            let locker = s_p.clone();
            if let Ok(mut s) = locker.lock() {
                s.entities.remove(&entity);
            };
        }
    }
    pub fn entity_signature_changed(&mut self, entity: EntityType, signature: &Signature) {
        for system in self.systems.iter_mut() {
            let (v, s_p) = system;
            println!("system typeid {:?} system: {:?}", v, s_p);
            if let Ok(mut s) = s_p.clone().lock() {
                if let Some(ss) = self.signatures.get(&v) {
                    // signature {1} selected signature {0, 1}
                    println!(
                        "signature {:?} selected signature {:?} is a subset ? {:?}",
                        signature,
                        ss,
                        signature.is_subset(ss)
                    );
                    if signature.is_subset(ss) {
                        s.entities.insert(entity);
                    } else {
                        s.entities.remove(&entity);
                    }
                }
            }
        }
    }
}

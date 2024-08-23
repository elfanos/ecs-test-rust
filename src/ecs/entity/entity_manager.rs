use core::panic;
use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    usize,
};

use array_init::array_init;
use bit_set::BitSet;

use crate::ecs::ecs::config::{EntityType, Signature, MAX_ENTITIES};

#[derive(Debug)]
pub enum EntityManagerResponse {
    CantAddMoreEntities,
    AddedEntities,
    CantRemoveEntity,
    RemovedEntity,
}

#[derive(Debug)]
pub struct EntityManager {
    entities: VecDeque<EntityType>,
    signatures: [Option<Signature>; 5000],
    living: u32,
}

impl EntityManager {
    pub fn create() -> Self {
        let mut vec_dequeue = VecDeque::new();
        for entity in 0..MAX_ENTITIES {
            vec_dequeue.push_back(entity);
        }

        Self {
            entities: vec_dequeue,
            living: 0,
            signatures: array_init::array_init(|_| None),
        }
    }
    pub fn create_entity(&mut self) -> u32 {
        if let Some(id) = self.entities.pop_front() {
            self.living += 1;
            id
        } else {
            panic!("could not create entity")
        }
    }

    pub fn set_signature(&mut self, entity: EntityType, signature: Signature) {
        self.signatures[entity as usize] = Some(signature)
    }
    pub fn remove_signature(&mut self, entity: EntityType) {
        self.signatures[entity as usize] = None;
    }

    pub fn destroy_entity(&mut self, entity: EntityType) {
        self.signatures[entity as usize] = None;
        self.entities.push_back(entity);
    }

    pub fn get_signature(&mut self, entity: EntityType) -> &mut Signature {
        println!("enitity {:?}", entity);
        if let Some(signature) = self.signatures.index_mut(entity as usize) {
            return signature;
        }
        panic!("tryig to access a entity signature from non existing");
    }
}

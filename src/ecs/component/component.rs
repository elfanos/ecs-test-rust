use core::panic;
use std::{any::Any, collections::HashMap};

use crate::ecs::ecs::config::{EntityType, MAX_ENTITIES};

pub trait IComponent: Any {
    fn entity_destroyed(&mut self, entity: EntityType);
    // fn get_component<T>(&mut self) -> Self;
}

#[derive(Debug)]
pub struct ComponentArray {
    component_array: [Option<Box<dyn Any>>; MAX_ENTITIES as usize],
    entity_to_index_map: HashMap<EntityType, usize>,
    index_to_entity_map: HashMap<usize, EntityType>,
    size: usize,
}
impl ComponentArray {
    pub fn new() -> Self {
        Self {
            component_array: array_init::array_init(|_| None), // Initialize the array with None values
            entity_to_index_map: HashMap::new(),
            index_to_entity_map: HashMap::new(),
            size: 0,
        }
    }
    // pub fn get_component_array()->

    pub fn insert_data<T: IComponent + 'static>(&mut self, entity: EntityType, component: T) {
        let new_index = self.size;
        assert!(
            !self.entity_to_index_map.contains_key(&entity),
            "Data ids added to the same id more than once."
        );

        assert!(
            (self.size as u32) < MAX_ENTITIES,
            "To many ids in this DataContainer"
        );

        self.entity_to_index_map.insert(entity, new_index);
        self.index_to_entity_map.insert(new_index, entity);
        self.component_array[new_index] = Some(Box::new(component));
        self.size += 1;
    }

    pub fn get_data_mut<T: Any>(&mut self, entity: EntityType) -> &mut T {
        assert!(
            self.entity_to_index_map.contains_key(&entity),
            "Retrieving non-existent component."
        );

        if let Some(v) = self.component_array[self.entity_to_index_map[&entity]]
            .as_mut()
            .and_then(|c| c.downcast_mut::<T>())
        {
            return v;
        } else {
            panic!("Cannot get the entity component from the given entity");
        }
    }
    pub fn get_data<T: Any>(&mut self, entity: EntityType) -> &T {
        assert!(
            self.entity_to_index_map.contains_key(&entity),
            "Retrieving non-existent component."
        );

        if let Some(v) = self.component_array[self.entity_to_index_map[&entity]]
            .as_mut()
            .unwrap()
            .downcast_ref::<T>()
        {
            return v;
        } else {
            panic!("wawd");
        }
    }
    pub fn get_components(&mut self) -> &[Option<Box<dyn Any>>] {
        &self.component_array
    }

    pub fn remove_data(&mut self, entity: EntityType) {
        assert!(
            self.entity_to_index_map.contains_key(&entity),
            "Removing non-existent component."
        );

        let index_of_removed_entity = self.entity_to_index_map[&entity];
        let index_of_last_element = self.size - 1;

        self.component_array[index_of_removed_entity] =
            self.component_array[index_of_last_element].take();

        let entity_of_last_element = self.index_to_entity_map[&index_of_last_element];
        self.entity_to_index_map
            .insert(entity_of_last_element, index_of_removed_entity);
        self.index_to_entity_map
            .insert(index_of_removed_entity, entity_of_last_element);

        self.entity_to_index_map.remove(&entity);
        self.index_to_entity_map.remove(&index_of_last_element);

        self.size -= 1;
    }
    //
}
impl IComponent for ComponentArray {
    fn entity_destroyed(&mut self, entity: EntityType) {
        if self.entity_to_index_map.contains_key(&entity) {
            self.remove_data(entity);
        }
    }
}

// Implement Debug for dyn IComponents (this is optional if you have a concrete implementation)
impl std::fmt::Debug for dyn IComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IComponents").finish()
    }
}

#[cfg(test)]
mod components {
    use super::*;

    type EntityType = u32;
    const MAX_ENTITIES: usize = 1000;

    #[derive(Debug, PartialEq, Eq)]
    struct TestComponent {
        value: i32,
    }
}

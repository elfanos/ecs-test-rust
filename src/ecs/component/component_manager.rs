use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::ecs::ecs::config::{ComponentType, EntityType};

use super::component::{ComponentArray, IComponent};

#[derive(Debug)]
pub struct ComponentManager {
    component_types: HashMap<TypeId, ComponentType>,
    component_arrays: HashMap<TypeId, Arc<Mutex<ComponentArray>>>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            component_arrays: HashMap::new(),
            next_component_type: 0,
        }
    }

    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();

        assert!(
            !self.component_types.contains_key(&type_id),
            "Registering component type more than once."
        );
        let component_array = ComponentArray::new();

        self.component_types
            .insert(type_id, self.next_component_type);
        self.component_arrays
            .insert(type_id, Arc::new(Mutex::new(component_array)));

        self.next_component_type += 1;
    }

    pub fn add_component<T: Any + IComponent>(&mut self, entity: EntityType, component: T) {
        let mut guard = self.get_component_array::<T>().lock().unwrap();
        guard.insert_data(entity, component);
    }
    pub fn remove_component<T: Any + IComponent>(&mut self, entity: EntityType) {
        self.get_component_internal_remove_data::<T>(entity);
    }

    fn get_component_internal_remove_data<T: Any + IComponent>(&self, entity: EntityType) {
        let mut guard = self.get_component_array::<T>().lock().unwrap();
        guard.remove_data(entity);
    }

    pub fn get_component<T: Any + IComponent>(&self, entity: EntityType) -> &T {
        self.get_component_internal_get_data::<T>(entity)
    }

    pub fn get_component_type<T: Any>(&self) -> &u32 {
        let type_id = TypeId::of::<T>();
        self.component_types.get(&type_id).unwrap()
    }

    pub fn get_component_mut<T: Any + IComponent, F, R>(&self, entity: EntityType, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let val = Arc::clone(
            self.component_arrays
                .clone()
                .get(&TypeId::of::<T>())
                .unwrap(),
        );
        let mut guard = val.lock().unwrap();
        let component = guard.get_data_mut::<T>(entity);

        f(component)
    }

    fn get_component_internal_get_data<T: Any + IComponent>(&self, entity: EntityType) -> &T {
        let mut guard = self.get_component_array::<T>().lock().unwrap();
        let data = guard.get_data::<T>(entity);

        unsafe { &*(data as *const _) }
    }

    fn get_component_array<T: Any + IComponent>(&self) -> &Arc<Mutex<ComponentArray>> {
        self.component_arrays
            .get(&TypeId::of::<T>())
            .expect("Component array not found")
    }

    pub fn entity_destroyed(&mut self, entity: EntityType) {
        for component_array in self.component_arrays.iter_mut() {
            let (_, components) = component_array;
            if let Ok(mut component) = components.lock() {
                component.entity_destroyed(entity)
            }
        }
    }
}

#[cfg(test)]
mod component_manager {
    use super::*;

    #[derive(Debug)]
    struct TestComponent {
        data: i32,
    }

    impl IComponent for TestComponent {
        fn entity_destroyed(&mut self, _entity: EntityType) {}
    }

    #[test]
    fn test_register_component() {
        let mut manager = ComponentManager::new();
        manager.register_component::<TestComponent>();

        assert!(manager
            .component_types
            .contains_key(&TypeId::of::<TestComponent>()));
        assert!(manager
            .component_arrays
            .contains_key(&TypeId::of::<TestComponent>()));
    }

    #[test]
    fn test_add_and_get_component() {
        let mut manager = ComponentManager::new();
        manager.register_component::<TestComponent>();

        let entity: EntityType = 1;
        let component = TestComponent { data: 42 };

        manager.add_component(entity, component);

        let opt_shareable_array = manager.get_component_array::<TestComponent>();
        let mut numbers_of_entries = 0;
        if let Ok(mut shareable_array) = opt_shareable_array.lock() {
            for correct in shareable_array.get_components().iter() {
                if let Some(_) = correct {
                    numbers_of_entries += 1;
                }
            }
        } else {
            assert!(false);
        }
        assert_eq!(numbers_of_entries, 1);
    }
}

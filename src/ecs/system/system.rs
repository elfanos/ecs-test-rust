use std::collections::HashSet;

use crate::ecs::ecs::config::EntityType;

#[derive(Debug)]
pub struct System {
    pub entities: HashSet<EntityType>,
}
impl System {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
    pub fn run_system(&self, f: fn(&System)) {
        f(self);
    }
}
pub struct Physics {}

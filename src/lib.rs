//! A minimalist, safe and fast ECS.
//! Composed of two libraries:
//! * entity_component
//! * world_dispatcher
//!
//! Planck ECS is a library that brings those two smaller parts together.
//! It adds the `maintain` function to world, which takes care of cleaning up
//! dead entities after running systems.
pub use entity_component::*;
pub use world_dispatcher::*;

use std::cell::RefMut;
use std::any::Any;

/// Extension to the `World` struct that adds a maintain() method.
pub trait WorldExt {
    fn maintain(&mut self);
}

impl WorldExt for World {
    /// Removes dead entities from all the registered storages.
    fn maintain(&mut self) {
        if let Ok(mut entities) = self.get_mut::<Entities>() {
            for (typeid, func) in COMPONENT_REGISTRY.lock().unwrap().iter() {
                if let Ok(any) = self.get_by_typeid(typeid) {
                    let any: RefMut<dyn Any> = RefMut::map(any, |j| j.as_any_mut());
                    func(any, entities.killed());
                }
            }
            entities.clear_killed();
        }
    }
}


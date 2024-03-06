use std::collections::HashMap;

use crate::prototype::Prototype;

pub trait AbstractFactory {
    fn get_prototype_map(&self) -> HashMap<String, Box<dyn Prototype>>;
}

/**
  Although this is used in the tests, it's just an example of how
  you can use this pattern and whatever client you're implementing
  heavily informs the exact factory design.

  In this case, it's a named instance of a prototype, but you can
  imagine a family of related objects that you want to use together

  Another popular example is to use this pattern to create a set
  of UI widgets with a consistent look and feel, but you can have
  different styles implemented, and a user might select the one they
  find most appealing. In this case the factory defines several different types.
*/

/**
 * Client of the abstract factory.
 */
pub struct PrototypeManager {
    prototypes: HashMap<String, Box<dyn Prototype>>,
}

impl PrototypeManager {
    pub fn new() -> PrototypeManager {
        PrototypeManager {
            prototypes: HashMap::new(),
        }
    }

    pub fn add_prototypes_from_factory(&mut self, factory: Box<dyn AbstractFactory>) {
        for (id, prototype) in factory.get_prototype_map() {
            self.prototypes.insert(id, prototype);
        }
    }

    pub fn get_instance(&self, id: &str) -> Option<Box<dyn Prototype>> {
        match self.prototypes.get(id) {
            Some(prototype) => Some(prototype.instantiate()),
            None => None,
        }
    }
}

impl Default for PrototypeManager {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;

// This will use the prototype pattern to demonstrate how you can use the abstract factory pattern to create a pluggable system
use crate::{abstractfactory::*, prototype::Prototype};

// Example prototypes
pub struct ConcretePrototypeA {
    data: String,
}

impl ConcretePrototypeA {
    pub fn new() -> ConcretePrototypeA {
        ConcretePrototypeA {
            data: "A".to_string(),
        }
    }
}

impl Prototype for ConcretePrototypeA {
    fn instantiate(&self) -> Box<dyn Prototype> {
        Box::new(ConcretePrototypeA {
            data: self.data.clone(),
        })
    }
    fn do_thing(&self) -> String {
        self.data.clone()
    }
}

pub struct ConcretePrototypeB {
    data: String,
}

impl ConcretePrototypeB {
    pub fn new() -> ConcretePrototypeB {
        ConcretePrototypeB {
            data: "B".to_string(),
        }
    }
}

impl Prototype for ConcretePrototypeB {
    fn instantiate(&self) -> Box<dyn Prototype> {
        Box::new(ConcretePrototypeB {
            data: self.data.clone(),
        })
    }
    fn do_thing(&self) -> String {
        self.data.clone()
    }
}
// Example factory implementations
struct ConcreteFactoryA;

impl ConcreteFactoryA {
    fn new() -> ConcreteFactoryA {
        ConcreteFactoryA
    }
}

impl AbstractFactory for ConcreteFactoryA {
    fn get_prototype_map(&self) -> HashMap<String, Box<dyn Prototype>> {
        let mut map = HashMap::<String, Box<dyn Prototype>>::new();
        map.insert("pA".to_string(), Box::new(ConcretePrototypeA::new()));
        map
    }
}

struct ConcreteFactoryB;
impl ConcreteFactoryB {
    fn new() -> ConcreteFactoryB {
        ConcreteFactoryB
    }
}

impl AbstractFactory for ConcreteFactoryB {
    fn get_prototype_map(&self) -> HashMap<String, Box<dyn Prototype>> {
        let mut map = HashMap::<String, Box<dyn Prototype>>::new();
        map.insert("pB".to_string(), Box::new(ConcretePrototypeB::new()));
        map
    }
}

#[test]
fn test_dynamic_factories() {
    let mut mgr = PrototypeManager::new();

    mgr.add_prototypes_from_factory(Box::new(ConcreteFactoryA::new()));

    let p1 = mgr.get_instance("pA");
    assert_eq!(p1.unwrap().do_thing(), "A");

    // Not registered yet, so will return None
    let p2 = mgr.get_instance("pB");
    assert_eq!(p2.is_none(), true);

    mgr.add_prototypes_from_factory(Box::new(ConcreteFactoryB::new()));

    // Now type B is added, should be available.
    let p2 = mgr.get_instance("pB");
    assert_eq!(p2.is_none(), false);
    assert_eq!(p2.unwrap().do_thing(), "B");
}

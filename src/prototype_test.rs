use crate::prototype::*;

// implement the Prototype trait for a concrete type.
struct ConcretePrototype {
    data: String,
}

// Constructor for the ConcretePrototype.
impl ConcretePrototype {
    fn new(data: &str) -> ConcretePrototype {
        ConcretePrototype {
            data: String::from(data),
        }
    }
}

// Simple implementation of the Prototype trait.
impl Prototype for ConcretePrototype {
    fn instantiate(&self) -> Box<dyn Prototype> {
        Box::new(ConcretePrototype {
            data: self.data.clone(),
        })
    }

    fn do_thing(&self) -> String {
        self.data.clone()
    }
}

struct PrototypeManager {
    prototypes: Vec<Box<dyn Prototype>>,
}

// Simple class to indicate which prototype to get
enum PrototypeType {
    One,
    Two,
}

// Stores prototypes and can return new instances of them.
impl PrototypeManager {
    pub fn new() -> PrototypeManager {
        let mut mgr: PrototypeManager = PrototypeManager {
            prototypes: Vec::new(),
        };

        mgr.add_prototype(Box::new(ConcretePrototype::new("A")));
        mgr.add_prototype(Box::new(ConcretePrototype::new("B")));

        mgr
    }

    fn add_prototype(&mut self, p: Box<dyn Prototype>) {
        self.prototypes.push(p);
    }

    pub fn get_prototype(&self, ptype: PrototypeType) -> Box<dyn Prototype> {
        match ptype {
            PrototypeType::One => self.prototypes[0].instantiate(),
            PrototypeType::Two => self.prototypes[1].instantiate(),
        }
    }
}

#[test]
fn test_concrete_instance() {
    let mgr: PrototypeManager = PrototypeManager::new();

    let p1 = mgr.get_prototype(PrototypeType::One);
    let p2 = mgr.get_prototype(PrototypeType::Two);

    assert_eq!(p1.do_thing(), "A");
    assert_eq!(p2.do_thing(), "B");
}

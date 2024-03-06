use std::vec::Vec;

// I was thinking about extending apply to accept a closure
pub trait Composible {
    fn apply(&self) -> String;
}

// This is the simplest thing I could think of for a leafy type
// Other options could be to have and Enum or Generic, I'd be
// interested to see this with an enum
pub struct Leaf {
    elem: String,
}

impl Composible for Leaf {
    fn apply(&self) -> String {
        self.elem.clone()
    }
}

impl Leaf {
    pub fn new(s: &str) -> Leaf {
        Leaf {
            elem: String::from(s),
        }
    }
}
// Took the route of making the top level composite hold an array
// This has an advantage of simplicity, but there's nothing
// stopping having a separate class, or using a Box<dyn Composible>
// and calling this a collection.
#[derive(Default)]
pub struct Composite {
    data: Vec<Box<dyn Composible>>,
}

impl Composible for Composite {
    // For apply to accept a closure, maybe replace the reduce with a second map
    fn apply(&self) -> String {
        let it = self.data.iter();
        let data = it
            .map(|elem| elem.apply())
            .reduce(|st, rest| format!("{}+{}", st, rest));
        match data {
            None => String::new(),
            Some(st) => format!("({})", st),
        }
    }
}

impl Composite {
    pub fn new() -> Composite {
        Composite { data: Vec::new() }
    }

    pub fn add<T: Composible + 'static>(&mut self, elem: T) {
        self.data.push(Box::new(elem));
    }
}

use std::vec::Vec;

pub trait Composible {
    fn apply(&self) -> String;
}
type Elem = dyn Composible;

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

pub struct Composite {
    data: Vec<Box<Elem>>,
}

impl Composible for Composite {
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

use std::vec::Vec;

pub trait Composible {
    fn apply(&self) -> String;
}
type Elem<'a> = &'a dyn Composible;

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

pub struct Composite<'a> {
    data: Vec<Box<Elem<'a>>>,
}

impl<'a> Composible for Composite<'a> {
    fn apply(&self) -> String {
        let it = self.data.iter();
        let data = it
            .map(|elem| elem.apply())
            .reduce(|st, rest| format!("{}+{}", st, rest));
        match data {
            None => String::new(),
            Some(st) => format!("({})",st),
        }
    }
}

impl<'a> Composite<'a> {
    pub fn new() -> Composite<'a> {
        Composite { data: Vec::new() }
    }

    pub fn push(&mut self, elem: Elem<'a>) {
        self.data.push(Box::<Elem<'a>>::new(elem));
    }

    pub fn pop(&mut self) -> Option<*mut Elem<'a>> {
        match self.data.pop() {
            None => None,
            Some(b) => Some(Box::<Elem<'a>>::into_raw(b)),
        }
    }
}

pub trait Visitor {
    fn visit_cat(&self) -> String;
    fn visit_dog(&self) -> String;
    fn visit_fish(&self) -> String;
}

pub trait AcceptVisitor {
    fn accept(&self, visitor: &dyn Visitor) -> String;
}

#[derive(Default)]
pub struct Cat;
impl Cat {
    pub fn new() -> Cat {
        Cat
    }
}

#[derive(Default)]
pub struct Dog;
impl Dog {
    pub fn new() -> Dog {
        Dog
    }
}

#[derive(Default)]
pub struct Fish;
impl Fish {
    pub fn new() -> Fish {
        Fish
    }
}

impl AcceptVisitor for Cat {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_cat()
    }
}

impl AcceptVisitor for Dog {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_dog()
    }
}

impl AcceptVisitor for Fish {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_fish()
    }
}

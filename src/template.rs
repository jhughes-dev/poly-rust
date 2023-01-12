// This is one of the simplest patterns for rust, and is much cleaner with
// generics, but in case of strategy and composite, if your types need to
// implement this, you might have trouble storing the generics without
// knowing size. This example shows how a dynamic template would work.

pub trait Interface {
    fn hook(&self) -> String;
}

impl dyn Interface {
    pub fn do_thing(&self) -> String {
        format!("*{}*", self.hook())
    }

    // called as <dyn Interface>::new(...)
    // Likely, in a real situation, this is better wrapped in a struct
    // and look more like a strategy pattern
    pub fn new<T: Interface + 'static>(elem: T) -> Box<dyn Interface> {
        Box::new(elem)
    }
}
pub struct Implementor1 {}
pub struct Implementor2 {}

impl Interface for Implementor1 {
    fn hook(&self) -> String {
        String::from("1")
    }
}

impl Interface for Implementor2 {
    fn hook(&self) -> String {
        String::from("2")
    }
}

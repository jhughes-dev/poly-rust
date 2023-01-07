pub trait Interface {
    fn hook(&self) -> String;
}

impl dyn Interface {
    pub fn do_thing(&self) -> String {
        format!("*{}*", self.hook())
    }
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

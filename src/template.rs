pub trait Interface {
    fn hook(&self) -> String;
}

impl dyn Interface {
    pub fn do_thing(&self) -> String {
        format!("*{}*", self.hook())
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

// Turducken

pub trait Decorator {
    fn delegate(&self) -> String;
}

// The Base Component needs to be not call another delegate,
// so it will stop doing things eventually. Alternatively, you could
// implement the decorator structs to accept Option<Box<dyn Decorator>> and
// then stop if None was passed--then you don't need the base--but this
// is a typical approach. Both look about as awkward to me.
//
// Another approach would be to avoid using NEW and instead use
// fn decorate(Box<dyn Decorator>) -> Box<dyn Decorator> to do the decoration,
// and some other function is used for the base regardless of type.
// See decorator_alt.rs for that example.
pub struct BaseComponent {

}

impl BaseComponent {
    pub fn new() -> BaseComponent {
        BaseComponent{}
    }
}

impl Decorator for BaseComponent {

    fn delegate(&self) -> String {
        "".to_string()
    }
}

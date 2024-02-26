pub trait Prototype {
    // Returns a new instance of a struct implementing the Prototype trait,
    // not using the name "clone" to avoid confusion with the Clone trait.
    fn instantiate(&self) -> Box<dyn Prototype>;
    // Just something to make this a little more interesting.
    fn do_thing(&self) -> String;
}

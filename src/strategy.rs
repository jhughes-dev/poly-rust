pub trait Strategy {
    fn something(&self) -> String;
}

pub struct Dohicky {
    strat: Box<dyn Strategy>,
}

// this feels pretty straitforward

impl Dohicky {
    pub fn new<T: Strategy + 'static>(p: T) -> Dohicky {
        Dohicky { strat: Box::new(p) }
    }

    pub fn do_something(&self) -> String {
        self.strat.something()
    }
    // Adding the ability to have a runtime switch to drive
    // home the polymorphism
    pub fn change_strategy<T: Strategy + 'static>(&mut self, p: T) {
        self.strat = Box::new(p);
    }
}

pub struct Thingy1 {}

impl Strategy for Thingy1 {
    fn something(&self) -> String {
        String::from("Thingy1")
    }
}

pub struct Thingy2 {}

impl Strategy for Thingy2 {
    fn something(&self) -> String {
        String::from("Thingy2")
    }
}

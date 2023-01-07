pub trait Strategy {
    fn something(&self) -> String;
}

pub struct Dohicky {
    strat: Box<dyn Strategy>,
}

impl Dohicky {
    pub fn new<T: Strategy + 'static>(p: T) -> Dohicky {
        Dohicky { strat: Box::new(p) }
    }

    pub fn do_thing(&self) -> String {
        self.strat.something()
    }

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

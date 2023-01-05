pub trait Strategy {
    fn something(&self) -> String;
}

pub struct Dohicky <'a> {
    strat: Box<&'a dyn Strategy>,
}

impl<'a> Dohicky<'a> {
    pub fn new(p: &'a dyn Strategy) -> Dohicky {
        Dohicky { strat: Box::<&'a dyn Strategy>::new(p) }
    }

    pub fn do_thing(&self) -> String {
        self.strat.something()
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

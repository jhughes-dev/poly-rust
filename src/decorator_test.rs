use crate::decorator::*;

pub struct Turkey {
    next: Box<dyn Decorator>,
}

impl Turkey {
    pub fn new(next: Box<dyn Decorator>) -> Turkey {
        Turkey{next}
    }
}

impl Decorator for Turkey {
    fn delegate(&self) -> String {
        "Turkey".to_string() + &self.next.delegate()
    }
}

pub struct Duck {
    next: Box<dyn Decorator>,
}

impl Duck {
    pub fn new(next: Box<dyn Decorator>) -> Duck {
        Duck{next}
    }
}

impl Decorator for Duck {
    fn delegate(&self) -> String {
        "Duck".to_string() + &self.next.delegate()
    }
}

pub struct Chicken {
    next: Box<dyn Decorator>,
}

impl Chicken {
    pub fn new(next: Box<dyn Decorator>) -> Chicken {
        Chicken{next}
    }
}

impl Decorator for Chicken {
    fn delegate(&self) -> String {
        "Chicken".to_string() + &self.next.delegate()
    }
}

#[test]
fn make_turducken()  {

    // Add the boxing and new here makes this a little hard to
    // understand, see decorator_alt.rs for what I think it a slightly better API
    let nothing = Box::new(BaseComponent::new());
    let turducken = Box::new(Turkey::new(
            Box::new(Duck::new(
                Box::new(Chicken::new(nothing))
            ))
        ));

    assert_eq!("TurkeyDuckChicken", turducken.delegate());

    // As an excercise,
    //    put a boiled egg, peeled, in the chicken,
    //       then stuff that in an ostrich,
    //          and finally feed that to a tyrannosaurus rex
    //              Heat at 420F for 6 days-> Tyrosturduckegg
}

use crate::decorator::*;

pub struct Turkey {
    next: Option<Box<dyn Decorator>>,
}

// This implementation does more than the standard approach, but the client code looks
// a lot cleaner. Since you can share this logic, it turns out to not be all that bad.
fn delegate_helper(msg: String, next: &Option<Box<dyn Decorator>>) -> String {
    match &next {
        Some(next) => msg + &next.delegate(),
        None => msg,
    }
}

impl Turkey {
    // Used as the terminiating decorator
    pub fn finish() -> Box<Turkey> {
        Box::new(Turkey { next: None })
    }
    // Used to add another decorator
    pub fn decorate(next: Box<dyn Decorator>) -> Box<Turkey> {
        Box::new(Turkey { next: Some(next) })
    }
}

// Note, the `finish` and `decorate` functions could be made into a separate trait
// with a deriveable macro for far less code duplication.
impl Decorator for Turkey {
    fn delegate(&self) -> String {
        delegate_helper("Turkey".to_string(), &self.next)
    }
}

struct Duck {
    next: Option<Box<dyn Decorator>>,
}

impl Duck {
    fn finish() -> Box<Duck> {
        Box::new(Duck { next: None })
    }
    fn decorate(next: Box<dyn Decorator>) -> Box<Duck> {
        Box::new(Duck { next: Some(next) })
    }
}

impl Decorator for Duck {
    fn delegate(&self) -> String {
        delegate_helper("Duck".to_string(), &self.next)
    }
}

struct Chicken {
    next: Option<Box<dyn Decorator>>,
}

impl Chicken {
    fn finish() -> Box<Chicken> {
        Box::new(Chicken { next: None })
    }
    fn decorate(next: Box<dyn Decorator>) -> Box<Chicken> {
        Box::new(Chicken { next: Some(next) })
    }
}

impl Decorator for Chicken {
    fn delegate(&self) -> String {
        delegate_helper("Chicken".to_string(), &self.next)
    }
}

#[test]
fn make_turducken() {

    // I think this looks a lot nicer than the standard approach since you don't wind up manually boxing everything.
    let turducken = Turkey::decorate(Duck::decorate(Chicken::finish()));

    assert_eq!("TurkeyDuckChicken", turducken.delegate());

    let chiturck = Chicken::decorate(Turkey::decorate(Duck::finish()));

    assert_eq!("ChickenTurkeyDuck", chiturck.delegate());

    let duchickey = Duck::decorate(Chicken::decorate(Turkey::finish()));

    assert_eq!("DuckChickenTurkey", duchickey.delegate());
}

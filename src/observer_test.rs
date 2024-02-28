use crate::observer::*;
use std::rc::Rc;
use std::cell::RefCell;

struct ConcreteObserver {
    pub state: i32,
}

impl ConcreteObserver {
    fn new() -> ConcreteObserver {
        ConcreteObserver {
            state: 0,
        }
    }
}

impl Observer for ConcreteObserver {
    fn notify(&mut self, _observable: &dyn Observable) {
        self.state += 1;
    }
}

#[test]
fn t() {
    let mut observable = ConcreteObservable::new();
    // Need to have RC which knows the concrete type ...
    let ob1 = Rc::new(RefCell::new(ConcreteObserver::new()));
    let ob2 = Rc::new(RefCell::new(ConcreteObserver::new()));

    // ... and RCs to a trait object
    let ob1_ref = ob1.clone() as Rc<RefCell<dyn Observer>>;
    let ob2_ref = ob2.clone() as Rc<RefCell<dyn Observer>>;

    observable.add_observer(&Rc::downgrade(&ob1_ref));
    observable.add_observer(&Rc::downgrade(&ob2_ref));

    observable.notify_observers();

    assert_eq!(ob1.borrow().state, 1);
    assert_eq!(ob2.borrow().state, 1);
}

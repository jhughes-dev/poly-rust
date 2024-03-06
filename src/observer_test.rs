use crate::observer::*;
use std::cell::RefCell;
use std::rc::Rc;

struct ConcreteObserver {
    pub state: i32,
}

impl ConcreteObserver {
    fn new() -> ConcreteObserver {
        ConcreteObserver { state: 0 }
    }
}

impl Observer for ConcreteObserver {
    fn notify(&mut self, _observable: &dyn Observable) {
        self.state += 1;
    }
}

#[test]
fn test_observer() {
    let mut observable = ConcreteObservable::new();
    // Need to have RC which knows the concrete type ...
    let ob1 = Rc::new(RefCell::new(ConcreteObserver::new()));
    let ob2 = Rc::new(RefCell::new(ConcreteObserver::new()));

    // ... and RCs to a RefCell containing a trait object - i.e. type erased
    let ob1_ref: Rc<RefCell<dyn Observer>> = ob1.clone();
    let ob2_ref: Rc<RefCell<dyn Observer>> = ob2.clone();

    observable.add_observer(&ob1_ref);
    observable.add_observer(&ob2_ref);

    observable.notify_observers();

    assert_eq!(ob1.borrow().state, 1);
    assert_eq!(ob2.borrow().state, 1);

    observable.notify_observers();

    assert_eq!(ob1.borrow().state, 2);
    assert_eq!(ob2.borrow().state, 2);
}

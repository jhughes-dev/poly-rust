use std::{
    cell::RefCell,
    rc::*,
};

// Observer looks straitforward, and it shows some aspects of Rust that really shine, but
// things get tricky with the Box<dyn Observer>, since having a mut reference stored in the
// Observable prevents it from being used elsewhere. This would put a limitation on the
// update method to be immutable, and that's not really useful, so we need to get creative.
// We're going to use Rc and Weak pointers with RefCell instead of Box like I'm so fond of,
// but it makes the client code harder to work with.

// Note: Even though we can make this work, it's not how I would recommend solve problems
// that people apply the observer pattern to. Using event handlers a'la closures is a better
// approach which lets you avoid the clunklyness of Rc<RefCell<dyn Observer>>, plus you don't
// need to implement the trait for your type.

pub trait Observable {
    fn add_observer(&mut self, observer: &Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: &Rc<RefCell<dyn Observer>>);
    fn notify_observers(&mut self);
}

pub trait Observer {
    fn notify(&mut self, observable: &dyn Observable);
}

pub struct ConcreteObservable {
    observers: Vec<Weak<RefCell<dyn Observer>>>,
}

impl ConcreteObservable {
    pub fn new() -> ConcreteObservable {
        ConcreteObservable {
            observers: Vec::new(),
        }
    }
}

impl Observable for ConcreteObservable {
    fn add_observer(&mut self, observer: &Rc<RefCell<dyn Observer>>) {
        self.observers.push(Rc::downgrade(observer));
    }

    fn remove_observer(&mut self, observer: &Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|o| !o.ptr_eq(&Rc::downgrade(observer)));
    }

    fn notify_observers(&mut self) {
        let mut observers_to_remove = Vec::new();
        for observer in &self.observers {
            // This is where the Weak pointer to RefCell comes in handy, and
            // we can get at a mut ref only when we're calling notify
            if let Some(ob) = observer.upgrade() {
                ob.borrow_mut().notify(self)
            } else {
                // If the observer is dead, we mark it for removal
                observers_to_remove.push(observer.clone());
            }
        }

        // Remove the dead observers
        for observer in observers_to_remove {
            self.observers.retain(|o| !o.ptr_eq(&observer));
        }
    }
}

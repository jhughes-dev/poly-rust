// I think observer is a pattern where Rust really shines, it's easy
// to tack on the trait to any struct and it's easy to implement the trait.
// However, things get trick with the Box<dyn Observer>, since having a mut reference
// stored in the Observable prevents it from being used elsewhere.
// This puts a limitation on the update method to be immutable, and
// that's not really useful, so we need to get creative.
// We're going to use Weak and RC pointers instead of Box like I'm so fond of.
use std::{
    cell::RefCell,
    rc::Weak,
};

pub trait Observable {
    fn add_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>);
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
    fn add_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>) {
        self.observers.push(observer.clone());
    }

    fn remove_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>) {
        self.observers.retain(|o| !o.ptr_eq(observer));
    }

    fn notify_observers(&mut self) {
        let mut observers_to_remove = Vec::new();
        for observer in &self.observers {
            // This is where the Weak pointer comes in handy, and
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
            self.remove_observer(&observer);
        }
    }
}

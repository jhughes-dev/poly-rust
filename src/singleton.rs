// The idea is that there's one controler and that a user implements different algorithms for something
// Basically, it's a singlton implementing a strategy pattern.

pub trait Controller {
    fn get_data(&self) -> String;
}

struct SingletonInternal {
    data: Option<Box<dyn Controller>>,
}

impl SingletonInternal {
    pub fn get_data(&self) -> String {
        match &self.data {
            Some(st) => st.get_data(),
            None => String::new(),
        }
    }

    pub fn set_controller<T: Controller + 'static>(&mut self, ctrl: T) {
        self.data = Some(Box::new(ctrl));
    }

    const fn new() -> SingletonInternal {
        SingletonInternal { data: None }
    }
}

fn get_instance() -> &'static mut SingletonInternal {
    static mut INSTANCE: SingletonInternal = SingletonInternal::new();
    unsafe { &mut INSTANCE }
}

pub mod controller {
    pub fn get_data() -> String {
        super::get_instance().get_data()
    }

    pub fn set_controller<T: super::Controller + 'static>(ctrl: T) {
        super::get_instance().set_controller(ctrl);
    }
}

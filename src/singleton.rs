// Singleton isn't really polymorpic, so this isn't so much in the spirit of the project, but I wanted to do it with a minumum of unsafe calls, and "non-initialized"

struct SingletonInternal {
    data: Option<String>,
}

impl SingletonInternal {
    pub fn get_data(&self) -> String {
        match &self.data {
            Some(st) => st.clone(),
            None => String::new(),
        }
    }

    pub fn update_data(&mut self, d: &str) {
        self.data = Some(String::from(d));
    }

    const fn new() -> SingletonInternal {
        SingletonInternal { data: None }
    }
}

fn get_instance() -> &'static mut SingletonInternal {
    static mut INSTANCE: SingletonInternal = SingletonInternal::new();
    unsafe { &mut INSTANCE }
}

pub mod global {
    pub fn get_data() -> String {
        super::get_instance().get_data()
    }

    pub fn update_data(d: &str) {
        super::get_instance().update_data(d);
    }
}

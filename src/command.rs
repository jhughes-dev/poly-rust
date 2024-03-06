// Command Trait with execute method
// This method accepts the internal processor
pub trait Command {
    fn execute(&self, proc: &mut Processor);
}

// Since processor has private data it can't be created outside the module.
#[derive(Default)]
pub struct Processor {
    data: i32,
}

// These functions are public to let commands be implemented outside this
// module, but that's a design decision for the example. These methods could
// be private so that only commands implemented in the module are allowed.
impl Processor {
    pub fn increment(&mut self, amount: i32) {
        self.data += amount;
    }

    pub fn decrement(&mut self, amount: i32) {
        self.data -= amount;
    }

    pub fn clear(&mut self) {
        self.data = 0;
    }
}

// App wraps the processor and keeps it from being generally accessible except
// through the command interface.
#[derive(Default)]
pub struct App {
    proc: Processor,
}

// App implements a public interface which accepts commands and relays the
// internal data from the processor.
impl App {
    pub fn new() -> App {
        App {
            proc: Processor { data: 0 },
        }
    }

    pub fn apply_command(&mut self, cmd: &dyn Command) {
        cmd.execute(&mut self.proc);
    }

    pub fn get_data(&self) -> i32 {
        self.proc.data
    }
}

// These commands are pretty artificial just to demonstrate how commands could
// be used to provide an end-user interface with limitations but allow a more
// incremental development interface.

// Set the internal value to some user decided value
pub struct SetValue {
    val: i32,
}

impl SetValue {
    pub fn new(val: i32) -> SetValue {
        SetValue { val }
    }
}

impl Command for SetValue {
    fn execute(&self, proc: &mut Processor) {
        proc.clear();
        proc.increment(self.val);
    }
}

// Bump the data value up by two
#[derive(Default)]
pub struct UpByTwo {}

impl UpByTwo {
    pub fn new() -> UpByTwo {
        UpByTwo {}
    }
}

impl Command for UpByTwo {
    fn execute(&self, proc: &mut Processor) {
        proc.increment(2);
    }
}

// Drop the data by three
#[derive(Default)]
pub struct DownByThree {}

impl DownByThree {
    pub fn new() -> DownByThree {
        DownByThree {}
    }
}

impl Command for DownByThree {
    fn execute(&self, proc: &mut Processor) {
        proc.decrement(3);
    }
}

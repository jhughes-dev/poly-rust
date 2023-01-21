use crate::command::*;

// This test shows the ways a set of commands could be used to control a
// separate structure without exposing the internals of that structure.
#[test]
fn workflow() {
    let mut app = App::new();

    app.apply_command(&SetValue::new(5));

    assert_eq!(app.get_data(), 5);
    app.apply_command(&UpByTwo::new());
    assert_eq!(app.get_data(), 7);

    app.apply_command(&DownByThree::new());
    assert_eq!(app.get_data(), 4);

    // Notice this won't let you create a processor
    // let mut proc = Processor {};
}

// This example allows for an external command to be implemented using the
// Processor interface, even though the Processor struct isn't available
// directly.
struct SetToZero {}

impl Command for SetToZero {
    fn execute(&self, proc: &mut Processor) {
        proc.clear();
    }
}

#[test]
fn clear_with_command() {
    let mut app = App::new();

    app.apply_command(&SetValue::new(5));
    assert_eq!(app.get_data(), 5);

    app.apply_command(&SetToZero {});
    assert_eq!(app.get_data(), 0);
}

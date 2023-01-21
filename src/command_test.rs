use crate::command::*;

#[test]
fn workflow() {
    let mut app = App::new();

    app.apply_command(&SetValue::new(5));

    assert_eq!(app.get_data(), 5);
    app.apply_command(&UpByTwo::new());
    assert_eq!(app.get_data(), 7);

    app.apply_command(&DownByThree::new());
    assert_eq!(app.get_data(), 4);

    // Notice this won't let you create a processor or access data from App
    // let mut proc = Processor {};
}

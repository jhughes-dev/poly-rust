use crate::singleton::*;

struct Controller1 {}

struct Controller2 {}

impl Controller for Controller1 {
    fn get_data(&self) -> String {
        String::from("#1")
    }
}

impl Controller for Controller2 {
    fn get_data(&self) -> String {
        String::from("#2")
    }
}

#[test]
fn set_controller() {
    {
        // Nothing in this global
        assert_eq!(controller::get_data(), "")
    }

    {
        // After updating, should get new value
        controller::set_controller(Controller1{});
        assert_eq!(controller::get_data(), "#1");
    }
    // Should still get the new value since it's stored somewhere else
    assert_eq!(controller::get_data(), "#1");

    controller::set_controller(Controller2{});
    assert_eq!(controller::get_data(), "#2");

}

// This test fails sporadically, it's supposed to do that. You can uncomment and run it a few times
// to see, but it's showing the singleton is being set from the previous test when it passes, and
// when it fails, stale() ran before the first test. There's a race condition, but it at least shows
// that the singleton is persisting--which it's supposed to.

// #[test]
// fn stale() {
//     // Should still get the new value since it's stored somewhere else
//     assert_eq!(controller::get_data(), "New");
// }

use crate::singleton::*;

#[test]
fn get_instance() {
    {
        // Nothing in this global
        assert_eq!(global::get_data(), "")
    }

    {
        // After updating, should get new value
        global::update_data("New");
        assert_eq!(global::get_data(), "New");
    }
    // Should still get the new value since it's stored somewhere else
    assert_eq!(global::get_data(), "New");
}

#[test]
fn stale() {
    // Should still get the new value since it's stored somewhere else
    assert_eq!(global::get_data(), "New");
}

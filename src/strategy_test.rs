use crate::strategy::*;

#[test]
fn test_impl_one() {
    let s = Dohicky::new(Thingy1 {});
    assert_eq!(s.do_thing(), "Thingy1");
}
#[test]
fn test_impl_two() {
    let s = Dohicky::new(Thingy2 {});
    assert_eq!(s.do_thing(), "Thingy2");
}

// This isn't technically anything special since we
// know it all at compile time, but in theory you'd
// be able to pick any strategy no matter what.
struct LocalThingy {}

impl Strategy for LocalThingy {
    fn something(&self) -> String {
        String::from("Local")
    }
}

#[test]
fn test_impl_local() {
    let mut s = Dohicky::new(LocalThingy {});
    assert_eq!(s.do_thing(), "Local");

    s.change_strategy(Thingy1 {});
    assert_eq!(s.do_thing(), "Thingy1");
}

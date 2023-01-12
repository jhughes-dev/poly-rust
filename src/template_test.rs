use crate::template::*;
#[test]
fn test_one() {
    let b = <dyn Interface>::new(Implementor1 {});
    assert_eq!(b.do_thing(), "*1*")
}
#[test]
fn test_two() {
    let b = <dyn Interface>::new(Implementor2 {});
    assert_eq!(b.do_thing(), "*2*")
}

pub struct ImplementorLocal {}

impl Interface for ImplementorLocal {
    fn hook(&self) -> String {
        String::from("L")
    }
}
#[test]
fn test_local() {
    let b = <dyn Interface>::new(ImplementorLocal {});
    assert_eq!(b.do_thing(), "*L*")
}

#[test]
fn test_hook() {
    let im = ImplementorLocal {};
    im.hook();

    let b = <dyn Interface>::new(ImplementorLocal {});
    b.hook();
}

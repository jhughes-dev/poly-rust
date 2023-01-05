use crate::template::*;
#[test]
fn test_one() {
    let b = Box::<&dyn Interface>::new(&Implementor1 {});
    assert_eq!(b.do_thing(), String::from("*1*"))
}
#[test]
fn test_two() {
    let b = Box::<&dyn Interface>::new(&Implementor2 {});
    assert_eq!(b.do_thing(), String::from("*2*"))
}

pub struct ImplementorLocal {}

impl Interface for ImplementorLocal {
    fn hook(&self) -> String {
        String::from("L")
    }
}
#[test]
fn test_local() {
    let b = Box::<&dyn Interface>::new(&ImplementorLocal {});
    assert_eq!(b.do_thing(), String::from("*L*"))
}

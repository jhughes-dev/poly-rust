use crate::composite::*;

#[test]
fn structure() {
    let mut inner = Composite::new();
    inner.add(Leaf::new("A"));
    inner.add(Leaf::new("B"));
    assert_eq!(inner.apply(), "(A+B)");

    let mut outer = Composite::new();
    outer.add(Leaf::new("C"));
    outer.add(inner);
    outer.add(Leaf::new("D"));
    assert_eq!(outer.apply(), "(C+(A+B)+D)");
}

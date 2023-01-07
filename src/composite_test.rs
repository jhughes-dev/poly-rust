use crate::composite::*;

#[test]
fn structure()
{
    let mut inner = Composite::new();
    let a = Leaf::new("A");
    let b = Leaf::new("B");
    inner.push(&a);
    inner.push(&b);
    assert_eq!(inner.apply(),"(A+B)");

    let mut outer = Composite::new();
    let c = Leaf::new("C");
    let d = Leaf::new("D");
    outer.push(&c);
    outer.push(&inner);
    outer.push(&d);
    assert_eq!(outer.apply(),"(C+(A+B)+D)");

}

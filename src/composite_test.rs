use crate::composite::*;

#[test]
fn structure()
{
    let mut inner = Composite::new();
    inner.push(Leaf::new("A"));
    inner.push(Leaf::new("B"));
    assert_eq!(inner.apply(),"(A+B)");

    let mut outer = Composite::new();
    outer.push(Leaf::new("C"));
    outer.push(inner);
    outer.push(Leaf::new("D"));
    assert_eq!(outer.apply(),"(C+(A+B)+D)");

}

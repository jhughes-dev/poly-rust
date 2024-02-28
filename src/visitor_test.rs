use crate::visitor::*;

struct SoundVisitor;
impl SoundVisitor {
    pub fn new() -> SoundVisitor {
        SoundVisitor
    }
}

impl Visitor for SoundVisitor {
    fn visit_cat(&self) -> String {
        "Meow".to_string()
    }

    fn visit_dog(&self) -> String {
        "Woof".to_string()
    }

    fn visit_fish(&self) -> String {
        "glub glub".to_string()
    }
}

struct PlayVisitor;

impl PlayVisitor {
    pub fn new() -> PlayVisitor {
        PlayVisitor
    }
}

impl Visitor for PlayVisitor {
    fn visit_cat(&self) -> String {
        "Play with string".to_string()
    }

    fn visit_dog(&self) -> String {
        "Play with ball".to_string()
    }

    fn visit_fish(&self) -> String {
        "Play with bubbles".to_string()
    }
}

#[test]
fn test_visitor() {
    let cat = Cat::new();
    let dog = Dog::new();
    let fish = Fish::new();

    let sound_visitor = SoundVisitor::new();
    let play_visitor = PlayVisitor::new();

    assert_eq!(cat.accept(&sound_visitor), "Meow".to_string());
    assert_eq!(dog.accept(&sound_visitor), "Woof".to_string());
    assert_eq!(fish.accept(&sound_visitor), "glub glub".to_string());

    assert_eq!(cat.accept(&play_visitor), "Play with string".to_string());
    assert_eq!(dog.accept(&play_visitor), "Play with ball".to_string());
    assert_eq!(fish.accept(&play_visitor), "Play with bubbles".to_string());
}

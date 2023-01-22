use crate::builder::*;
use rand::Rng;

struct RangeAttack {
    min: i32,
    max: i32,
}

impl Attack for RangeAttack {
    fn deal_damage(&mut self) -> i32 {
        rand::thread_rng().gen_range(self.min..self.max)
    }
}

impl RangeAttack {
    pub fn new(min: i32, max: i32) -> Self {
        RangeAttack { min, max }
    }
}

struct FixedAttack {
    strength: i32,
}

impl Attack for FixedAttack {
    fn deal_damage(&mut self) -> i32 {
        self.strength
    }
}
impl FixedAttack {
    pub fn new(strength: i32) -> Self {
        FixedAttack { strength }
    }
}
struct BubbleDefense {
    refreshed: bool,
    strength: i32,
}

impl Defense for BubbleDefense {
    fn absorb_damage(&mut self, damage: i32) -> i32 {
        let mut damage_out = damage;
        if self.refreshed {
            damage_out -= self.strength;
        }
        self.refreshed = !self.refreshed;
        std::cmp::max(0, damage_out)
    }
}

impl BubbleDefense {
    pub fn new(strength: i32) -> BubbleDefense {
        BubbleDefense {
            refreshed: true,
            strength,
        }
    }
}
struct SheildDefense {
    strength: i32,
}

impl Defense for SheildDefense {
    fn absorb_damage(&mut self, damage: i32) -> i32 {
        std::cmp::max(0, damage - self.strength)
    }
}

impl SheildDefense {
    pub fn new(strength: i32) -> SheildDefense {
        SheildDefense { strength }
    }
}

fn wizard() -> Character {
    CharacterBuilder::new()
        .set_health(30)
        .set_attack(RangeAttack::new(5, 10))
        .set_defense(BubbleDefense::new(10))
        .finalize()
}

fn warrior() -> Character {
    CharacterBuilder::new()
        .set_health(50)
        .set_attack(FixedAttack::new(7))
        .set_defense(SheildDefense::new(5))
        .finalize()
}

#[test]
fn battle() {
    let mut wizard = wizard();
    let mut warrior = warrior();

    warrior.attack_character(&mut wizard);
    assert_eq!(wizard.is_alive(), true);

    wizard.attack_character(&mut warrior);
    assert_eq!(warrior.is_alive(), true);
}

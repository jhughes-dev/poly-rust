// This is using strategy pattern for the runtime polymorphism
pub trait Attack {
    fn deal_damage(&mut self) -> i32;
}

pub trait Defense {
    fn absorb_damage(&mut self, damage: i32) -> i32;
}

pub struct Character {
    attack: Box<dyn Attack>,
    defense: Box<dyn Defense>,
    health: i32,
}

impl Character {
    pub fn take_damage(&mut self, val: i32) {
        self.health -= self.defense.absorb_damage(val);
        self.health = std::cmp::max(0, self.health);
    }
    pub fn attack_character(&mut self, other: &mut Character) {
        let damage = self.attack.deal_damage();
        other.take_damage(damage);
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
struct DefaultAttack {}

impl Attack for DefaultAttack {
    fn deal_damage(&mut self) -> i32 {
        1
    }
}
struct DefaultDefense {}

impl Defense for DefaultDefense {
    fn absorb_damage(&mut self, damage: i32) -> i32 {
        damage
    }
}

#[derive(Default)]
pub struct CharacterBuilder {
    attack: Option<Box<dyn Attack>>,
    defense: Option<Box<dyn Defense>>,
    health: Option<i32>,
}

impl CharacterBuilder {
    pub fn new() -> Self {
        CharacterBuilder::default()
    }
    pub fn set_health(mut self, health: i32) -> Self {
        self.health = Some(health);
        return self;
    }
    pub fn set_attack<T: Attack + 'static>(mut self, attack: T) -> Self {
        self.attack = Some(Box::new(attack));
        return self;
    }

    pub fn set_defense<T: Defense + 'static>(mut self, defense: T) -> Self {
        self.defense = Some(Box::new(defense));
        return self;
    }

    pub fn finalize(self) -> Character {
        Character {
            attack: self.attack.unwrap_or_else(|| Box::new(DefaultAttack {})),
            defense: self.defense.unwrap_or_else(|| Box::new(DefaultDefense {})),
            health: self.health.unwrap_or_default(),
        }
    }
}

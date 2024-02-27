use crate::factorymethod::*;

pub struct ZombieCreator;

impl ZombieCreator {
    pub fn new() -> ZombieCreator {
        ZombieCreator
    }
}

impl MonsterCreator for ZombieCreator {
    fn create_monster(&self) -> Box<dyn Monster> {
        Box::new(Zombie::new())
    }
}

pub struct Zombie;
impl Zombie {
    pub fn new() -> Zombie {
        Zombie
    }
}

impl Monster for Zombie {
    fn attack(&self) -> String {
        "Brains!".to_string()
    }
}

struct VampireCreator;
impl VampireCreator {
    pub fn new() -> VampireCreator {
        VampireCreator
    }
}

impl MonsterCreator for VampireCreator {
    fn create_monster(&self) -> Box<dyn Monster> {
        Box::new(Vampire::new())
    }
}

pub struct Vampire;
impl Vampire {
    pub fn new() -> Vampire {
        Vampire
    }
}

impl Monster for Vampire {
    fn attack(&self) -> String {
        "I want to suck your blood!".to_string()
    }
}

#[test]

fn factory_method() {
    let zombie_creator = Box::new(ZombieCreator::new());
    let zombie = zombie_creator.create_monster();
    assert_eq!(zombie.attack(), "Brains!".to_string());

    let vampire_creator = Box::new(VampireCreator::new());
    let vampire = vampire_creator.create_monster();
    assert_eq!(vampire.attack(), "I want to suck your blood!".to_string());
}

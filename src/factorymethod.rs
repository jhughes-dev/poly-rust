// Creator
pub trait MonsterCreator {
    fn create_monster(&self) -> Box<dyn Monster>;
}
// Product
pub trait Monster {
    fn attack(&self) -> String;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gender {
    Male = 0,
    Female = 1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerClass {
    Warrior,
    Mage,
    FireMage,
    Archer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Experience {
    pub level: u32,
    pub current: u64,
    pub to_next: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mana {
    pub current: u32,
    pub max: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerStats {
    pub health: Health,
    pub mana: Mana,
    pub experience: Experience,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: Health {
                current: 50,
                max: 50,
            },
            mana: Mana {
                current: 20,
                max: 20,
            },
            experience: Experience {
                level: 1,
                current: 0,
                to_next: 20,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerState {
    Moving = 0,
    Idle = 1,
    Attack = 2,
    Death = 3,
    TakingDamage = 4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id(pub u32);

pub enum Input {
    Move(Direction),
    Attack,
    Run,
    Quit,
}

pub type InputQueue = Vec<Input>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayerCharacter;

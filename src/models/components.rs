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
    Cringe = 0,
    Based = 1,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        String::from(match self {
            Gender::Cringe => "cringe",
            _ => "based",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerClass {
    Warrior,
    Mage,
    FireMage,
    Archer,
}

impl ToString for PlayerClass {
    fn to_string(&self) -> String {
        use PlayerClass::*;
        String::from(match self {
            Warrior => "warrior",
            Mage => "mage",
            FireMage => "fire_mage",
            Archer => "archer",
        })
    }
}

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Health {
//     pub current: u32,
//     pub max: u32,
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Experience {
//     pub level: u32,
//     pub current: u64,
//     pub to_next: u64,
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Mana {
//     pub current: u32,
//     pub max: u32,
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrimaryStat {
    pub current: u32,
    pub max: u32,
}

impl PrimaryStat {
    pub fn as_percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerStats {
    pub health: PrimaryStat,
    pub mana: PrimaryStat,
    pub experience: PrimaryStat,
    pub level: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: PrimaryStat {
                current: 50,
                max: 50,
            },
            mana: PrimaryStat {
                current: 20,
                max: 20,
            },
            experience: PrimaryStat {
                current: 0,
                max: 20,
            },
            level: 1,
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

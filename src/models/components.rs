#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub direction: Direction,
}

// #[derive(Component)]
// #[storage(DenseVecStorage)]
// pub struct Velocity(i32);

#[derive(Clone, Copy, Debug, PartialEq)]
// #[storage(DenseVecStorage)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
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

pub struct PlayerSprites;
impl PlayerSprites {}

pub enum Input {
    Move(Direction),
    Attack,
    Run,
    Quit,
}

// pub struct Input(pub sdl2::keyboard::Scancode);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayerCharacter;

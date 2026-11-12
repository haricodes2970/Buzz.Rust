use bevy::prelude::*;

// --- Player ---

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position {
    pub value: Vec2,
}

impl Default for Position {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
        }
    }
}

#[derive(Component)]
pub struct Grounded {
    pub value: bool,
}

impl Default for Grounded {
    fn default() -> Self {
        Self { value: false }
    }
}

// --- Power-up state ---

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerState {
    Small,
    Super,
    Fire,
    Starman,
}

impl Default for PowerState {
    fn default() -> Self {
        PowerState::Small
    }
}

#[derive(Component)]
pub struct StarmanTimer {
    pub seconds: f32,
}

impl Default for StarmanTimer {
    fn default() -> Self {
        Self { seconds: 0.0 }
    }
}

// --- Enemies ---

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Goomba;

#[derive(Component)]
pub struct KoopaTroopa;

#[derive(Component)]
pub struct PiranhaPlant;

#[derive(Component)]
pub struct Firebar;

#[derive(Component)]
pub struct BulletBill;

#[derive(Component)]
pub struct EnemyAI {
    pub patrol_left: f32,
    pub patrol_right: f32,
    pub direction: f32, // -1.0 left, 1.0 right
    pub speed: f32,
    pub patrol_timer: f32,
}

impl EnemyAI {
    pub fn new(left: f32, right: f32, speed: f32) -> Self {
        Self {
            patrol_left: left,
            patrol_right: right,
            direction: 1.0,
            speed,
            patrol_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Stompable;

#[derive(Component)]
pub struct Kickable;

// --- Items ---

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct PowerUp;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerUpType {
    SuperMushroom,
    FireFlower,
    Starman,
    OneUp,
}

#[derive(Component)]
pub struct SuperMushroom;

#[derive(Component)]
pub struct FireFlower;

#[derive(Component)]
pub struct StarmanItem;

#[derive(Component)]
pub struct OneUpMushroom;

#[derive(Component)]
pub struct Collectable {
    pub points: u32,
}

impl Collectable {
    pub fn new(points: u32) -> Self {
        Self { points }
    }
}

// --- Projectiles ---

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

impl Projectile {
    pub fn new(direction: Vec2, speed: f32, lifetime: f32) -> Self {
        Self {
            direction,
            speed,
            lifetime,
        }
    }
}

// --- Level end ---

#[derive(Component)]
pub struct Flagpole;

#[derive(Component)]
pub struct Flag;

// --- Camera ---

#[derive(Component)]
pub struct CameraFollow;

// --- Misc ---

#[derive(Component)]
pub struct LevelBoundary {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

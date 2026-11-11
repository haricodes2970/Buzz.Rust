use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub gravity: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_velocity: f32,
    pub fall_gravity_multiplier: f32,
    pub max_fall_speed: f32,
    pub coyote_time: f32,
    pub jump_buffer_time: f32,
    pub tile_size: f32,
    pub level_width: f32,
    pub level_height: f32,
    pub level_time_limit: f32, // seconds
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            gravity: 1500.0,
            walk_speed: 120.0,
            run_speed: 200.0,
            jump_velocity: -500.0,
            fall_gravity_multiplier: 1.8,
            max_fall_speed: 600.0,
            coyote_time: 0.1,
            jump_buffer_time: 0.1,
            tile_size: 32.0,
            level_width: 3200.0,
            level_height: 720.0,
            level_time_limit: 300.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct Lives {
    pub value: u32,
}

#[derive(Resource)]
pub struct TimerResource {
    pub seconds: f32,
    pub time_limit: f32,
    pub expired: bool,
}

impl Default for TimerResource {
    fn default() -> Self {
        Self {
            seconds: 300.0,
            time_limit: 300.0,
            expired: false,
        }
    }
}

#[derive(Resource, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerUpState {
    #[default]
    Small,
    Super,
    Fire,
    Starman,
}

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub index: u32, // 1-based
}

#[derive(Resource, Default)]
pub struct Xp {
    pub value: u32,
    pub level: u32,
}

#[derive(Resource, Default)]
pub struct Coins {
    pub value: u32,
}

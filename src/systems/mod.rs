pub mod input;
pub mod movement;
pub mod physics;
pub mod camera;
pub mod enemy;
pub mod audio;
pub mod gameplay;

use bevy::prelude::*;
use crate::GameState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Input (handles menu + gameplay)
        app.add_systems(Update, input::handle_input);

        // Movement & physics
        app.add_systems(
            Update,
            (
                movement::apply_gravity,
                movement::apply_movement,
                movement::resolve_collisions,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );

        // Gameplay: timer, coin collection, power-up collection, enemy stomp
        app.add_systems(
            Update,
            (
                gameplay::update_timer,
                gameplay::collect_coins,
                gameplay::collect_powerup,
                gameplay::stomp_enemy,
            )
                .run_if(in_state(GameState::Playing)),
        );

        // Camera
        app.add_systems(
            Update,
            camera::camera_follow.run_if(in_state(GameState::Playing)),
        );

        // Enemy AI
        app.add_systems(Update, enemy::enemy_ai.run_if(in_state(GameState::Playing)));

        // Audio
        app.add_systems(Update, audio::play_audio.run_if(in_state(GameState::Playing)));
    }
}

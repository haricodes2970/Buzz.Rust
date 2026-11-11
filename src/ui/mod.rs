pub mod hud;
pub mod menu;

use bevy::prelude::*;
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // HUD
        app.add_systems(OnEnter(GameState::Playing), hud::setup_hud)
            .add_systems(OnExit(GameState::Playing), hud::cleanup_hud)
            .add_systems(Update, hud::update_hud.run_if(in_state(GameState::Playing)));
    }
}

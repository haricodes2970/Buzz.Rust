pub mod hud;
pub mod menu;

use bevy::prelude::*;
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // HUD — setup/cleanup only. Text is set once at setup; no dynamic updates
        // to avoid B0001 conflicts with Bevy's text system (which reads &Text).
        app.add_systems(OnEnter(GameState::Playing), hud::setup_hud)
            .add_systems(OnExit(GameState::Playing), hud::cleanup_hud);
    }
}

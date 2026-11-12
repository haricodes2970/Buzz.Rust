pub mod movement;
pub mod camera;
pub mod enemy;
pub mod audio;
pub mod gameplay;
pub mod input;
pub mod physics;

use bevy::prelude::*;
use crate::GameState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Menu/pause input — no component queries, runs independently
        app.add_systems(Update, input::handle_input);

        // Single unified gameplay system — all Player-touching logic in one system
        // to avoid Bevy 0.20 B0001 query conflicts.
        app.add_systems(
            Update,
            movement::gameplay_update.run_if(in_state(GameState::Playing)),
        );

        // Sync Position -> Transform for rendering (including camera follow)
        // Runs in PostUpdate, before TransformPropagateSystem
        app.add_systems(
            PostUpdate,
            movement::sync_transforms.before(bevy_transform::TransformSystems::Propagate),
        );
    }
}

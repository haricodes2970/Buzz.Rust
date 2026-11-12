pub mod movement;

use bevy::prelude::*;
use bevy_transform::TransformSystems;
use crate::GameState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Menu/pause input — no Player component queries, runs independently
        app.add_systems(Update, movement::handle_input);

        // Single unified gameplay system — all Player-touching logic in one system
        // to avoid Bevy 0.20 B0001 query conflicts. Uses Position (custom component)
        // instead of Transform for the player.
        app.add_systems(
            Update,
            movement::gameplay_update.run_if(in_state(GameState::Playing)),
        );

        // Sync Position -> Transform for rendering. Runs in PostUpdate, before
        // TransformPropagateSystem, so Bevy's renderer sees updated Transform values.
        app.add_systems(
            PostUpdate,
            movement::sync_transforms.before(TransformSystems::Propagate),
        );
    }
}

use bevy::prelude::*;
use crate::game::components::*;

pub fn camera_follow(
    mut camera_query: Query<(&mut Transform, &CameraFollow), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut camera_transform, _) in camera_query.iter_mut() {
        let target_position = player_transform.translation;
        let current_position = camera_transform.translation;

        // Exponential smoothing: frame-rate independent
        let rate = 8.0;
        let t = 1.0 - (-rate * time.delta_secs()).exp();

        let mut new_position = current_position.lerp(target_position, t);
        new_position.z = current_position.z; // Keep camera Z

        // Clamp camera to level bounds (keep player centered within bounds)
        // Level bounds: -1600 to +1600 X, -400 to +400 Y approximately
        let view_width = 1280.0 / 2.0;
        let view_height = 720.0 / 2.0;
        let level_left = -1600.0 + view_width;
        let level_right = 1600.0 - view_width;
        let level_bottom = -400.0 + view_height;
        let level_top = 400.0 - view_height;

        new_position.x = new_position.x.clamp(level_left, level_right);
        new_position.y = new_position.y.clamp(level_bottom, level_top);

        camera_transform.translation = new_position;
    }
}

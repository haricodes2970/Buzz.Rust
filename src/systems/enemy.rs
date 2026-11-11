use bevy::prelude::*;
use crate::game::components::*;

pub fn enemy_ai(
    mut enemy_query: Query<(&mut Transform, &mut Velocity, &mut EnemyAI), With<Goomba>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut ai) in enemy_query.iter_mut() {
        // Move in current direction
        transform.translation.x += ai.direction * ai.speed * time.delta_secs();
        velocity.value.x = ai.direction * ai.speed;

        // Check patrol boundaries and turn around
        if ai.direction > 0.0 && transform.translation.x >= ai.patrol_right {
            ai.direction = -1.0;
            transform.translation.x = ai.patrol_right;
        } else if ai.direction < 0.0 && transform.translation.x <= ai.patrol_left {
            ai.direction = 1.0;
            transform.translation.x = ai.patrol_left;
        }
    }
}

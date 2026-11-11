use bevy::prelude::*;
use crate::game::components::*;
use crate::game::resources::*;
use crate::GameState;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Velocity, &mut Grounded, &PowerState), With<Player>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    // Handle menu input
    if current_state.get() == &GameState::MainMenu {
        if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::Playing);
        }
        return;
    }

    // Handle pause toggle
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }

    // Only process gameplay input when playing
    if current_state.get() != &GameState::Playing {
        return;
    }

    let Ok((mut velocity, mut grounded, _power_state)) = player_query.single_mut() else {
        return;
    };

    // Horizontal input
    let mut axis = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        axis -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        axis += 1.0;
    }

    let speed = if keyboard.pressed(KeyCode::ShiftLeft) {
        config.run_speed
    } else {
        config.walk_speed
    };

    let target_velocity = axis * speed;
    let accel = if grounded.value { 2000.0 } else { 1200.0 };
    velocity.value.x = lerp(velocity.value.x, target_velocity, accel * time.delta_secs());

    // Jump input
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW) {
        velocity.value.y = config.jump_velocity;
        grounded.value = false;
    }

    // Variable jump height - release to cut
    if (keyboard.just_released(KeyCode::Space) || keyboard.just_released(KeyCode::KeyW))
        && velocity.value.y < 0.0
    {
        velocity.value.y *= 0.5;
    }
}

fn lerp(current: f32, target: f32, factor: f32) -> f32 {
    current + (target - current) * factor.clamp(0.0, 1.0)
}

pub fn apply_gravity(
    mut player_query: Query<(&mut Velocity, &Grounded), With<Player>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let Ok((mut velocity, grounded)) = player_query.single_mut() else {
        return;
    };

    if !grounded.value {
        let mut gravity = config.gravity;
        if velocity.value.y > 0.0 {
            gravity *= config.fall_gravity_multiplier;
        }
        velocity.value.y += gravity * time.delta_secs();

        if velocity.value.y > config.max_fall_speed {
            velocity.value.y = config.max_fall_speed;
        }
    }
}

pub fn apply_movement(
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in player_query.iter_mut() {
        transform.translation += velocity.value.extend(0.0) * time.delta_secs();
    }
}

pub fn resolve_collisions(
    mut player_query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Grounded,
        &Collider,
    ),
    With<Player>,
>,
    platform_query: Query<(&Transform, &Collider), Without<Player>>,
) {
    for (mut player_transform, mut player_velocity, mut grounded, player_collider) in
        player_query.iter_mut()
    {
        grounded.value = false;
        let player_bottom = player_transform.translation.y - player_collider.size.y / 2.0;
        let player_top = player_transform.translation.y + player_collider.size.y / 2.0;
        let player_left = player_transform.translation.x - player_collider.size.x / 2.0;
        let player_right = player_transform.translation.x + player_collider.size.x / 2.0;

        for (platform_transform, platform_collider) in platform_query.iter() {
            let plat_bottom = platform_transform.translation.y - platform_collider.size.y / 2.0;
            let plat_top = platform_transform.translation.y + platform_collider.size.y / 2.0;
            let plat_left = platform_transform.translation.x - platform_collider.size.x / 2.0;
            let plat_right = platform_transform.translation.x + platform_collider.size.x / 2.0;

            let overlap_x = player_right.max(plat_left) - player_left.min(plat_right);
            let overlap_y = player_bottom.max(plat_top) - player_top.min(plat_bottom);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                if player_velocity.value.y > 0.0 && player_bottom <= plat_top + 2.0 {
                    player_transform.translation.y =
                        plat_top + player_collider.size.y / 2.0 + 0.1;
                    player_velocity.value.y = 0.0;
                    grounded.value = true;
                } else if player_velocity.value.y < 0.0 && player_top >= plat_bottom - 2.0 {
                    player_transform.translation.y =
                        plat_bottom - player_collider.size.y / 2.0 - 0.1;
                    player_velocity.value.y = 0.0;
                }

                if player_velocity.value.x > 0.0 && player_right <= plat_left + 2.0 {
                    player_transform.translation.x =
                        plat_left - player_collider.size.x / 2.0 - 0.1;
                    player_velocity.value.x = 0.0;
                } else if player_velocity.value.x < 0.0 && player_left >= plat_right - 2.0 {
                    player_transform.translation.x =
                        plat_right + player_collider.size.x / 2.0 + 0.1;
                    player_velocity.value.x = 0.0;
                }
            }
        }
    }
}

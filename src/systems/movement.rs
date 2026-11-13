use bevy::prelude::*;
use crate::assets::{AnimationTimer, GameAssets};
use crate::game::components::*;
use crate::game::resources::*;
use crate::GameState;

fn lerp(current: f32, target: f32, factor: f32) -> f32 {
    current + (target - current) * factor.clamp(0.0, 1.0)
}

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let state = *current_state.get();

    if state == GameState::MainMenu || state == GameState::GameOver || state == GameState::LevelComplete {
        if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::Playing);
        }
    }

    if state == GameState::Playing || state == GameState::Paused {
        if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Enter) {
            if state == GameState::Playing {
                next_state.set(GameState::Paused);
            } else {
                next_state.set(GameState::Playing);
            }
        }
    }
}

pub fn gameplay_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(
        &mut Position,
        &mut Velocity,
        &mut Grounded,
        &mut Collider,
        &mut PowerState,
        &mut AnimationTimer,
        &mut Sprite,
    ),
    With<Player>,
>,
    collidable_query: Query<(
        Entity,
        &Transform,
        &Collider,
        Option<&Coin>,
        Option<&PowerUpType>,
        Option<&Enemy>,
        Option<&Stompable>,
    ),
    Without<Player>,
>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    mut coins: ResMut<Coins>,
    mut power_up_state: ResMut<PowerUpState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<TimerResource>,
    time: Res<Time>,
    mut commands: Commands,
    mut goomba_query: Query<(&mut Position, &mut Velocity, &mut EnemyAI), (With<Goomba>, Without<CameraFollow>, Without<Player>)>,
) {
    let Ok((
        mut position,
        mut velocity,
        mut grounded,
        mut collider,
        mut power_state,
        mut anim,
        mut sprite,
    )) = player_query.single_mut()
    else {
        return;
    };

    // === INPUT ===
    let mut axis = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        axis -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        axis += 1.0;
    }

    let speed = if keyboard.pressed(KeyCode::ShiftLeft) {
        200.0
    } else {
        120.0
    };

    let target_velocity = axis * speed;
    let accel = if grounded.value { 2000.0 } else { 1200.0 };
    velocity.value.x = lerp(velocity.value.x, target_velocity, accel * time.delta_secs());

    // Jump
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        if grounded.value {
            velocity.value.y = -500.0;
            grounded.value = false;
        }
    }

    // Variable jump height - release early = shorter jump
    if (keyboard.just_released(KeyCode::Space) || keyboard.just_released(KeyCode::KeyW) || keyboard.just_released(KeyCode::ArrowUp))
        && velocity.value.y < 0.0
    {
        velocity.value.y *= 0.5;
    }

    // === GRAVITY ===
    if !grounded.value {
        let mut gravity = 1500.0;
        if velocity.value.y > 0.0 {
            gravity *= 1.8; // fall faster than rise
        }
        velocity.value.y += gravity * time.delta_secs();
        if velocity.value.y > 600.0 {
            velocity.value.y = 600.0;
        }
    }

    // === MOVEMENT ===
    position.value.x += velocity.value.x * time.delta_secs();
    position.value.y += velocity.value.y * time.delta_secs();

    // === ANIMATION ===
    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
        anim.current = (anim.current + 1) % anim.frames.len();
    }

    // Determine animation frame
    let frame_set = if !grounded.value {
        if anim.frames.len() > 2 {
            &anim.frames[2] // jump frame
        } else {
            &anim.frames[anim.current % anim.frames.len()]
        }
    } else if axis.abs() > 0.01 {
        &anim.frames[anim.current % anim.frames.len()] // running
    } else {
        &anim.frames[0] // idle
    };

    sprite.image = frame_set.clone();

    // Flip sprite based on direction
    if axis < -0.01 {
        sprite.flip_x = true;
    } else if axis > 0.01 {
        sprite.flip_x = false;
    }

    // === COLLISIONS ===
    // Bevy 2D: Y increases DOWNWARD
    // Top of object = y - size.y/2
    // Bottom of object = y + size.y/2
    grounded.value = false;

    let player_bottom = position.value.y + collider.size.y / 2.0;  // bottom edge
    let player_top = position.value.y - collider.size.y / 2.0;     // top edge
    let player_left = position.value.x - collider.size.x / 2.0;    // left edge
    let player_right = position.value.x + collider.size.x / 2.0;   // right edge

    for (entity, col_transform, col_collider, coin, power_up, enemy, stompable) in
        collidable_query.iter()
    {
        let col_bottom = col_transform.translation.y + col_collider.size.y / 2.0;
        let col_top = col_transform.translation.y - col_collider.size.y / 2.0;
        let col_left = col_transform.translation.x - col_collider.size.x / 2.0;
        let col_right = col_transform.translation.x + col_collider.size.x / 2.0;

        let overlap_x = player_right.max(col_left) - player_left.min(col_right);
        let overlap_y = player_bottom.max(col_top) - player_top.min(col_bottom);

        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            continue;
        }

        // Platform collision (non-collectible, non-enemy)
        if coin.is_none() && power_up.is_none() && enemy.is_none() {
            // Calculate which side we hit based on overlap
            // We resolve along the axis of least overlap

            // Check if we hit from above (falling onto platform)
            let from_above = player_bottom - col_top;
            let from_below = col_bottom - player_top;
            let from_left = player_right - col_left;
            let from_right = col_right - player_left;

            let min_overlap = overlap_x.min(overlap_y);

            if min_overlap == overlap_x {
                // Horizontal collision - hit from left or right
                if from_left < from_right {
                    // Hit left side of platform
                    position.value.x = col_left - collider.size.x / 2.0 - 0.1;
                } else {
                    // Hit right side of platform
                    position.value.x = col_right + collider.size.x / 2.0 + 0.1;
                }
                velocity.value.x = 0.0;
            } else {
                // Vertical collision - hit from top or bottom
                if from_above < from_below {
                    // Hit from above (landing on platform)
                    position.value.y = col_top - collider.size.y / 2.0 - 0.1;
                    velocity.value.y = 0.0;
                    grounded.value = true;
                } else {
                    // Hit from below (hit head on platform)
                    position.value.y = col_bottom + collider.size.y / 2.0 + 0.1;
                    velocity.value.y = 0.0;
                }
            }
            continue;
        }

        // Coin collection
        if coin.is_some() {
            commands.entity(entity).despawn();
            score.value += 10;
            coins.value += 1;
        }

        // Power-up collection
        if let Some(pu_type) = power_up {
            match pu_type {
                PowerUpType::SuperMushroom => *power_state = PowerState::Super,
                PowerUpType::FireFlower => *power_state = PowerState::Fire,
                PowerUpType::Starman => *power_state = PowerState::Starman,
                PowerUpType::OneUp => *power_state = PowerState::Super,
            }

            match *power_state {
                PowerState::Super => collider.size = Vec2::new(24.0, 52.0),
                PowerState::Small => collider.size = Vec2::new(24.0, 36.0),
                _ => {}
            }

            score.value += 50;
            commands.entity(entity).despawn();
        }

        // Enemy stomp
        if enemy.is_some()
            && stompable.is_some()
            && velocity.value.y > 0.0
            && player_bottom <= col_top + 5.0
            && player_bottom > col_top - 20.0
        {
            commands.entity(entity).despawn();
            velocity.value.y = -300.0;
            score.value += 100;
        }
    }

    // === BUZZ TIMER ===
    if !timer.expired {
        timer.seconds -= time.delta_secs();
        if timer.seconds <= 0.0 {
            timer.seconds = 0.0;
            timer.expired = true;

            if lives.value > 0 {
                lives.value -= 1;
            }

            if lives.value == 0 {
                next_state.set(GameState::GameOver);
            } else {
                timer.seconds = timer.time_limit;
                timer.expired = false;
            }
        }
    }

    // === ENEMY AI (Goomba patrol) ===
    for (mut g_position, mut g_velocity, mut ai) in goomba_query.iter_mut() {
        g_position.value.x += ai.direction * ai.speed * time.delta_secs();
        g_velocity.value.x = ai.direction * ai.speed;

        if ai.direction > 0.0 && g_position.value.x >= ai.patrol_right {
            ai.direction = -1.0;
            g_position.value.x = ai.patrol_right;
        } else if ai.direction < 0.0 && g_position.value.x <= ai.patrol_left {
            ai.direction = 1.0;
            g_position.value.x = ai.patrol_left;
        }
    }
}

// --- Sync Position -> Transform for rendering ---
pub fn sync_transforms(
    mut query: Query<(&Position, &mut Transform, Option<&CameraFollow>)>,
    player_query: Query<&Position, With<Player>>,
    time: Res<Time>,
) {
    let player_pos = player_query.single().ok();

    for (position, mut transform, camera) in query.iter_mut() {
        if camera.is_some() {
            // Camera: smooth-follow the player
            if let Some(pp) = player_pos {
                let rate = 8.0;
                let dt = time.delta_secs();
                let t = 1.0 - (-rate * dt).exp();
                transform.translation.x = transform.translation.x.lerp(pp.value.x, t);
                transform.translation.y = transform.translation.y.lerp(pp.value.y, t);
            }
        } else {
            transform.translation.x = position.value.x;
            transform.translation.y = position.value.y;
        }
    }
}
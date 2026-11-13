use bevy::prelude::*;
use crate::game::components::*;
use crate::game::resources::*;
use crate::GameState;

// --- Buzz Timer ---

pub fn update_timer(
    mut timer: ResMut<TimerResource>,
    time: Res<Time>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.expired {
        return;
    }

    timer.seconds -= time.delta_secs();
    if timer.seconds <= 0.0 {
        timer.seconds = 0.0;
        timer.expired = true;

        // Lose a life
        if lives.value > 0 {
            lives.value -= 1;
        }

        if lives.value == 0 {
            next_state.set(GameState::GameOver);
        } else {
            // Restart level - reset timer
            timer.seconds = timer.time_limit;
            timer.expired = false;
        }
    }
}

// --- Coin Collection ---

pub fn collect_coins(
    mut commands: Commands,
    player_query: Query<(&Position, &Collider), With<Player>>,
    coin_query: Query<(Entity, &Transform, &Collider), With<Coin>>,
    mut score: ResMut<Score>,
    mut coins: ResMut<Coins>,
) {
    let Ok((player_position, player_collider)) = player_query.single() else {
        return;
    };

    let player_bottom = player_position.value.y - player_collider.size.y / 2.0;
    let player_top = player_position.value.y + player_collider.size.y / 2.0;
    let player_left = player_position.value.x - player_collider.size.x / 2.0;
    let player_right = player_position.value.x + player_collider.size.x / 2.0;

    for (entity, coin_transform, coin_collider) in coin_query.iter() {
        let coin_bottom = coin_transform.translation.y - coin_collider.size.y / 2.0;
        let coin_top = coin_transform.translation.y + coin_collider.size.y / 2.0;
        let coin_left = coin_transform.translation.x - coin_collider.size.x / 2.0;
        let coin_right = coin_transform.translation.x + coin_collider.size.x / 2.0;

        let overlap_x = player_right.max(coin_left) - player_left.min(coin_right);
        let overlap_y = player_bottom.max(coin_top) - player_top.min(coin_bottom);

        if overlap_x > 0.0 && overlap_y > 0.0 {
            commands.entity(entity).despawn();
            score.value += 10;
            coins.value += 1;
        }
    }
}

// --- Power-up Collection ---

pub fn collect_powerup(
    mut commands: Commands,
    mut player_query: Query<(&mut Position, &mut Collider, &mut PowerState), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &Collider, &PowerUpType)>,
    mut power_up_state: ResMut<PowerUpState>,
    mut score: ResMut<Score>,
) {
    if let Ok((mut player_position, mut player_collider, mut player_power)) =
        player_query.single_mut()
    {
        let player_bottom = player_position.value.y - player_collider.size.y / 2.0;
        let player_top = player_position.value.y + player_collider.size.y / 2.0;
        let player_left = player_position.value.x - player_collider.size.x / 2.0;
        let player_right = player_position.value.x + player_collider.size.x / 2.0;

        for (entity, pu_transform, pu_collider, pu_type) in powerup_query.iter() {
            let pu_bottom = pu_transform.translation.y - pu_collider.size.y / 2.0;
            let pu_top = pu_transform.translation.y + pu_collider.size.y / 2.0;
            let pu_left = pu_transform.translation.x - pu_collider.size.x / 2.0;
            let pu_right = pu_transform.translation.x + pu_collider.size.x / 2.0;

            let overlap_x = player_right.max(pu_left) - player_left.min(pu_right);
            let overlap_y = player_bottom.max(pu_top) - player_top.min(pu_bottom);

            if overlap_x > 0.0 && overlap_y > 0.0 {
                commands.entity(entity).despawn();

                let new_state = match pu_type {
                    PowerUpType::SuperMushroom => PowerState::Super,
                    PowerUpType::FireFlower => PowerState::Fire,
                    PowerUpType::Starman => PowerState::Starman,
                    PowerUpType::OneUp => PowerState::Super,
                };

                *player_power = new_state;
                *power_up_state = match new_state {
                    PowerState::Small => PowerUpState::Small,
                    PowerState::Super => PowerUpState::Super,
                    PowerState::Fire => PowerUpState::Fire,
                    PowerState::Starman => PowerUpState::Starman,
                };

                // Resize player collider for Super state
                match new_state {
                    PowerState::Super => {
                        player_collider.size = Vec2::new(24.0, 52.0);
                    }
                    PowerState::Small => {
                        player_collider.size = Vec2::new(24.0, 36.0);
                    }
                    _ => {}
                }

                score.value += 50;
            }
        }
    }
}

// --- Enemy Stomp ---

pub fn stomp_enemy(
    mut commands: Commands,
    mut player_query: Query<(&mut Velocity, &Position, &Collider), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Collider), (With<Enemy>, With<Stompable>)>,
    mut score: ResMut<Score>,
) {
    let Ok((mut velocity, player_position, player_collider)) = player_query.single_mut() else {
        return;
    };

    // Only stomp if falling
    if velocity.value.y <= 0.0 {
        return;
    }

    let player_bottom = player_position.value.y - player_collider.size.y / 2.0;
    let player_left = player_position.value.x - player_collider.size.x / 2.0;
    let player_right = player_position.value.x + player_collider.size.x / 2.0;

    for (entity, enemy_transform, enemy_collider) in enemy_query.iter() {
        let enemy_top = enemy_transform.translation.y + enemy_collider.size.y / 2.0;
        let enemy_left = enemy_transform.translation.x - enemy_collider.size.x / 2.0;
        let enemy_right = enemy_transform.translation.x + enemy_collider.size.x / 2.0;

        // Player must be above enemy and overlapping horizontally
        if player_bottom <= enemy_top + 5.0
            && player_bottom > enemy_top - 20.0
            && player_right > enemy_left
            && player_left < enemy_right
        {
            commands.entity(entity).despawn();
            velocity.value.y = -300.0; // Bounce
            score.value += 100;
        }
    }
}

// --- Level End Detection ---

pub fn check_level_end(
    mut commands: Commands,
    player_query: Query<(&Position, &Collider), With<Player>>,
    trigger_query: Query<&Transform, (With<Name>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<TimerResource>,
) {
    let Ok((player_position, player_collider)) = player_query.single() else {
        return;
    };

    let player_bottom = player_position.value.y - player_collider.size.y / 2.0;
    let player_top = player_position.value.y + player_collider.size.y / 2.0;
    let player_left = player_position.value.x - player_collider.size.x / 2.0;
    let player_right = player_position.value.x + player_collider.size.x / 2.0;

    for trigger_transform in trigger_query.iter() {
        // Only check the level end trigger (named "LevelEndTrigger")
        if trigger_transform.translation.x < 1400.0 {
            continue;
        }

        let trigger_left = trigger_transform.translation.x - 20.0;
        let trigger_right = trigger_transform.translation.x + 20.0;
        let trigger_top = trigger_transform.translation.y + 200.0;
        let trigger_bottom = trigger_transform.translation.y - 200.0;

        if player_right > trigger_left
            && player_left < trigger_right
            && player_top > trigger_bottom
            && player_bottom < trigger_top
        {
            timer.expired = true;
            next_state.set(GameState::LevelComplete);
        }
    }
}
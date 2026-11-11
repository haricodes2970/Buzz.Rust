use bevy::audio::{AudioPlayer, AudioSource};
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
            commands.spawn((
                AudioPlayer::<AudioSource>(asset_server.load("audio/death.wav")),
                PlaybackSettings::DESPAWN,
            ));
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
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &Collider), With<Player>>,
    coin_query: Query<(Entity, &Transform, &Collider), With<Coin>>,
    mut score: ResMut<Score>,
    mut coins: ResMut<Coins>,
) {
    let Ok((player_transform, player_collider)) = player_query.single() else {
        return;
    };

    let player_bottom = player_transform.translation.y - player_collider.size.y / 2.0;
    let player_top = player_transform.translation.y + player_collider.size.y / 2.0;
    let player_left = player_transform.translation.x - player_collider.size.x / 2.0;
    let player_right = player_transform.translation.x + player_collider.size.x / 2.0;

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
            commands.spawn((
                AudioPlayer::<AudioSource>(asset_server.load("audio/coin.wav")),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

// --- Power-up Collection ---

pub fn collect_powerup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Transform, &mut Collider, &mut PowerState), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &Collider, &PowerUpType)>,
    mut power_up_state: ResMut<PowerUpState>,
    mut score: ResMut<Score>,
) {
    if let Ok((mut player_transform, mut player_collider, mut player_power)) =
        player_query.single_mut()
    {
        let player_bottom = player_transform.translation.y - player_collider.size.y / 2.0;
        let player_top = player_transform.translation.y + player_collider.size.y / 2.0;
        let player_left = player_transform.translation.x - player_collider.size.x / 2.0;
        let player_right = player_transform.translation.x + player_collider.size.x / 2.0;

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
                score.value += 50;
                commands.spawn((
                    AudioPlayer::<AudioSource>(asset_server.load("audio/powerup.wav")),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}

// --- Enemy Stomp ---

pub fn stomp_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Velocity, &Transform, &Collider), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Collider), (With<Enemy>, With<Stompable>)>,
    mut score: ResMut<Score>,
) {
    let Ok((mut velocity, player_transform, player_collider)) = player_query.single_mut() else {
        return;
    };

    // Only stomp if falling
    if velocity.value.y <= 0.0 {
        return;
    }

    let player_bottom = player_transform.translation.y - player_collider.size.y / 2.0;
    let player_left = player_transform.translation.x - player_collider.size.x / 2.0;
    let player_right = player_transform.translation.x + player_collider.size.x / 2.0;

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
            commands.spawn((
                AudioPlayer::<AudioSource>(asset_server.load("audio/stomp.wav")),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

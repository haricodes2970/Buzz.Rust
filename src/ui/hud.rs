use bevy::prelude::*;
use crate::game::resources::*;
use crate::GameState;

#[derive(Component)]
pub struct HudScore;

#[derive(Component)]
pub struct HudCoins;

#[derive(Component)]
pub struct HudLives;

#[derive(Component)]
pub struct HudTimer;

#[derive(Component)]
pub struct HudEntity;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            HudEntity,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                HudScore,
                Text::new("Score: 0"),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudCoins,
                Text::new("Coins: 0"),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudLives,
                Text::new("Lives: 3"),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudTimer,
                Text::new("Time: 300"),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
        });
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_hud(
    score: Res<Score>,
    coins: Res<Coins>,
    lives: Res<Lives>,
    timer: Res<TimerResource>,
    mut score_query: Query<&mut Text, With<HudScore>>,
    mut coins_query: Query<&mut Text, With<HudCoins>>,
    mut lives_query: Query<&mut Text, With<HudLives>>,
    mut timer_query: Query<&mut Text, With<HudTimer>>,
) {
    if let Ok(mut text) = score_query.single_mut() {
        text.0 = format!("Score: {}", score.value);
    }
    if let Ok(mut text) = coins_query.single_mut() {
        text.0 = format!("Coins: {}", coins.value);
    }
    if let Ok(mut text) = lives_query.single_mut() {
        text.0 = format!("Lives: {}", lives.value);
    }
    if let Ok(mut text) = timer_query.single_mut() {
        text.0 = format!("Time: {}", timer.seconds as u32);
    }
}

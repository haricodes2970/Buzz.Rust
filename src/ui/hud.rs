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

pub fn setup_hud(
    mut commands: Commands,
    score: Res<Score>,
    coins: Res<Coins>,
    lives: Res<Lives>,
    timer: Res<TimerResource>,
) {
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
                Text::new(format!("Score: {}", score.value)),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudCoins,
                Text::new(format!("Coins: {}", coins.value)),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudLives,
                Text::new(format!("Lives: {}", lives.value)),
                TextFont {
                    font_size: bevy::text::FontSize::Px(24.0),
                    ..default()
                },
                TextColor::WHITE,
            ));
            parent.spawn((
                HudTimer,
                Text::new(format!("Time: {}", timer.seconds as u32)),
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

use bevy::prelude::*;
use crate::game::MainMenuEntity;
use crate::GameState;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_main_menu(mut commands: Commands) {
    // Camera for UI
    commands.spawn((Camera2d, MainMenuEntity));

    // Title text
    commands.spawn((
        MainMenuEntity,
        Text::new("BUZZ"),
        TextFont {
            font_size: bevy::text::FontSize::Px(120.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));

    // Story text
    commands.spawn((
        MainMenuEntity,
        Text::new("Hari must run through 50 connected levels to rescue his friend Chethan from the gangster. Complete each level before the \"buzz\" timer rings."),
        TextFont {
            font_size: bevy::text::FontSize::Px(24.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(250.0),
            left: Val::Px(50.0),
            width: Val::Px(600.0),
            ..default()
        },
    ));

    // Play hint text (keyboard-activated)
    commands.spawn((
        MainMenuEntity,
        Text::new("Press SPACE to Play"),
        TextFont {
            font_size: bevy::text::FontSize::Px(32.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(420.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- Game Over Screen ---

#[derive(Component)]
pub struct GameOverEntity;

pub fn setup_game_over(mut commands: Commands) {
    commands.spawn((Camera2d, GameOverEntity));
    commands.spawn((
        GameOverEntity,
        Text::new("GAME OVER"),
        TextFont {
            font_size: bevy::text::FontSize::Px(100.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
    commands.spawn((
        GameOverEntity,
        Text::new("Press SPACE to try again"),
        TextFont {
            font_size: bevy::text::FontSize::Px(32.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- Level Complete Screen ---

#[derive(Component)]
pub struct LevelCompleteEntity;

pub fn setup_level_complete(mut commands: Commands) {
    commands.spawn((Camera2d, LevelCompleteEntity));
    commands.spawn((
        LevelCompleteEntity,
        Text::new("LEVEL COMPLETE!"),
        TextFont {
            font_size: bevy::text::FontSize::Px(100.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
    commands.spawn((
        LevelCompleteEntity,
        Text::new("Press SPACE to continue"),
        TextFont {
            font_size: bevy::text::FontSize::Px(32.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

pub fn cleanup_level_complete(
    mut commands: Commands,
    query: Query<Entity, With<LevelCompleteEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- Pause Screen ---

#[derive(Component)]
pub struct PauseEntity;

pub fn setup_pause(mut commands: Commands) {
    commands.spawn((Camera2d, PauseEntity));
    commands.spawn((
        PauseEntity,
        Text::new("PAUSED"),
        TextFont {
            font_size: bevy::text::FontSize::Px(100.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
    commands.spawn((
        PauseEntity,
        Text::new("Press ESC to resume"),
        TextFont {
            font_size: bevy::text::FontSize::Px(32.0),
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

pub fn cleanup_pause(mut commands: Commands, query: Query<Entity, With<PauseEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::game::MainMenuEntity;
use crate::GameState;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    // Camera for UI
    commands.spawn((Camera2d, MainMenuEntity));

    // Background - tile background across screen
    for x in 0..(1280 / 18 + 2) {
        for y in 0..(720 / 18 + 2) {
            commands.spawn((
                MainMenuEntity,
                Sprite {
                    image: assets.ground_tile.clone(),
                    ..default()
                },
                Transform::from_xyz(x as f32 * 18.0, y as f32 * 18.0, 0.0),
                Name::new("MenuBackground"),
            ));
        }
    }

    // Title text "BUZZ" - large, centered
    commands.spawn((
        MainMenuEntity,
        Text::new("BUZZ"),
        TextFont {
            font_size: bevy::text::FontSize::Px(120.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.15, 0.15)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(500.0),
            ..default()
        },
        Name::new("TitleText"),
    ));

    // Shadow for title
    commands.spawn((
        MainMenuEntity,
        Text::new("BUZZ"),
        TextFont {
            font_size: bevy::text::FontSize::Px(96.0),
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(86.0),
            left: Val::Px(504.0),
            ..default()
        },
    ));

    // Story text
    commands.spawn((
        MainMenuEntity,
        Text::new("Hari must run through 50 connected levels"),
        TextFont {
            font_size: bevy::text::FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(240.0),
            left: Val::Px(380.0),
            width: Val::Px(520.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuEntity,
        Text::new("to rescue his friend Chethan from the gangster."),
        TextFont {
            font_size: bevy::text::FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(270.0),
            left: Val::Px(380.0),
            width: Val::Px(520.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuEntity,
        Text::new("Complete each level before the \"buzz\" timer rings!"),
        TextFont {
            font_size: bevy::text::FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(380.0),
            width: Val::Px(520.0),
            ..default()
        },
    ));

    // Play hint text - pulsating
    commands.spawn((
        MainMenuEntity,
        Text::new("PRESS SPACE TO PLAY"),
        TextFont {
            font_size: bevy::text::FontSize::Px(40.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.1)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(400.0),
            left: Val::Px(440.0),
            ..default()
        },
        Name::new("PlayHint"),
    ));

    // Player sprite preview
    let player_img = assets.player_idle.first().cloned().unwrap_or_default();
    commands.spawn((
        MainMenuEntity,
        Sprite {
            image: player_img,
            ..default()
        },
        Transform::from_xyz(320.0, 360.0, 1.0),
        Name::new("TitlePlayer"),
    ));

    // Koomba enemy preview
    let goomba_img = assets.goomba.first().cloned().unwrap_or_default();
    commands.spawn((
        MainMenuEntity,
        Sprite {
            image: goomba_img,
            ..default()
        },
        Transform::from_xyz(850.0, 360.0, 1.0),
        Name::new("TitleGoomba"),
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

pub fn setup_game_over(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((Camera2d, GameOverEntity));

    // Background
    for x in 0..(1280 / 18 + 2) {
        for y in 0..(720 / 18 + 2) {
            commands.spawn((
                GameOverEntity,
                Sprite {
                    image: assets.ground_tile.clone(),
                    ..default()
                },
                Transform::from_xyz(x as f32 * 18.0, y as f32 * 18.0, 0.0),
                Name::new("GameOverBackground"),
            ));
        }
    }

    commands.spawn((
        GameOverEntity,
        Text::new("GAME OVER"),
        TextFont {
            font_size: bevy::text::FontSize::Px(96.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.1, 0.1)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(220.0),
            left: Val::Px(450.0),
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
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(420.0),
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

pub fn setup_level_complete(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((Camera2d, LevelCompleteEntity));

    // Background
    for x in 0..(1280 / 18 + 2) {
        for y in 0..(720 / 18 + 2) {
            commands.spawn((
                LevelCompleteEntity,
                Sprite {
                    image: assets.ground_tile.clone(),
                    ..default()
                },
                Transform::from_xyz(x as f32 * 18.0, y as f32 * 18.0, 0.0),
                Name::new("LevelCompleteBackground"),
            ));
        }
    }

    commands.spawn((
        LevelCompleteEntity,
        Text::new("LEVEL COMPLETE!"),
        TextFont {
            font_size: bevy::text::FontSize::Px(96.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.9, 0.1)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(360.0),
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
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(420.0),
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
            font_size: bevy::text::FontSize::Px(96.0),
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.1)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(500.0),
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
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(350.0),
            left: Val::Px(450.0),
            ..default()
        },
    ));
}

pub fn cleanup_pause(mut commands: Commands, query: Query<Entity, With<PauseEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
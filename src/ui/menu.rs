use bevy::prelude::*;
use crate::game::MainMenuEntity;
use crate::GameState;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_main_menu(mut commands: Commands) {
    // Camera for UI
    commands.spawn(Camera2d);

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

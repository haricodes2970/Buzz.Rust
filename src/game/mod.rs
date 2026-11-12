pub mod components;
pub mod resources;

use bevy::audio::{AudioPlayer, AudioSource};
use bevy::prelude::*;
use crate::game::components::*;
use crate::game::resources::*;
use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>()
            .init_resource::<Score>()
            .init_resource::<Lives>()
            .init_resource::<TimerResource>()
            .init_resource::<PowerUpState>()
            .init_resource::<CurrentLevel>()
            .init_resource::<Coins>()
            .init_resource::<Xp>()
            .add_systems(OnEnter(GameState::MainMenu), crate::ui::menu::setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), crate::ui::menu::cleanup_main_menu)
            .add_systems(OnEnter(GameState::Playing), setup_level)
            .add_systems(OnExit(GameState::Playing), cleanup_level)
            .add_systems(OnEnter(GameState::GameOver), crate::ui::menu::setup_game_over)
            .add_systems(OnExit(GameState::GameOver), crate::ui::menu::cleanup_game_over)
            .add_systems(
                OnEnter(GameState::LevelComplete),
                crate::ui::menu::setup_level_complete,
            )
            .add_systems(
                OnExit(GameState::LevelComplete),
                crate::ui::menu::cleanup_level_complete,
            )
            .add_systems(OnEnter(GameState::Paused), crate::ui::menu::setup_pause)
            .add_systems(OnExit(GameState::Paused), crate::ui::menu::cleanup_pause);
    }
}

#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct LevelEntities;

fn setup_level(mut commands: Commands, _config: Res<GameConfig>) {
    // Spawn player
    commands.spawn((
        Player,
        Position { value: Vec2::new(100.0, 300.0) },
        Velocity::default(),
        Grounded::default(),
        Collider::new(24.0, 36.0),
        PowerState::default(),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(24.0, 36.0)),
            ..default()
        },
        Name::new("Player"),
    ));

    // Spawn camera
    commands.spawn((
        Camera2d,
        CameraFollow,
        Transform::from_xyz(100.0, 300.0, 1000.0),
    ));

    // Spawn some platforms (ground)
    commands.spawn((
        LevelEntities,
        Collider::new(600.0, 40.0),
        Transform::from_xyz(400.0, 400.0, 0.0),
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2),
            custom_size: Some(Vec2::new(600.0, 40.0)),
            ..default()
        },
        Name::new("Ground"),
    ));

    commands.spawn((
        LevelEntities,
        Collider::new(200.0, 20.0),
        Transform::from_xyz(800.0, 300.0, 0.0),
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        Name::new("Platform"),
    ));

    commands.spawn((
        LevelEntities,
        Collider::new(200.0, 20.0),
        Transform::from_xyz(1100.0, 250.0, 0.0),
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        Name::new("Platform2"),
    ));

    // Spawn some coins
    commands.spawn((
        LevelEntities,
        Coin,
        Collectable::new(10),
        Transform::from_xyz(850.0, 270.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Name::new("Coin"),
    ));

    commands.spawn((
        LevelEntities,
        Coin,
        Collectable::new(10),
        Transform::from_xyz(870.0, 270.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Name::new("Coin2"),
    ));

    // Spawn a Super Mushroom
    commands.spawn((
        LevelEntities,
        PowerUp,
        SuperMushroom,
        PowerUpType::SuperMushroom,
        Transform::from_xyz(1150.0, 230.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Name::new("SuperMushroom"),
    ));

    // Spawn a Goomba enemy
    commands.spawn((
        LevelEntities,
        Enemy,
        Goomba,
        Position { value: Vec2::new(900.0, 376.0) },
        EnemyAI::new(700.0, 1000.0, 60.0),
        Stompable,
        Velocity::default(),
        Collider::new(24.0, 24.0),
        Sprite {
            color: Color::srgb(0.8, 0.3, 0.1),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Name::new("Goomba"),
    ));

    // Spawn flagpole at end of level
    commands.spawn((
        LevelEntities,
        Flagpole,
        Transform::from_xyz(1400.0, 200.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(10.0, 200.0)),
            ..default()
        },
        Name::new("Flagpole"),
    ));

    // Spawn flag at top of pole
    commands.spawn((
        LevelEntities,
        Flag,
        Transform::from_xyz(1405.0, 310.0, 0.0),
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(30.0, 20.0)),
            ..default()
        },
        Name::new("Flag"),
    ));

    // Level end trigger (invisible)
    commands.spawn((
        LevelEntities,
        Collider::new(40.0, 400.0),
        Transform::from_xyz(1420.0, 300.0, 0.0),
        Name::new("LevelEndTrigger"),
    ));
}

fn cleanup_level(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelEntities>>,
    camera_query: Query<Entity, With<CameraFollow>>,
) {
    for entity in level_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in camera_query.iter() {
        commands.entity(entity).despawn();
    }
}

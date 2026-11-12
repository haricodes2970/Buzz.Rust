pub mod components;
pub mod resources;

use bevy::audio::{AudioPlayer, AudioSource};
use bevy::prelude::*;
use crate::assets::{AnimationTimer, GameAssets};
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

fn setup_level(mut commands: Commands, _config: Res<GameConfig>, assets: Res<GameAssets>) {
    // Spawn player with Kenney sprite
    commands.spawn((
        Player,
        Position { value: Vec2::new(100.0, 300.0) },
        Velocity::default(),
        Grounded::default(),
        Collider::new(20.0, 28.0),
        PowerState::default(),
        AnimationTimer::new(
            assets.player_idle.clone(),
            10.0,
            true,
        ),
        Sprite {
            image: assets.player_idle[0].clone(),
            ..default()
        },
        Name::new("Player"),
    ));

    // Spawn camera
    commands.spawn((
        Camera2d,
        CameraFollow,
        Position { value: Vec2::new(100.0, 300.0) },
        Transform::from_xyz(100.0, 300.0, 1000.0),
    ));

    // Spawn ground platforms using Kenney tiles
    for x in 0..160 {
        commands.spawn((
            LevelEntities,
            Collider::new(18.0, 18.0),
            Transform::from_xyz(x as f32 * 18.0 + 9.0, 400.0, 0.0),
            Sprite {
                image: assets.ground_tile.clone(),
                ..default()
            },
            Name::new("Ground"),
        ));
    }

    // Some platforms
    commands.spawn((
        LevelEntities,
        Collider::new(18.0 * 4.0, 18.0),
        Transform::from_xyz(400.0, 300.0, 0.0),
        Sprite {
            image: assets.ground_tile.clone(),
            ..default()
        },
        Name::new("Platform"),
    ));

    commands.spawn((
        LevelEntities,
        Collider::new(18.0 * 3.0, 18.0),
        Transform::from_xyz(700.0, 240.0, 0.0),
        Sprite {
            image: assets.ground_tile.clone(),
            ..default()
        },
        Name::new("Platform2"),
    ));

    commands.spawn((
        LevelEntities,
        Collider::new(18.0 * 4.0, 18.0),
        Transform::from_xyz(1000.0, 180.0, 0.0),
        Sprite {
            image: assets.ground_tile.clone(),
            ..default()
        },
        Name::new("Platform3"),
    ));

    // Spawn coins
    commands.spawn((
        LevelEntities,
        Coin,
        Collectable::new(10),
        Position { value: Vec2::new(420.0, 270.0) },
        Collider::new(12.0, 12.0),
        Sprite {
            image: assets.coin[0].clone(),
            ..default()
        },
        Name::new("Coin"),
    ));

    commands.spawn((
        LevelEntities,
        Coin,
        Collectable::new(10),
        Position { value: Vec2::new(440.0, 270.0) },
        Collider::new(12.0, 12.0),
        Sprite {
            image: assets.coin[1].clone(),
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
        Position { value: Vec2::new(720.0, 220.0) },
        Collider::new(18.0, 18.0),
        Sprite {
            image: assets.super_mushroom.clone(),
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
        Collider::new(20.0, 20.0),
        AnimationTimer::new(
            assets.goomba.clone(),
            8.0,
            true,
        ),
        Sprite {
            image: assets.goomba[0].clone(),
            ..default()
        },
        Name::new("Goomba"),
    ));

    // Spawn flagpole at end of level
    commands.spawn((
        LevelEntities,
        Flagpole,
        Position { value: Vec2::new(1400.0, 200.0) },
        Collider::new(10.0, 200.0),
        Sprite {
            image: assets.flag_pole.clone(),
            ..default()
        },
        Name::new("Flagpole"),
    ));

    // Spawn flag at top of pole
    commands.spawn((
        LevelEntities,
        Flag,
        Position { value: Vec2::new(1405.0, 310.0) },
        Collider::new(30.0, 20.0),
        Sprite {
            image: assets.flag.clone(),
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

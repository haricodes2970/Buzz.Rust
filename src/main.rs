mod assets;
mod game;
mod systems;
mod ui;
mod levels;

use assets::{GameAssets, load_assets};
use bevy::prelude::*;
use game::GamePlugin;
use systems::SystemsPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Buzz".into(),
                resolution: (1280u32, 720u32).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<GameAssets>()
        .add_systems(Startup, load_assets)
        .add_plugins(GamePlugin)
        .add_plugins(SystemsPlugin)
        .add_plugins(UiPlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    LevelComplete,
}

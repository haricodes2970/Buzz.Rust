use bevy::prelude::*;

// Asset handles for all Kenney pixel-platformer sprites
#[derive(Resource, Default)]
pub struct GameAssets {
    // Player character sprites (24x24 each)
    pub player_idle: Vec<Handle<Image>>,
    pub player_run: Vec<Handle<Image>>,
    pub player_jump: Handle<Image>,
    pub player_fall: Handle<Image>,

    // Enemy sprites
    pub goomba: Vec<Handle<Image>>,
    pub koopa: Vec<Handle<Image>>,
    pub piranha_plant: Vec<Handle<Image>>,
    pub flying_enemy: Vec<Handle<Image>>,

    // Item sprites
    pub coin: Vec<Handle<Image>>,
    pub super_mushroom: Handle<Image>,
    pub fire_flower: Handle<Image>,
    pub starman: Handle<Image>,
    pub one_up: Handle<Image>,

    // Tile sprites (18x18 each)
    pub ground_tile: Handle<Image>,
    pub grass_tile: Handle<Image>,
    pub dirt_tile: Handle<Image>,
    pub pipe_tile: Handle<Image>,
    pub pipe_top_tile: Handle<Image>,
    pub flag_pole: Handle<Image>,
    pub flag: Handle<Image>,

    // UI elements
    pub heart_full: Handle<Image>,
    pub heart_empty: Handle<Image>,
    pub coin_ui: Handle<Image>,
}

// Animation state for a sprite - cycles through individual image handles
#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frames: Vec<Handle<Image>>,
    pub current: usize,
    pub looping: bool,
}

impl AnimationTimer {
    pub fn new(frames: Vec<Handle<Image>>, fps: f32, looping: bool) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            frames,
            current: 0,
            looping,
        }
    }
}

// Load all Kenney pixel-platformer assets
pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<GameAssets>,
) {
    // --- Character sprites (24x24 each, tiles_0000 to tiles_0026) ---
    // Character tiles layout (9x3 grid):
    // Row 0 (tiles 0-8): Idle/Run frames
    // Row 1 (tiles 9-17): Jump/Fall frames
    // Row 2 (tiles 18-26): Other frames

    // Player idle: tiles 0-3 (4 frames)
    assets.player_idle = (0..4)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // Player run: tiles 4-7 (4 frames)
    assets.player_run = (4..8)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // Player jump: tile 8
    assets.player_jump = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0008.png");

    // Player fall: tile 9
    assets.player_fall = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0009.png");

    // --- Enemy sprites ---
    // Goomba: tiles 10-13 (4 frames walk)
    assets.goomba = (10..14)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // Koopa: tiles 14-17 (4 frames)
    assets.koopa = (14..18)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // Piranha Plant: tiles 18-21 (4 frames)
    assets.piranha_plant = (18..22)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // Flying enemy: tiles 22-26 (5 frames)
    assets.flying_enemy = (22..27)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/Characters/tile_{:04}.png", i)))
        .collect();

    // --- Item sprites ---
    // Coin: use tile_0000 from Tiles/ directory (18x18 tiles)
    assets.coin = (0..4)
        .map(|i| asset_server.load(format!("kenney_pixel-platformer/Tiles/tile_{:04}.png", i)))
        .collect();

    // Power-ups: use character tiles
    assets.super_mushroom = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0012.png");
    assets.fire_flower = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0013.png");
    assets.starman = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0014.png");
    assets.one_up = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0015.png");

    // --- Tile sprites (18x18 each) ---
    // Ground tile: tile_0000 from main Tiles directory
    assets.ground_tile = asset_server.load("kenney_pixel-platformer/Tiles/tile_0000.png");

    // Grass tile: tile_0001
    assets.grass_tile = asset_server.load("kenney_pixel-platformer/Tiles/tile_0001.png");

    // Dirt tile: tile_0002
    assets.dirt_tile = asset_server.load("kenney_pixel-platformer/Tiles/tile_0002.png");

    // Pipe tiles
    assets.pipe_tile = asset_server.load("kenney_pixel-platformer/Tiles/tile_0003.png");
    assets.pipe_top_tile = asset_server.load("kenney_pixel-platformer/Tiles/tile_0004.png");

    // Flag pole and flag
    assets.flag_pole = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0016.png");
    assets.flag = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0017.png");

    // --- UI elements ---
    assets.heart_full = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0018.png");
    assets.heart_empty = asset_server.load("kenney_pixel-platformer/Tiles/Characters/tile_0019.png");
    assets.coin_ui = asset_server.load("kenney_pixel-platformer/Tiles/tile_0005.png");

    info!("Assets loaded successfully!");
}

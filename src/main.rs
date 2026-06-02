mod ball;
mod game_over;
mod hud;
mod menu;
mod paddle;
mod state;

use bevy::{asset::{embedded_asset, load_embedded_asset}, prelude::*, window::WindowResolution};
use state::{AppState, Score};
use ball::{spawn_ball, move_ball, ball_collide, check_goal};
use game_over::{spawn_game_over, game_over_input};
use hud::{spawn_hud, update_score_display};
use menu::{spawn_menu, menu_input};
use paddle::{spawn_players, move_paddle};

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 480.;

// All image handles needed at runtime. Stored as a resource so spawn systems
// can clone handles without going through the AssetServer each time.
// Handles are reference-counted pointers; cloning is cheap and does not
// duplicate the texture data.
#[derive(Resource)]
pub struct GameAssets {
    pub ball: Handle<Image>,
    pub player: Handle<Image>,
    pub computer: Handle<Image>,
    pub board: Handle<Image>,
    pub score_bar: Handle<Image>,
}

fn main() {
    // Break out of the fluent chain so we can call embedded_asset! on &mut App
    // before .run() consumes it.
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
            resizable: false,
            title: "Pong".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .init_state::<AppState>()
    .init_resource::<Score>()
    .add_systems(Startup, (spawn_camera, load_game_assets))
    .add_systems(OnEnter(AppState::Menu), spawn_menu)
    .add_systems(OnEnter(AppState::Playing), (spawn_players, spawn_ball, spawn_hud))
    .add_systems(OnEnter(AppState::GameOver), spawn_game_over)
    .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
    .add_systems(
        Update,
        (move_paddle, move_ball, ball_collide, check_goal, update_score_display)
            .run_if(in_state(AppState::Playing)),
    )
    .add_systems(Update, game_over_input.run_if(in_state(AppState::GameOver)));

    // embedded_asset!(app, path) uses include_bytes! to bake the file into the
    // binary at compile time. The path is relative to THIS source file (src/main.rs).
    // Bevy registers the bytes under the "embedded://" asset source so the
    // AssetServer can load them without touching the filesystem at runtime.
    embedded_asset!(app, "../assets/Ball.png");
    embedded_asset!(app, "../assets/Player.png");
    embedded_asset!(app, "../assets/Computer.png");
    embedded_asset!(app, "../assets/Board.png");
    embedded_asset!(app, "../assets/ScoreBar.png");

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Runs once at startup. Loads all image handles from the embedded asset source
// and stores them in GameAssets. Systems that need a texture read from this
// resource instead of calling AssetServer::load every time.
fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load_embedded_asset! generates the same embedded:// path that
    // embedded_asset! registered above, because both macros are called from
    // this file and use file!() to compute the path consistently.
    commands.insert_resource(GameAssets {
        ball: load_embedded_asset!(&*asset_server, "../assets/Ball.png"),
        player: load_embedded_asset!(&*asset_server, "../assets/Player.png"),
        computer: load_embedded_asset!(&*asset_server, "../assets/Computer.png"),
        board: load_embedded_asset!(&*asset_server, "../assets/Board.png"),
        score_bar: load_embedded_asset!(&*asset_server, "../assets/ScoreBar.png"),
    });
}

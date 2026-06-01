mod ball;
mod game_over;
mod hud;
mod menu;
mod paddle;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use state::{AppState, Score};
use ball::{spawn_ball, move_ball, ball_collide, check_goal};
use game_over::{spawn_game_over, game_over_input};
use hud::{spawn_hud, update_score_display};
use menu::{spawn_menu, menu_input};
use paddle::{spawn_players, move_paddle};

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(AppState::Menu), spawn_menu)
        .add_systems(OnEnter(AppState::Playing), (spawn_players, spawn_ball, spawn_hud))
        .add_systems(OnEnter(AppState::GameOver), spawn_game_over)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(
            Update,
            (move_paddle, move_ball, ball_collide, check_goal, update_score_display)
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(Update, game_over_input.run_if(in_state(AppState::GameOver)))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

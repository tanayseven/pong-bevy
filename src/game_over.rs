use bevy::prelude::*;
use crate::state::{AppState, Score, Side, Winner};

pub fn spawn_game_over(mut commands: Commands, winner: Option<Res<Winner>>) {
    let msg = match winner.as_ref().map(|w| w.0) {
        Some(Side::Left) => "LEFT PLAYER WINS!",
        Some(Side::Right) => "RIGHT PLAYER WINS!",
        None => "GAME OVER",
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            DespawnOnExit(AppState::GameOver),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(msg),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("ENTER — Rematch   ESC — Menu"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn game_over_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
    mut score: ResMut<Score>,
) {
    if input.just_pressed(KeyCode::Enter) {
        *score = Score::default();
        next.set(AppState::Playing);
    }
    if input.just_pressed(KeyCode::Escape) {
        *score = Score::default();
        next.set(AppState::Menu);
    }
}

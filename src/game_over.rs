use bevy::prelude::*;
use crate::state::{AppState, Score, Side, Winner};

// `Option<Res<Winner>>` is used instead of `Res<Winner>` because the Winner
// resource is only inserted when a player wins (in `check_goal`). Bevy would
// panic at startup if a system unconditionally requested a resource that
// doesn't exist yet. Wrapping it in `Option` tells Bevy to pass `None` rather
// than crashing when the resource is absent.
pub fn spawn_game_over(mut commands: Commands, winner: Option<Res<Winner>>) {
    // Read the winner out of the Option<Res<...>> using a chain of map calls.
    // `winner.as_ref()` borrows the Option, `.map(|w| w.0)` copies the Side
    // (Side is Copy) out of the Res smart pointer.
    let msg = match winner.as_ref().map(|w| w.0) {
        Some(Side::Left) => "LEFT PLAYER WINS!",
        Some(Side::Right) => "RIGHT PLAYER WINS!",
        // Defensive fallback: should never be reached in normal play, but
        // avoids a panic if somehow we arrive here without a Winner resource.
        None => "GAME OVER",
    };

    // Same flexbox-centred full-screen layout as the main menu.
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
            // All entities spawned here are automatically cleaned up when
            // leaving GameOver — either on rematch (→ Playing) or menu (→ Menu).
            DespawnOnExit(AppState::GameOver),
        ))
        .with_children(|p| {
            // The winner message was computed above as a `&'static str` from
            // a match, so no allocation is needed to pass it to Text::new.
            p.spawn((
                Text::new(msg),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("ENTER - Rematch   ESC - Menu"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn game_over_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<AppState>>,
    // `ResMut<Score>` gives mutable access to reset the scores before a new game.
    mut score: ResMut<Score>,
) {
    if input.just_pressed(KeyCode::Enter) {
        // Dereference through ResMut to replace the whole Score value with the
        // default (zeroed) one. This triggers change detection: any system that
        // reads `score.is_changed()` will see it as changed next frame.
        *score = Score::default();
        // Transition back into Playing. OnExit(GameOver) despawns the game over
        // screen; OnEnter(Playing) re-spawns players, ball, and the score HUD.
        next.set(AppState::Playing);
    }
    if input.just_pressed(KeyCode::Escape) {
        *score = Score::default();
        // Return to the title screen instead of starting a new round.
        next.set(AppState::Menu);
    }
}
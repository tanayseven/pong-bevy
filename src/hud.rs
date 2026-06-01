use bevy::prelude::*;
use crate::state::{AppState, Score};

#[derive(Component)]
pub struct LeftScoreText;

#[derive(Component)]
pub struct RightScoreText;

pub fn spawn_hud(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        Text::new(score.left.to_string()),
        TextFont { font_size: 48.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            left: Val::Px(160.),
            ..default()
        },
        LeftScoreText,
        DespawnOnExit(AppState::Playing),
    ));

    commands.spawn((
        Text::new(score.right.to_string()),
        TextFont { font_size: 48.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.),
            right: Val::Px(160.),
            ..default()
        },
        RightScoreText,
        DespawnOnExit(AppState::Playing),
    ));
}

pub fn update_score_display(
    score: Res<Score>,
    mut left: Query<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right: Query<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
) {
    if score.is_changed() {
        for mut t in &mut left {
            *t = Text::new(score.left.to_string());
        }
        for mut t in &mut right {
            *t = Text::new(score.right.to_string());
        }
    }
}

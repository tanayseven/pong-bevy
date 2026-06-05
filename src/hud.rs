use bevy::prelude::*;
use crate::state::{AppState, Score};
use crate::GameAssets;

// Height of the ScoreBar image in pixels. Exported so ball and paddle systems
// can use it as the top boundary instead of the raw window edge.
pub const SCOREBAR_HEIGHT: f32 = 47.;

// Marker components — zero-sized structs used to tag entities so we can
// query for them specifically without reading any data from them.
#[derive(Component)]
pub struct LeftScoreText;

#[derive(Component)]
pub struct RightScoreText;

// `Res<Score>` gives read-only access to the Score resource.
// `Res<GameAssets>` provides the pre-loaded ScoreBar handle.
pub fn spawn_hud(mut commands: Commands, score: Res<Score>, assets: Res<GameAssets>) {
    // Clone the handle — cheap reference copy, no texture duplication.
    let scorebar: Handle<Image> = assets.score_bar.clone();

    // --- Left score bar background ---
    // `ImageNode` is Bevy's UI image component (the UI equivalent of `Sprite`
    // for world-space entities). It renders a texture inside a UI layout node.
    commands.spawn((
        ImageNode {
            image: scorebar.clone(),
            ..default()
        },
        // ScoreBar.png is 341×47. We pin it to the top-left and stretch it to
        // cover the left half of the window (320×47) using absolute positioning.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Px(0.),
            width: Val::Px(crate::WINDOW_WIDTH / 2.),
            height: Val::Px(47.),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // --- Right score bar background (horizontally mirrored) ---
    commands.spawn((
        ImageNode {
            image: scorebar,
            // `flip_x` mirrors the image along the vertical axis so the
            // chevron cut faces the correct direction on the right side.
            flip_x: true,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            right: Val::Px(0.),
            width: Val::Px(crate::WINDOW_WIDTH / 2.),
            height: Val::Px(47.),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // --- Score text (rendered on top of the bar images) ---
    // UI entities are drawn in spawn order within the same z-layer, so
    // spawning the text after the bars ensures it appears in front of them.
    commands.spawn((
        Text::new(score.left.to_string()),
        TextFont { font_size: 36.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            // Vertically centre within the 47px bar; horizontally land near
            // the middle of the left 320px half.
            top: Val::Px(5.),
            left: Val::Px(145.),
            ..default()
        },
        ZIndex(1),
        LeftScoreText,
        DespawnOnExit(AppState::Playing),
    ));

    commands.spawn((
        Text::new(score.right.to_string()),
        TextFont { font_size: 36.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(145.),
            ..default()
        },
        ZIndex(1),
        RightScoreText,
        DespawnOnExit(AppState::Playing),
    ));
}

pub fn update_score_display(
    score: Res<Score>,
    // `Without<T>` on each query ensures they can never match the same entity,
    // letting Bevy hand out two simultaneous `&mut Text` borrows safely.
    mut left: Query<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right: Query<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
) {
    // `is_changed()` is true only on frames where something wrote to
    // ResMut<Score>, avoiding unnecessary string allocations every frame.
    if score.is_changed() {
        for mut t in &mut left {
            *t = Text::new(score.left.to_string());
        }
        for mut t in &mut right {
            *t = Text::new(score.right.to_string());
        }
    }
}

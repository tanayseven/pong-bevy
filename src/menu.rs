use bevy::prelude::*;
use crate::state::AppState;

pub fn spawn_menu(mut commands: Commands) {
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
            DespawnOnExit(AppState::Menu),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("PONG"),
                TextFont { font_size: 96.0, ..default() },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("Press ENTER to start"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn menu_input(input: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Enter) {
        next.set(AppState::Playing);
    }
}

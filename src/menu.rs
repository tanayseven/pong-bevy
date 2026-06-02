use bevy::prelude::*;
use crate::state::AppState;

pub fn spawn_menu(mut commands: Commands) {
    // Bevy's UI system uses an entity hierarchy where parent nodes lay out
    // their children using a CSS flexbox-like algorithm. The root node here
    // fills the entire window and centres its children vertically and
    // horizontally — like a `display: flex; flex-direction: column;
    // align-items: center; justify-content: center` div in CSS.
    commands
        .spawn((
            // `Node` is the core UI layout component. Adding it to an entity
            // opts it into Bevy's UI layout pass. Every UI entity needs one.
            Node {
                // `Val::Percent(100.)` means 100% of the parent's size.
                // The implicit root of Bevy's UI is the window itself, so
                // 100% here means the full window width/height.
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                // Stack children top-to-bottom (title text, then subtitle text).
                flex_direction: FlexDirection::Column,
                // Centre children along the cross axis (horizontally, since
                // flex_direction is Column — the cross axis is the x axis).
                align_items: AlignItems::Center,
                // Centre children along the main axis (vertically).
                justify_content: JustifyContent::Center,
                // `Val::Px(20.)` is an absolute pixel gap between each child.
                row_gap: Val::Px(20.),
                ..default()
            },
            // `BackgroundColor` fills the node's rectangle with a colour.
            BackgroundColor(Color::BLACK),
            // This entity (and its children) will be automatically despawned
            // when the app transitions away from AppState::Menu.
            DespawnOnExit(AppState::Menu),
        ))
        // `.with_children` gives a `ChildBuilder` whose `.spawn` calls attach
        // the new entities as children of the parent. Child UI entities are
        // laid out by the parent's flexbox rules defined above.
        .with_children(|p| {
            // `Text::new("...")` creates a UI text entity with a single span.
            // `TextFont` controls the font and size (Bevy loads a built-in
            // font when no handle is provided).
            // `TextColor` sets the text colour.
            p.spawn((
                Text::new("PONG"),
                TextFont { font_size: 96.0, ..default() },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("Press ENTER to start"),
                TextFont { font_size: 24.0, ..default() },
                // `Color::srgb(r, g, b)` uses the sRGB colour space with
                // components in the 0.0–1.0 range. This gives a grey colour.
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn menu_input(input: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    // `just_pressed` is true only on the single frame the key transitions
    // from up to down. Using `pressed` here would fire every frame the key
    // is held, potentially triggering multiple transitions in a row.
    if input.just_pressed(KeyCode::Enter) {
        next.set(AppState::Playing);
    }
}
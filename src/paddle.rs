use bevy::prelude::*;
use crate::state::AppState;

pub const PADDLE_SPEED: f32 = 100.;
pub const PADDLE_OFFSET: f32 = 30.;
pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 70.;

// Stores the keybindings for a single paddle. Attaching this component to an entity
// instead of hard-coding keys lets both paddles share the same move_paddle system.
#[derive(Component)]
pub struct Paddle {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

pub fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(crate::WINDOW_WIDTH, crate::WINDOW_HEIGHT)),
            ..Default::default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Left paddle: W/S keys.
    commands.spawn((
        Transform::from_translation(Vec3::new(-(crate::WINDOW_WIDTH / 2.) + 50., 0., 1.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Right paddle: arrow keys.
    commands.spawn((
        Transform::from_translation(Vec3::new((crate::WINDOW_WIDTH / 2.) - 50., 0., 1.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
        DespawnOnExit(AppState::Playing),
    ));
}

pub fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(
                (-crate::WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (crate::WINDOW_HEIGHT / 2.) - PADDLE_OFFSET,
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(
                (-crate::WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (crate::WINDOW_HEIGHT / 2.) - PADDLE_OFFSET,
            );
        }
    }
}

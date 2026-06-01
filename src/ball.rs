use bevy::prelude::*;
use rand::RngExt;
use crate::state::{AppState, Score, Side, Winner};
use crate::paddle::{Paddle, PADDLE_WIDTH, PADDLE_HEIGHT};

pub const BALL_DIMENSION: f32 = 15.;
pub const WINNING_SCORE: u32 = 5;

// The Ball component stores the current velocity as a 2D vector (pixels per second).
#[derive(Component)]
pub struct Ball(pub Vec2);

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0., 0., 2.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_DIMENSION, BALL_DIMENSION)),
            ..Default::default()
        },
        Ball(Vec2::new(-100., 0.)),
        DespawnOnExit(AppState::Playing),
    ));
}

pub fn move_ball(mut ball: Query<(&mut Transform, &mut Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut ball {
        // extend(0.) promotes the 2D velocity to a Vec3 so it can be added to translation.
        pos.translation += ball.0.extend(0.) * time.delta_secs();
    }
}

pub fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        // Wall bounce: flip vertical velocity when the ball's edge reaches the top/bottom.
        if ball.translation.y.abs() + BALL_DIMENSION / 2. > crate::WINDOW_HEIGHT / 2. {
            velocity.0.y *= -1.;
        }
        for paddle in &paddles {
            // AABB overlap: ball straddles the paddle's x column and their y spans intersect.
            if ball.translation.x - BALL_DIMENSION / 2. < paddle.translation.x - PADDLE_WIDTH / 2.
                && ball.translation.x + BALL_DIMENSION / 2.
                    > paddle.translation.x + PADDLE_WIDTH / 2.
                && ball.translation.y + BALL_DIMENSION / 2.
                    > paddle.translation.y - PADDLE_HEIGHT / 2.
                && ball.translation.y - BALL_DIMENSION / 2.
                    < paddle.translation.y + PADDLE_HEIGHT / 2.
            {
                velocity.0 *= -1.;
                // Randomise vertical angle so rallies stay unpredictable.
                let mut rng = rand::rng();
                let up = rng.random_bool(0.5);
                if up {
                    velocity.0.y = rng.random_range(-100..-20) as f32;
                } else {
                    velocity.0.y = rng.random_range(20..100) as f32;
                }
            }
        }
    }
}

pub fn check_goal(
    mut commands: Commands,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    mut score: ResMut<Score>,
    mut next: ResMut<NextState<AppState>>,
) {
    for (mut pos, mut velocity) in &mut balls {
        let scored = if pos.translation.x < -crate::WINDOW_WIDTH / 2. {
            Some(Side::Right)
        } else if pos.translation.x > crate::WINDOW_WIDTH / 2. {
            Some(Side::Left)
        } else {
            None
        };

        if let Some(side) = scored {
            match side {
                Side::Left => score.left += 1,
                Side::Right => score.right += 1,
            }
            // Reset ball to centre regardless so it can't score twice before state transition.
            pos.translation = Vec3::new(0., 0., 2.);
            if score.left >= WINNING_SCORE || score.right >= WINNING_SCORE {
                velocity.0 = Vec2::ZERO;
                commands.insert_resource(Winner(side));
                next.set(AppState::GameOver);
            } else {
                let dir = if matches!(side, Side::Left) { -1. } else { 1. };
                velocity.0 = Vec2::new(100. * dir, 0.);
            }
        }
    }
}

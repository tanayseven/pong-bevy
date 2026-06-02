use bevy::prelude::*;
use rand::RngExt;
use crate::state::{AppState, Score, Side, Winner};
use crate::paddle::{Paddle, PADDLE_WIDTH, PADDLE_HEIGHT};
use crate::hud::SCOREBAR_HEIGHT;
use crate::GameAssets;

pub const BALL_DIMENSION: f32 = 15.;
// First player to reach this many points wins and triggers the GameOver state.
pub const WINNING_SCORE: u32 = 5;

// A component can carry data — here the ball's current velocity in pixels/sec.
// The newtype pattern (tuple struct with one field) lets us write
// `Query<&Ball>` to fetch only ball entities, not every entity with a Vec2.
#[derive(Component)]
pub struct Ball(pub Vec2);

// `Res<GameAssets>` provides the pre-loaded handles so we don't hit the
// filesystem on every spawn.
pub fn spawn_ball(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        // z = 2 renders the ball in front of paddles (z = 1) and background (z = 0).
        Transform::from_translation(Vec3::new(0., 0., 2.)),
        // Ball.png is a 30×30 magenta circle. custom_size scales it to the
        // game's 15×15 hitbox so collision logic stays unchanged.
        Sprite {
            image: assets.ball.clone(),
            custom_size: Some(Vec2::new(BALL_DIMENSION, BALL_DIMENSION)),
            ..Default::default()
        },
        // Initial velocity: 100 px/s to the left. The x component is
        // negative because in Bevy's 2D coordinate system, left is −x.
        Ball(Vec2::new(-100., 0.)),
        DespawnOnExit(AppState::Playing),
    ));
}

pub fn move_ball(mut ball: Query<(&mut Transform, &mut Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut ball {
        // `Vec2::extend(z)` promotes a 2D vector to Vec3 by appending z.
        // We extend with 0 so that translation (which is Vec3) isn't shifted
        // in the z axis — the ball stays at the layer it was spawned at.
        pos.translation += ball.0.extend(0.) * time.delta_secs();
    }
}

// This system takes TWO queries. Bevy's scheduler analyses parameter types
// at startup and verifies that the two queries don't alias mutable access to
// the same component on the same entity. Because `balls` needs `&mut Ball`
// and `paddles` only needs `&Transform` (filtered to entities With<Paddle>),
// there is no overlap and they can run safely — even in parallel if Bevy's
// scheduler decides to.
pub fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    // `With<Paddle>` is a query filter: include only entities that have the
    // Paddle component, even though we don't read Paddle's data here.
    // It narrows the query to avoid testing the ball against itself.
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        // Bottom wall: bounce when the ball's lower edge hits the floor.
        if ball.translation.y - BALL_DIMENSION / 2. < -crate::WINDOW_HEIGHT / 2. {
            velocity.0.y *= -1.;
        }
        // Top wall: bounce off the bottom edge of the ScoreBar, not the raw
        // window top. SCOREBAR_HEIGHT (47px) is subtracted so the ball never
        // slides behind the UI bar.
        if ball.translation.y + BALL_DIMENSION / 2. > crate::WINDOW_HEIGHT / 2. - SCOREBAR_HEIGHT {
            velocity.0.y *= -1.;
        }

        for paddle in &paddles {
            // Axis-Aligned Bounding Box (AABB) overlap test.
            // The ball (15 px square) overlaps the paddle (10×70 px) when:
            //   • horizontally: the ball's left edge is past the paddle's left
            //     AND the ball's right edge is past the paddle's right — i.e.
            //     the ball's x span fully straddles the paddle's thin x column.
            //   • vertically: the spans intersect (standard interval overlap).
            if ball.translation.x - BALL_DIMENSION / 2. < paddle.translation.x - PADDLE_WIDTH / 2.
                && ball.translation.x + BALL_DIMENSION / 2.
                    > paddle.translation.x + PADDLE_WIDTH / 2.
                && ball.translation.y + BALL_DIMENSION / 2.
                    > paddle.translation.y - PADDLE_HEIGHT / 2.
                && ball.translation.y - BALL_DIMENSION / 2.
                    < paddle.translation.y + PADDLE_HEIGHT / 2.
            {
                // Reverse both axes so the ball heads back toward the centre.
                velocity.0 *= -1.;
                // Add randomised vertical angle to keep rallies unpredictable.
                // rand::rng() creates a thread-local RNG seeded from the OS.
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

// This system needs to both read a Query AND mutate two resources, so it
// takes `Commands`, a `Query`, `ResMut<Score>`, and `ResMut<NextState>`.
// Bevy validates at startup that no two concurrently-running systems take
// conflicting mutable access to the same resource or component.
pub fn check_goal(
    // `Commands` is needed here to insert the `Winner` resource at runtime.
    mut commands: Commands,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    // `ResMut<T>` gives mutable access to a resource. Bevy ensures only one
    // system holds ResMut<Score> at a time (enforced at compile + schedule time).
    mut score: ResMut<Score>,
    // `NextState<S>` is the resource you write to in order to request a state
    // transition. Bevy reads it at the end of the frame and performs the
    // transition (running OnExit then OnEnter) before the next Update tick.
    mut next: ResMut<NextState<AppState>>,
) {
    for (mut pos, mut velocity) in &mut balls {
        // Ball past the left edge → right player scores; past right → left scores.
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
            // Move ball back to centre immediately. If we only did this on the
            // non-winning path, the ball would stay off-screen for one extra
            // frame and this system would fire again before the state transition
            // processes, potentially double-counting the goal.
            pos.translation = Vec3::new(0., 0., 2.);

            if score.left >= WINNING_SCORE || score.right >= WINNING_SCORE {
                // Stop the ball so it doesn't re-trigger while the transition
                // is pending (state changes are deferred to end-of-frame).
                velocity.0 = Vec2::ZERO;
                // `commands.insert_resource` inserts a resource that didn't
                // exist yet. If it already exists it is overwritten.
                // The `spawn_game_over` system (which runs in OnEnter(GameOver))
                // reads this resource to know who won.
                commands.insert_resource(Winner(side));
                // `.set(S)` queues the transition. The actual OnExit/OnEnter
                // schedules run after the current Update completes.
                next.set(AppState::GameOver);
            } else {
                // Serve toward the player who just conceded.
                let dir = if matches!(side, Side::Left) { -1. } else { 1. };
                velocity.0 = Vec2::new(100. * dir, 0.);
            }
        }
    }
}
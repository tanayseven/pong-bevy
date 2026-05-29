use bevy::{prelude::*, window::WindowResolution};
use rand::RngExt;

// Fixed window dimensions in pixels. The Bevy 2D coordinate system places (0, 0) at the
// centre of the window, so x ranges from -320 to +320 and y from -240 to +240.
const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    let mut app = App::new();
    // DefaultPlugins bundles everything Bevy needs: rendering, windowing, input, audio, etc.
    // We override WindowPlugin to set a fixed 640×480 non-resizable window.
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
            resizable: false,
            title: "Pong".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    // Startup systems run once when the app launches, in the order listed.
    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    // Update systems run every frame. move_paddle reads input, move_ball advances position,
    // ball_collide checks for and resolves collisions.
    app.add_systems(Update, (move_paddle, move_ball, ball_collide));
    app.run();
}

// Bevy requires at least one Camera entity to render anything to the screen.
// Camera2d sets up an orthographic projection centred at (0, 0).
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Stores the keybindings for a single paddle. Attaching this component to an entity
// instead of hard-coding keys lets both paddles share the same move_paddle system.
#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_players(mut commands: Commands) {
    // Full-screen black rectangle that acts as the game background.
    // Spawned at z = 0 (default) so it sits behind the paddles and ball.
    commands.spawn(Sprite {
        color: Color::BLACK,
        custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..Default::default()
    });

    // Left paddle: positioned 50 pixels in from the left edge of the window.
    // z = 1 places it in front of the background but behind the ball (z = 2).
    // Controlled with W (up) and S (down).
    commands.spawn((
        Transform::from_translation(Vec3::new(-(WINDOW_WIDTH / 2.) + 50., 0., 1.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));

    // Right paddle: mirrored position on the right side of the window.
    // Controlled with the arrow keys.
    commands.spawn((
        Transform::from_translation(Vec3::new((WINDOW_WIDTH / 2.) - 50., 0., 1.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    ));
}

// Pixels per second the paddle travels when a key is held.
const PADDLE_SPEED: f32 = 100.;
// Half the paddle height (35 px) would be the exact edge, but we use 30 so the paddle
// stops slightly before the wall — keeps the sprite fully visible.
const PADDLE_OFFSET: f32 = 30.;

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        // Multiply by delta_secs so movement speed is frame-rate independent.
        if input.pressed(settings.move_up) {
            pos.translation.y += PADDLE_SPEED * time.delta_secs();
            // Clamp to keep the paddle inside the window boundaries.
            pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.) + PADDLE_OFFSET, (WINDOW_HEIGHT / 2.) - PADDLE_OFFSET);
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.) + PADDLE_OFFSET, (WINDOW_HEIGHT / 2.) - PADDLE_OFFSET);
        }
    }
}

// The Ball component stores the current velocity as a 2D vector (pixels per second).
// Using a newtype wrapper around Vec2 lets us query for Ball entities specifically.
#[derive(Component)]
struct Ball (Vec2);

fn spawn_ball(mut commands: Commands) {
    // Ball starts at the centre of the screen moving left at 100 px/s.
    // z = 2 ensures it renders on top of both the background (z = 0) and paddles (z = 1).
    commands.spawn((
        Transform::from_translation(Vec3::new(0., 0., 2.)),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_DIMENSION, BALL_DIMENSION)),
            ..Default::default()
        },
        Ball(Vec2::new(-100., 0.)),
    ));
}

fn move_ball(
    mut ball: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
) {
    for (mut pos, ball) in &mut ball {
        // extend(0.) promotes the 2D velocity to a Vec3 (z = 0) so it can be added to
        // the 3D translation. Multiplying by delta_secs keeps movement frame-rate independent.
        pos.translation += ball.0.extend(0.) * time.delta_secs();
    }
}

const BALL_DIMENSION: f32 = 15.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 70.;

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        // Wall bounce: if the ball's edge reaches the top or bottom of the window,
        // flip the vertical component of velocity to reflect it back inward.
        if ball.translation.y.abs() + BALL_DIMENSION / 2. > WINDOW_HEIGHT / 2. {
            velocity.0.y *= -1.;
        }
        for paddle in &paddles {
            // AABB overlap test: the ball (15 px square) overlaps the paddle (10×70 px)
            // when the ball's horizontal span completely straddles the paddle's x position
            // (ball left < paddle left AND ball right > paddle right) AND their vertical
            // spans intersect. Because the ball is wider than the paddle, this effectively
            // triggers when the ball passes through the paddle's x column.
            if ball.translation.x - BALL_DIMENSION / 2. < paddle.translation.x - PADDLE_WIDTH / 2.
                && ball.translation.x + BALL_DIMENSION / 2. > paddle.translation.x + PADDLE_WIDTH / 2.
                && ball.translation.y + BALL_DIMENSION / 2. > paddle.translation.y - PADDLE_HEIGHT / 2.
                && ball.translation.y - BALL_DIMENSION / 2. < paddle.translation.y + PADDLE_HEIGHT / 2.
            {
                // Reverse both velocity components to send the ball back toward the centre.
                velocity.0 *= -1.;
                // Randomise the vertical angle after each paddle hit to make rallies less
                // predictable. A coin flip chooses the sign; the magnitude is picked from
                // [20, 100) px/s so the ball always has some vertical movement.
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

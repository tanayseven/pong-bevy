use bevy::{prelude::*, window::WindowResolution};
use rand::RngExt;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;
const PADDLE_SPEED: f32 = 100.;
const PADDLE_OFFSET: f32 = 30.;
const BALL_DIMENSION: f32 = 15.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 70.;
const WINNING_SCORE: u32 = 5;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource, Default, Clone, Copy)]
struct Score {
    left: u32,
    right: u32,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Resource)]
struct Winner(Side);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                resizable: false,
                title: "Pong".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<AppState>()
        .init_resource::<Score>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(AppState::Menu), spawn_menu)
        .add_systems(OnEnter(AppState::Playing), (spawn_players, spawn_ball, spawn_hud))
        .add_systems(OnEnter(AppState::GameOver), spawn_game_over)
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(
            Update,
            (move_paddle, move_ball, ball_collide, check_goal, update_score_display)
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(Update, game_over_input.run_if(in_state(AppState::GameOver)))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// --- Menu ---

fn spawn_menu(mut commands: Commands) {
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

fn menu_input(input: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Enter) {
        next.set(AppState::Playing);
    }
}

// --- Gameplay ---

// Stores the keybindings for a single paddle. Attaching this component to an entity
// instead of hard-coding keys lets both paddles share the same move_paddle system.
#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

// The Ball component stores the current velocity as a 2D vector (pixels per second).
#[derive(Component)]
struct Ball(Vec2);

#[derive(Component)]
struct LeftScoreText;

#[derive(Component)]
struct RightScoreText;

fn spawn_players(mut commands: Commands) {
    // Full-screen black rectangle that acts as the game background.
    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..Default::default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Left paddle: W/S keys.
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
        DespawnOnExit(AppState::Playing),
    ));

    // Right paddle: arrow keys.
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
        DespawnOnExit(AppState::Playing),
    ));
}

fn spawn_ball(mut commands: Commands) {
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

fn spawn_hud(mut commands: Commands, score: Res<Score>) {
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

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(
                (-WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (WINDOW_HEIGHT / 2.) - PADDLE_OFFSET,
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(
                (-WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (WINDOW_HEIGHT / 2.) - PADDLE_OFFSET,
            );
        }
    }
}

fn move_ball(mut ball: Query<(&mut Transform, &mut Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut ball {
        // extend(0.) promotes the 2D velocity to a Vec3 so it can be added to translation.
        pos.translation += ball.0.extend(0.) * time.delta_secs();
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        // Wall bounce: flip vertical velocity when the ball's edge reaches the top/bottom.
        if ball.translation.y.abs() + BALL_DIMENSION / 2. > WINDOW_HEIGHT / 2. {
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

fn check_goal(
    mut commands: Commands,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    mut score: ResMut<Score>,
    mut next: ResMut<NextState<AppState>>,
) {
    for (mut pos, mut velocity) in &mut balls {
        let scored = if pos.translation.x < -WINDOW_WIDTH / 2. {
            Some(Side::Right)
        } else if pos.translation.x > WINDOW_WIDTH / 2. {
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

fn update_score_display(
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

// --- Game Over ---

fn spawn_game_over(mut commands: Commands, winner: Option<Res<Winner>>) {
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

fn game_over_input(
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
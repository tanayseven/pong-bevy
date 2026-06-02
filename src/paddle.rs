use bevy::prelude::*;
use crate::state::AppState;
use crate::hud::SCOREBAR_HEIGHT;
use crate::GameAssets;

// Pixels per second a paddle travels when a key is held down.
pub const PADDLE_SPEED: f32 = 100.;
// How many pixels from the window edge the paddle centre is clamped to.
// Using 30 (slightly less than half the paddle height of 35) keeps the
// sprite fully visible instead of clipping into the wall.
pub const PADDLE_OFFSET: f32 = 30.;
pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 70.;

// `Component` is the core Bevy derive. A component is data attached to an
// entity. By storing the keybindings here instead of hard-coding them, both
// paddles can share the same `move_paddle` system — the system just reads
// whatever keys this component says to use.
#[derive(Component)]
pub struct Paddle {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

// `Commands` provides deferred ECS mutations: spawn, despawn, insert/remove
// components, insert/remove resources. Changes are batched and applied after
// all systems in the current schedule set finish executing.
//
// `AssetServer` is a Bevy resource that loads files from the `assets/`
// directory at runtime. `asset_server.load("foo.png")` returns a
// `Handle<Image>` immediately — loading happens asynchronously in the
// background. The sprite renders as transparent until the asset is ready,
// which on a local disk is typically within one frame.
pub fn spawn_players(mut commands: Commands, assets: Res<GameAssets>) {
    // Spawning a tuple of components creates one entity with all of them.
    // Board.png (802×455) is the court background. custom_size stretches it
    // to fill the full window. z defaults to 0 (behind paddles and ball).
    commands.spawn((
        // Setting `image` on a Sprite makes Bevy render the texture instead
        // of a flat colour. The `color` field (defaulting to WHITE) acts as
        // a tint multiplier — WHITE passes the texture through unmodified.
        // `.clone()` on a Handle is cheap — it copies the reference, not the data.
        Sprite {
            image: assets.board.clone(),
            custom_size: Some(Vec2::new(crate::WINDOW_WIDTH, crate::WINDOW_HEIGHT)),
            ..Default::default()
        },
        // `DespawnOnExit(S)` is a Bevy marker component that instructs the
        // state machine to automatically despawn this entity when the app
        // leaves state S. This replaces manual cleanup in OnExit systems.
        DespawnOnExit(AppState::Playing),
    ));

    commands.spawn((
        // `Transform` is Bevy's position/rotation/scale component. All
        // rendered entities need one. `from_translation` sets position;
        // z = 1 places it in front of the background (z = 0).
        // The 2D coordinate origin (0, 0) is the window centre, so
        // -(WIDTH/2) + 50 positions this 50px from the left edge.
        Transform::from_translation(Vec3::new(-(crate::WINDOW_WIDTH / 2.) + 50., 0., 1.)),
        // Player.png is 17×120. custom_size scales it to the game's 10×70
        // paddle size. Both share the same aspect ratio (≈0.14), so the
        // image scales proportionally without distortion.
        Sprite {
            image: assets.player.clone(),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        // Each paddle entity carries its own Paddle component, so the same
        // `move_paddle` system handles both without any if/else branching.
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
        DespawnOnExit(AppState::Playing),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new((crate::WINDOW_WIDTH / 2.) - 50., 0., 1.)),
        // Computer.png is the same shape as Player.png but in blue.
        Sprite {
            image: assets.computer.clone(),
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

// System parameters are injected by Bevy's scheduler. The order and names
// don't matter — Bevy matches by type. Here:
//   Query<(&mut Transform, &Paddle)>  — fetch every entity that has both
//       a Transform and a Paddle component; `&mut` means we want write access.
//   Res<ButtonInput<KeyCode>>  — read-only access to the keyboard state resource.
//   Res<Time>  — read-only access to Bevy's time resource (frame delta, etc.).
pub fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Iterating a Query yields one tuple per matching entity.
    for (mut pos, settings) in &mut paddles {
        // `input.pressed` returns true every frame the key is held down.
        // Contrast with `just_pressed` (true only on the first frame the key
        // is pressed) and `just_released` (true only on release).
        if input.pressed(settings.move_up) {
            // Multiplying by `delta_secs()` makes movement frame-rate
            // independent: a paddle moves PADDLE_SPEED pixels per second
            // regardless of whether the game runs at 30 fps or 144 fps.
            pos.translation.y += PADDLE_SPEED * time.delta_secs();
            // Clamp keeps the paddle inside the window. Without this, holding
            // a key would move the sprite off-screen indefinitely.
            pos.translation.y = pos.translation.y.clamp(
                (-crate::WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (crate::WINDOW_HEIGHT / 2.) - SCOREBAR_HEIGHT - PADDLE_OFFSET,
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= PADDLE_SPEED * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp(
                (-crate::WINDOW_HEIGHT / 2.) + PADDLE_OFFSET,
                (crate::WINDOW_HEIGHT / 2.) - SCOREBAR_HEIGHT - PADDLE_OFFSET,
            );
        }
    }
}
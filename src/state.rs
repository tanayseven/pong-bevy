use bevy::prelude::*;

// `States` is a Bevy derive macro that turns an enum into a state machine type.
// The required derives are:
//   Debug    — Bevy logs state names during transitions
//   Clone    — Bevy clones the state when storing it internally
//   PartialEq + Eq + Hash — used as a key in Bevy's internal state registry
//   Default  — sets the initial state (overridden by #[default] on a variant)
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    // `#[default]` marks this variant as the starting state. Bevy reads this
    // when `init_state::<AppState>()` is called in main.
    #[default]
    Menu,
    Playing,
    GameOver,
}

// `Resource` marks a struct as a Bevy resource: a single global instance
// stored in the ECS World. Unlike components (which are attached to entities),
// resources are standalone. Any system can read or mutate them via Res<T> /
// ResMut<T>. `Default` lets `init_resource::<Score>()` create the zero-value.
#[derive(Resource, Default, Clone, Copy)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

// Plain Rust enum — not a component or resource itself, but used as the
// payload inside the `Winner` resource so systems know which side won.
#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

// `Winner` is inserted as a resource dynamically (via `commands.insert_resource`)
// only when the game ends. Systems that need it use `Option<Res<Winner>>` so
// they don't panic if it hasn't been inserted yet.
#[derive(Resource)]
pub struct Winner(pub Side);
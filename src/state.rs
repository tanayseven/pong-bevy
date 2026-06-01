use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Resource)]
pub struct Winner(pub Side);

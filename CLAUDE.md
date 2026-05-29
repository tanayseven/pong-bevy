# Pong Bevy - Development Guide

## Project Overview

A Pong game built with [Bevy](https://bevy.rs) game engine in Rust.

## Toolchain

Managed via [mise](https://mise.jdx.dev). Run `mise install` to get the pinned Rust toolchain.

```
mise install
```

## Build

```bash
cargo build           # debug build
cargo build --release # optimised release build
```

## Run

```bash
cargo run
```

## Controls

| Player | Move Up | Move Down |
|--------|---------|-----------|
| Left   | W       | S         |
| Right  | ↑       | ↓         |

## Web (WASM) Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cargo install wasm-bindgen-cli
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy_pong" \
  ./target/wasm32-unknown-unknown/release/bevy_pong.wasm
```

Serve the `out/` directory with any static file server.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| bevy | 0.18 | Game engine |
| bevy_rapier2d | 0.34 | 2D physics (available, not yet wired) |
| rand | 0.10 | Randomness for ball deflection |

## Architecture

All game logic lives in `src/main.rs`:

- `spawn_camera` — spawns the 2D camera
- `spawn_players` — spawns the background, left paddle (W/S), right paddle (↑/↓)
- `spawn_ball` — spawns the ball with an initial leftward velocity
- `move_paddle` — moves paddles based on key input, clamped to window bounds
- `move_ball` — translates the ball by its velocity each frame
- `ball_collide` — bounces the ball off top/bottom walls and paddles, randomises vertical angle on paddle hits

## CI

GitHub Actions (`.github/workflows/ci.yaml`) builds and releases for Linux, Windows, macOS (Intel + Apple Silicon), and WASM via itch.io.

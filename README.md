# Pong

A classic two-player Pong game built with [Bevy](https://bevy.rs) 0.18.

Play online at [tanayseven.itch.io/pong-bevy](https://tanayseven.itch.io/pong-bevy).

## Controls

| Player | Move Up | Move Down |
|--------|---------|-----------|
| Left   | W       | S         |
| Right  | ↑       | ↓         |

## Getting Started

### Prerequisites

Install [mise](https://mise.jdx.dev) to get the correct Rust toolchain automatically:

```bash
mise install
```

### Build & Run

```bash
cargo run
```

### Release Build

```bash
cargo build --release
```

Binary is at `target/release/bevy_pong`.

### Web (WASM) Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cargo install wasm-bindgen-cli
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy_pong" \
  ./target/wasm32-unknown-unknown/release/bevy_pong.wasm
```

Serve the `out/` directory with any static HTTP server.

## Downloads

Pre-built binaries for Linux, Windows, macOS, and WASM are published automatically via [GitHub Releases](../../releases/latest) on every push to `master`.

## Tech Stack

- **[Bevy](https://bevy.rs) 0.18** — game engine
- **[bevy_rapier2d](https://github.com/dimforge/bevy_rapier) 0.34** — 2D physics
- **[rand](https://docs.rs/rand) 0.10** — ball deflection randomness

## License

[LICENSE.txt](LICENSE.txt)

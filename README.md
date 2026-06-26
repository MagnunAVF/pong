# Pong

A two-player Pong clone built with Rust and Bevy.

## Requirements

- Rust 1.85+ (edition 2024)
- Cargo

## Build and Run

```sh
cargo run --release
```

## Gameplay

Two players compete on a single keyboard. The first player to reach the score limit wins.

| Player | Move Up | Move Down |
|--------|---------|-----------|
| Player 1 (left) | `W` | `S` |
| Player 2 (right) | `Up` | `Down` |

## Controls

| State | Key | Action |
|-------|-----|--------|
| Main Menu | `Enter` | Start game |
| Main Menu | `Esc` | Quit |
| In-game | `P` | Pause |
| In-game | `Esc` | Return to main menu |
| Paused | `P` | Resume |
| Paused | `Esc` | Return to main menu |
| Game Over | `R` | Return to main menu |

## Project Structure

```
src/
  main.rs       -- App setup, window config, GameState enum, shared types
  ball.rs       -- Ball spawning, movement, wall bounce, scoring events
  paddle.rs     -- Paddle spawning, player input, boundary clamping
  collision.rs  -- AABB ball-vs-paddle collision, angle deflection, speed ramp
  score.rs      -- Score resource, win condition, HUD
  ui.rs         -- All screens (menu, pause, game over), walls, center line
```

## Tech Stack

- [Bevy](https://bevyengine.org/) 0.19.0 -- ECS game engine
- [rand](https://crates.io/crates/rand) 0.10.1 -- Random initial ball direction

## Configuration

A few constants can be changed directly in the source to tune the game:

| Constant | File | Default | Description |
|----------|------|---------|-------------|
| `WIN_SCORE` | `score.rs` | `2` | Points needed to win |
| `BALL_SPEED` | `ball.rs` | `300.0` | Initial ball speed (px/s) |
| `MAX_SPEED` | `collision.rs` | `600.0` | Ball speed cap after paddle hits |
| `PADDLE_SPEED` | `paddle.rs` | `400.0` | Paddle movement speed (px/s) |
| `WINDOW_WIDTH` | `main.rs` | `800` | Window width in pixels |
| `WINDOW_HEIGHT` | `main.rs` | `600` | Window height in pixels |

## License

MIT

use bevy::prelude::*;

use crate::ui::WALL_THICKNESS;
use crate::{GameState, InGame, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_paddles)
            .add_systems(
                Update,
                (move_player_one, move_player_two, clamp_paddles)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_X_OFFSET: f32 = 40.0;
const PADDLE_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

fn spawn_paddles(mut commands: Commands, existing: Query<(), With<Paddle>>) {
    if !existing.is_empty() {
        return;
    }
    let half_width = WINDOW_WIDTH as f32 / 2.0;
    let size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

    // Player 1 — left paddle
    commands.spawn((
        InGame,
        Paddle,
        Player::One,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(-half_width + PADDLE_X_OFFSET, 0.0, 0.0),
    ));

    // Player 2 / AI — right paddle
    commands.spawn((
        InGame,
        Paddle,
        Player::Two,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(half_width - PADDLE_X_OFFSET, 0.0, 0.0),
    ));
}

fn move_paddle(transform: &mut Transform, up: bool, down: bool, delta_secs: f32) {
    let direction = if up { 1.0 } else { 0.0 } + if down { -1.0 } else { 0.0 };
    transform.translation.y += direction * PADDLE_SPEED * delta_secs;
}

fn move_player_one(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform), With<Paddle>>,
) {
    for (player, mut transform) in &mut query {
        if !matches!(player, Player::One) {
            continue;
        }
        move_paddle(
            &mut transform,
            keyboard.pressed(KeyCode::KeyW),
            keyboard.pressed(KeyCode::KeyS),
            time.delta_secs(),
        );
    }
}

fn move_player_two(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform), With<Paddle>>,
) {
    for (player, mut transform) in &mut query {
        if !matches!(player, Player::Two) {
            continue;
        }
        move_paddle(
            &mut transform,
            keyboard.pressed(KeyCode::ArrowUp),
            keyboard.pressed(KeyCode::ArrowDown),
            time.delta_secs(),
        );
    }
}

fn clamp_paddles(mut query: Query<&mut Transform, With<Paddle>>) {
    let half_height = WINDOW_HEIGHT as f32 / 2.0;
    let min_y = -half_height + WALL_THICKNESS + PADDLE_HEIGHT / 2.0;
    let max_y = half_height - WALL_THICKNESS - PADDLE_HEIGHT / 2.0;

    for mut transform in &mut query {
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}

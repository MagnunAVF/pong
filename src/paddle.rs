use bevy::prelude::*;

use crate::{GameState, WINDOW_WIDTH};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles)
            .add_systems(Update, move_player_one.run_if(in_state(GameState::Playing)));
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

fn spawn_paddles(mut commands: Commands) {
    let half_width = WINDOW_WIDTH as f32 / 2.0;
    let size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

    // Player 1 — left paddle
    commands.spawn((
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

fn move_player_one(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform), With<Paddle>>,
) {
    for (player, mut transform) in &mut query {
        if !matches!(player, Player::One) {
            continue;
        }

        let mut direction = 0.0_f32;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction -= 1.0;
        }

        transform.translation.y += direction * PADDLE_SPEED * time.delta_secs();
    }
}

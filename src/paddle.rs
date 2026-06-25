use bevy::prelude::*;

use crate::WINDOW_WIDTH;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles);
    }
}

pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_X_OFFSET: f32 = 40.0;

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

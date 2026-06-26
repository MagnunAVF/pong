use bevy::prelude::*;

use crate::GameState;
use crate::ball::{BALL_SIZE, BALL_SPEED, Ball, Velocity};
use crate::paddle::{PADDLE_HEIGHT, PADDLE_WIDTH, Paddle, Player};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ball_vs_paddles.run_if(in_state(GameState::Playing)));
    }
}

const SPEED_INCREASE: f32 = 1.05;
const MAX_SPEED: f32 = BALL_SPEED * 2.0;

fn aabb_overlap(a: Vec2, a_half: Vec2, b: Vec2, b_half: Vec2) -> bool {
    (a.x - b.x).abs() < a_half.x + b_half.x && (a.y - b.y).abs() < a_half.y + b_half.y
}

fn accelerate(velocity: &mut Velocity) {
    let increased = velocity.0 * SPEED_INCREASE;
    velocity.0 = if increased.length() > MAX_SPEED {
        increased.normalize() * MAX_SPEED
    } else {
        increased
    };
}

fn ball_vs_paddles(
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    paddle_query: Query<(&Transform, &Player), (With<Paddle>, Without<Ball>)>,
) {
    let Ok((mut velocity, mut ball_tf)) = ball_query.single_mut() else {
        return;
    };

    let ball_half = Vec2::splat(BALL_SIZE / 2.0);
    let paddle_half = Vec2::new(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0);

    for (paddle_tf, player) in &paddle_query {
        let ball_pos = ball_tf.translation.truncate();
        let paddle_pos = paddle_tf.translation.truncate();

        if !aabb_overlap(ball_pos, ball_half, paddle_pos, paddle_half) {
            continue;
        }

        match player {
            // Ball approaching from the right of the left paddle
            Player::One if velocity.0.x < 0.0 => {
                velocity.0.x = -velocity.0.x;
                ball_tf.translation.x = paddle_pos.x + paddle_half.x + ball_half.x;
                accelerate(&mut velocity);
            }
            // Ball approaching from the left of the right paddle
            Player::Two if velocity.0.x > 0.0 => {
                velocity.0.x = -velocity.0.x;
                ball_tf.translation.x = paddle_pos.x - paddle_half.x - ball_half.x;
                accelerate(&mut velocity);
            }
            _ => {}
        }
    }
}

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
// Maximum outgoing angle from horizontal (steepest bounce at paddle edges)
const MAX_BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_2 * 5.0 / 6.0; // 75°

fn aabb_overlap(a: Vec2, a_half: Vec2, b: Vec2, b_half: Vec2) -> bool {
    (a.x - b.x).abs() < a_half.x + b_half.x && (a.y - b.y).abs() < a_half.y + b_half.y
}

/// Recompute velocity direction based on where the ball hit the paddle.
/// `x_dir` is +1.0 (ball leaves rightward) or -1.0 (ball leaves leftward).
fn deflect(velocity: &mut Velocity, ball_y: f32, paddle_y: f32, x_dir: f32) {
    let relative = ((ball_y - paddle_y) / (PADDLE_HEIGHT / 2.0)).clamp(-1.0, 1.0);
    let angle = relative * MAX_BOUNCE_ANGLE;
    let speed = velocity.0.length() * SPEED_INCREASE;
    let speed = speed.min(MAX_SPEED);
    velocity.0 = Vec2::new(x_dir * angle.cos(), angle.sin()) * speed;
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
            // Ball approaching from the right of the left paddle → leaves rightward
            Player::One if velocity.0.x < 0.0 => {
                ball_tf.translation.x = paddle_pos.x + paddle_half.x + ball_half.x;
                deflect(&mut velocity, ball_pos.y, paddle_pos.y, 1.0);
            }
            // Ball approaching from the left of the right paddle → leaves leftward
            Player::Two if velocity.0.x > 0.0 => {
                ball_tf.translation.x = paddle_pos.x - paddle_half.x - ball_half.x;
                deflect(&mut velocity, ball_pos.y, paddle_pos.y, -1.0);
            }
            _ => {}
        }
    }
}

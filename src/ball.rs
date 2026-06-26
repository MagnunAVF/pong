use bevy::prelude::*;
use rand::RngExt;

use crate::ui::WALL_THICKNESS;
use crate::{GameState, WINDOW_HEIGHT};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball).add_systems(
            Update,
            (move_ball, bounce_off_walls)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub const BALL_SIZE: f32 = 10.0;
pub const BALL_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn move_ball(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Ball>>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn bounce_off_walls(mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>) {
    let limit = WINDOW_HEIGHT as f32 / 2.0 - WALL_THICKNESS - BALL_SIZE / 2.0;

    for (mut velocity, mut transform) in &mut query {
        let y = transform.translation.y;
        if y > limit && velocity.0.y > 0.0 {
            velocity.0.y = -velocity.0.y;
            transform.translation.y = limit;
        } else if y < -limit && velocity.0.y < 0.0 {
            velocity.0.y = -velocity.0.y;
            transform.translation.y = -limit;
        }
    }
}

pub fn spawn_ball(mut commands: Commands) {
    let mut rng = rand::rng();

    // Angle between 30° and 60° from horizontal so the ball is never too flat or too steep
    let angle: f32 = rng.random_range(30.0_f32..60.0_f32).to_radians();
    let x_sign: f32 = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let y_sign: f32 = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

    let velocity = Vec2::new(x_sign * angle.cos(), y_sign * angle.sin()) * BALL_SPEED;

    commands.spawn((
        Ball,
        Velocity(velocity),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

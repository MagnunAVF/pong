use bevy::prelude::*;
use rand::RngExt;


pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

pub const BALL_SIZE: f32 = 10.0;
pub const BALL_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec2);

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

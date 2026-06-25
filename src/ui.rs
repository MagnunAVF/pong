use bevy::prelude::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_center_line, spawn_walls));
    }
}

// --- Center line ---

const DASH_WIDTH: f32 = 4.0;
const DASH_HEIGHT: f32 = 15.0;
const DASH_GAP: f32 = 15.0;

fn spawn_center_line(mut commands: Commands) {
    let half_height = WINDOW_HEIGHT as f32 / 2.0;
    let period = DASH_HEIGHT + DASH_GAP;
    let num_dashes = ((WINDOW_HEIGHT as f32) / period).ceil() as i32 + 1;

    for i in 0..num_dashes {
        let y = -half_height + (i as f32) * period + DASH_HEIGHT / 2.0;
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(DASH_WIDTH, DASH_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }
}

// --- Walls ---

pub const WALL_THICKNESS: f32 = 10.0;

#[derive(Component)]
pub struct Wall;

fn spawn_walls(mut commands: Commands) {
    let half_height = WINDOW_HEIGHT as f32 / 2.0;
    let wall_size = Vec2::new(WINDOW_WIDTH as f32, WALL_THICKNESS);

    for sign in [-1.0_f32, 1.0] {
        let y = sign * (half_height - WALL_THICKNESS / 2.0);
        commands.spawn((
            Wall,
            Sprite {
                color: Color::WHITE,
                custom_size: Some(wall_size),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }
}

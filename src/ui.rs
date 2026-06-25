use bevy::prelude::*;

use crate::WINDOW_HEIGHT;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_center_line);
    }
}

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

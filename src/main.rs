mod ball;
mod collision;
mod paddle;
mod score;
mod ui;

use ball::BallPlugin;
use bevy::{prelude::*, window::WindowResolution};
use collision::CollisionPlugin;
use paddle::PaddlePlugin;
use score::ScorePlugin;
use ui::UiPlugin;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<GameState>()
        .add_plugins((
            BallPlugin,
            CollisionPlugin,
            PaddlePlugin,
            ScorePlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

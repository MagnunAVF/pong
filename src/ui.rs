use bevy::prelude::*;

use crate::score::Winner;
use crate::{GameState, InGame, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_center_line, spawn_walls),
        )
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), despawn_menu)
        .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::Paused), spawn_pause_screen)
        .add_systems(OnExit(GameState::Paused), despawn_pause_screen)
        .add_systems(
            Update,
            handle_pause_input
                .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
        .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen)
        .add_systems(
            Update,
            handle_game_over_input.run_if(in_state(GameState::GameOver)),
        );
    }
}

// --- Center line ---

const DASH_WIDTH: f32 = 4.0;
const DASH_HEIGHT: f32 = 15.0;
const DASH_GAP: f32 = 15.0;

#[derive(Component)]
struct CenterLineDash;

fn spawn_center_line(mut commands: Commands, existing: Query<(), With<CenterLineDash>>) {
    if !existing.is_empty() {
        return;
    }
    let half_height = WINDOW_HEIGHT as f32 / 2.0;
    let period = DASH_HEIGHT + DASH_GAP;
    let num_dashes = ((WINDOW_HEIGHT as f32) / period).ceil() as i32 + 1;

    for i in 0..num_dashes {
        let y = -half_height + (i as f32) * period + DASH_HEIGHT / 2.0;
        commands.spawn((
            InGame,
            CenterLineDash,
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

fn spawn_walls(mut commands: Commands, existing: Query<(), With<Wall>>) {
    if !existing.is_empty() {
        return;
    }
    let half_height = WINDOW_HEIGHT as f32 / 2.0;
    let wall_size = Vec2::new(WINDOW_WIDTH as f32, WALL_THICKNESS);

    for sign in [-1.0_f32, 1.0] {
        let y = sign * (half_height - WALL_THICKNESS / 2.0);
        commands.spawn((
            InGame,
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

// --- Main menu ---

#[derive(Component)]
struct MenuScreen;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PONG"),
                TextFont {
                    font_size: FontSize::Px(96.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press Enter to Start"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

// --- Pause screen ---

#[derive(Component)]
struct PauseScreen;

fn spawn_pause_screen(mut commands: Commands) {
    commands
        .spawn((
            PauseScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press Escape to Resume"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_pause_screen(mut commands: Commands, query: Query<Entity, With<PauseScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

// --- Game over screen ---

#[derive(Component)]
struct GameOverScreen;

fn spawn_game_over_screen(mut commands: Commands, winner: Res<Winner>) {
    let winner_text = match &winner.0 {
        Some(crate::ball::Scorer::Player1) => "Player 1 Wins!",
        Some(crate::ball::Scorer::Player2) => "Player 2 Wins!",
        None => "It's a Draw!",
    };

    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(winner_text),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("Press R to Restart"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Menu);
    }
}

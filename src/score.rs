use bevy::prelude::*;

use crate::GameState;
use crate::ball::{BallExited, Scorer};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<Winner>()
            .add_observer(on_point_scored)
            .add_systems(Startup, spawn_score_hud)
            .add_systems(Update, update_score_hud)
            .add_systems(OnEnter(GameState::Menu), reset_score);
    }
}

pub const WIN_SCORE: u32 = 7;

#[derive(Resource, Default, Debug)]
pub struct Score {
    pub player1: u32,
    pub player2: u32,
}

/// Tracks which player won. Set when transitioning to GameOver, cleared on Menu reset.
#[derive(Resource, Default, Debug)]
pub struct Winner(pub Option<Scorer>);

#[derive(Component)]
enum ScoreDisplay {
    Player1,
    Player2,
}

fn spawn_score_hud(mut commands: Commands) {
    let text_font = TextFont {
        font_size: FontSize::Px(64.0),
        ..default()
    };

    // Player 1 score — left quarter
    commands.spawn((
        ScoreDisplay::Player1,
        Text::new("0"),
        text_font.clone(),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(160.0),
            ..default()
        },
    ));

    // Player 2 score — right quarter
    commands.spawn((
        ScoreDisplay::Player2,
        Text::new("0"),
        text_font,
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(160.0),
            ..default()
        },
    ));
}

fn update_score_hud(score: Res<Score>, mut query: Query<(&ScoreDisplay, &mut Text)>) {
    for (display, mut text) in &mut query {
        let value = match display {
            ScoreDisplay::Player1 => score.player1,
            ScoreDisplay::Player2 => score.player2,
        };
        text.0 = value.to_string();
    }
}

fn on_point_scored(
    trigger: On<BallExited>,
    mut score: ResMut<Score>,
    mut winner: ResMut<Winner>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match trigger.event().scorer {
        Scorer::Player1 => score.player1 += 1,
        Scorer::Player2 => score.player2 += 1,
    }

    if score.player1 >= WIN_SCORE {
        winner.0 = Some(Scorer::Player1);
        next_state.set(GameState::GameOver);
    } else if score.player2 >= WIN_SCORE {
        winner.0 = Some(Scorer::Player2);
        next_state.set(GameState::GameOver);
    }
}

fn reset_score(mut score: ResMut<Score>, mut winner: ResMut<Winner>) {
    *score = Score::default();
    winner.0 = None;
}

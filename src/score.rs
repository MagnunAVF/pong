use bevy::prelude::*;

use crate::GameState;
use crate::ball::{BallExited, Scorer};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_observer(on_point_scored)
            .add_systems(OnEnter(GameState::Menu), reset_score);
    }
}

#[derive(Resource, Default, Debug)]
pub struct Score {
    pub player1: u32,
    pub player2: u32,
}

fn on_point_scored(trigger: On<BallExited>, mut score: ResMut<Score>) {
    match trigger.event().scorer {
        Scorer::Player1 => {
            score.player1 += 1;
            info!("Player 1 scores! ({} - {})", score.player1, score.player2);
        }
        Scorer::Player2 => {
            score.player2 += 1;
            info!("Player 2 scores! ({} - {})", score.player1, score.player2);
        }
    }
}

fn reset_score(mut score: ResMut<Score>) {
    *score = Score::default();
}

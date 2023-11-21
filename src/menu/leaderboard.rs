use bevy::prelude::*;

#[derive(Resource)]
pub struct Leaderboard {
    pub leaderboard: Vec<Score>,
}

impl Leaderboard {
    pub fn new() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub score: i32,
    pub player: String,
}

impl Score {
    pub fn new() -> Self {
        todo!()
    }
}

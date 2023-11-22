use bevy::prelude::*;

#[derive(Resource, Clone, Default, Debug)]
pub struct Leaderboard {
    pub leaderboard: Vec<Score>,
}

impl Leaderboard {
    pub fn default() -> Self {
        Leaderboard {
            leaderboard: vec![Score { score: 0 }; 10],
        }
    }

    pub fn add_score(&mut self, score: i32) {
        self.leaderboard.sort();
        let pos = self
            .leaderboard
            .binary_search(&Score { score })
            .unwrap_or_else(|e| e);
        self.leaderboard.insert(pos, Score { score });
        self.leaderboard.reverse();
        self.leaderboard.pop();
    }
}

#[derive(Resource)]
pub struct PlayerName(pub String);

#[derive(Resource, Debug, Copy, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Score {
    pub score: i32,
}

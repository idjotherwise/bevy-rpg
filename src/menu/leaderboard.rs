use bevy::prelude::*;

#[derive(Resource, Clone, Default, Debug)]
pub struct Leaderboard {
    pub leaderboard: Vec<(PlayerName, Score)>,
}

impl Leaderboard {
    pub fn default() -> Self {
        Leaderboard {
            leaderboard: vec![(PlayerName("".to_string()), Score { score: 0 }); 10],
        }
    }

    pub fn add_score(&mut self, name: String, score: i32) {
        self.leaderboard.sort_by(|a, b| a.1.cmp(&b.1));
        let pos = self
            .leaderboard
            .binary_search_by(|(_, b)| b.cmp(&Score { score }))
            .unwrap_or_else(|e| e);
        self.leaderboard
            .insert(pos, (PlayerName(name), Score { score }));
        self.leaderboard.reverse();
        self.leaderboard.pop();
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerName(pub String);

// Unit struct to identify the player name text bundle
#[derive(Component)]
pub struct NameText;

impl PlayerName {
    pub fn set(&mut self, new_name: &String) {
        self.0 = new_name.to_string();
    }
}

#[derive(Resource, Debug, Copy, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Score {
    pub score: i32,
}

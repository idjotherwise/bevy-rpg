use bevy::prelude::*;

pub struct LevelPlugin;

#[derive(Resource, Debug, Copy, Default, Clone)]
pub struct Level {
    pub value: i32,
    pub exp_max: i32,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>();
    }
}

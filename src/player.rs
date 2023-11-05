use crate::{loading::TextureAssets, GameState};
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// Player related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        // .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            texture: textures.character.clone(),
            ..Default::default()
        })
        .insert(Player);
}

use crate::{actions::Actions, loading::TextureAssets, GameState};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// Player related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::new(2., 2., 1.)),
            texture: textures.character.clone(),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let half_player_size = 32.;
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    let x_min = -(window.width() / 2.0) + half_player_size;
    let x_max = window.width() / 2.0 - half_player_size;
    let y_min = -(window.height() / 2.0) + half_player_size;
    let y_max = window.height() / 2.0 - half_player_size;
    for mut player_transform in &mut player_query {
        let new_pos = player_transform.translation + movement;
        if new_pos.x > x_min && new_pos.x < x_max && new_pos.y > y_min && new_pos.y < y_max {
            player_transform.translation += movement;
        }
    }
}

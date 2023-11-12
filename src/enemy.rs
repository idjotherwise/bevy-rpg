use crate::{actions::Actions, loading::TextureAssets, GameState};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

/// Enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_systems(Update, move_enemy.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_enemy(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::new(2., 2., 1.)),
            texture: textures.monster.clone(),
            ..Default::default()
        })
        .insert(Enemy);
}

fn move_enemy(
    time: Res<Time>,
    actions: Res<Actions>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if actions.enemy_movement.is_none() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let half_enemy_size = 32.;
    let speed = 150.;
    let movement = Vec3::new(
        actions.enemy_movement.unwrap().x * speed * time.delta_seconds(),
        actions.enemy_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    let x_min = -(window.width() / 2.0) + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = -(window.height() / 2.0) + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;
    for mut enemy_transform in &mut enemy_query {
        let new_pos = enemy_transform.translation + movement;
        if new_pos.x > x_min && new_pos.x < x_max && new_pos.y > y_min && new_pos.y < y_max {
            enemy_transform.translation += movement;
        }
    }
}

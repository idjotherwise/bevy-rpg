use crate::{actions::Actions, loading::TextureAssets, GameState};
use bevy::prelude::*;

pub struct BulletPlugin;

#[derive(Component)]
pub struct Bullet;

/// Bullet related stuff like movement
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullet.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_bullet.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_bullet(actions: Res<Actions>, mut commands: Commands, textures: Res<TextureAssets>) {
    if actions.bullet_movement.is_none() {
        return;
    }
    println!("shooting at {:?}", actions.bullet_movement.unwrap());
    let (x, y) = (
        actions.bullet_movement.unwrap().x,
        actions.bullet_movement.unwrap().y,
    );
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            texture: textures.bolt.clone(),
            ..Default::default()
        })
        .insert(Bullet);
}

fn move_bullet(
    time: Res<Time>,
    actions: Res<Actions>,
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
) {
    if actions.bullet_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.bullet_movement.unwrap().x * speed * time.delta_seconds(),
        actions.bullet_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut bullet_transform in &mut bullet_query {
        bullet_transform.translation += movement;
    }
}

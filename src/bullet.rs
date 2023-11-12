use crate::player::Player;
use crate::{menu::MainCamera, GameState};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct BulletPlugin;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

/// Bullet related stuff like movement
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_bullet.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_bullet.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_bullet(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    player: Query<&Transform, With<Player>>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }
    let (camera, camera_transform) = q_camera.single();
    let window = q_windows.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    custom_size: Some(Vec2::new(4., 2.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    player.single().translation.x,
                    player.single().translation.y,
                    0.,
                )),
                ..default()
            })
            .insert(Bullet {
                lifetime: 1.,
                speed: 1.,
                direction: Vec2::new(world_position.x, world_position.y),
            });
    }
}

fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
) {
    for (mut bullet_transform, mut bullet, entity) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        let moving = bullet.direction * bullet.speed * time.delta_seconds();
        bullet_transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

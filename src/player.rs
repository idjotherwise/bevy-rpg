use crate::{actions::Actions, bullet::Bullet, enemy::Enemy, loading::TextureAssets, GameState};
use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
}

#[derive(Event, Default)]
pub struct Death {
    pub message: String,
}

/// Player related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_event::<Death>()
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_entities);
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
        .insert(Player {
            direction: Vec2::new(1., 0.).normalize(),
        });
}

fn move_player(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, Entity), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut collision_event: EventWriter<Death>,
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
    let (mut player_transform, player_entity) = player_query.single_mut();
    let new_pos = player_transform.translation + movement;
    if new_pos.x > x_min && new_pos.x < x_max && new_pos.y > y_min && new_pos.y < y_max {
        player_transform.translation += movement;
    }

    for enemy_transform in &enemy_query {
        let collision = collide(
            player_transform.translation,
            player_transform.scale.truncate() * 5.0,
            enemy_transform.translation,
            enemy_transform.scale.truncate() * 10.0,
        );
        if collision.is_some() {
            collision_event.send(Death {
                message: "The ninjas got to you!".to_string(),
            });
            next_state.set(GameState::Menu);
            commands.entity(player_entity).despawn();
        }
    }
}
fn cleanup_entities(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_enemy: Query<Entity, With<Enemy>>,
    q_bullets: Query<Entity, With<Bullet>>,
    q_camera: Query<Entity, With<Camera2d>>,
) {
    for entity in q_player.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in q_enemy.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in q_bullets.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in q_camera.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

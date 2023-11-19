use crate::{bullet::Bullet, loading::TextureAssets, GameState};
use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};
use rand::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub health: i32,
    collider: Collider,
}

// TODO: Move to own submodule

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

/// Enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
            .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_event::<CollisionEvent>()
            .add_systems(Update, move_enemy.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut timer: ResMut<SpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    (2. * random::<f32>() - 1.) * 100.,
                    (2. * random::<f32>() - 1.) * 100.,
                    1.,
                ))
                .with_scale(Vec3::new(2., 2., 1.)),
                texture: textures.monster.clone(),
                ..Default::default()
            })
            .insert(Enemy {
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                    .normalize(),
                health: 10,
                collider: Collider,
            });
    };
}

fn move_enemy(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, Option<&Enemy>), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = 32.;
    let speed = 90.;
    let x_min = -(window.width() / 2.0) + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = -(window.height() / 2.0) + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;
    for (_, mut enemy_transform, enemy) in &mut enemy_query {
        if let Some(enemy) = enemy {
            let movement = Vec3::new(
                enemy.direction.x * speed * time.delta_seconds(),
                enemy.direction.y * speed * time.delta_seconds(),
                0.,
            );
            let new_pos = enemy_transform.translation + movement;
            if new_pos.x > x_min && new_pos.x < x_max && new_pos.y > y_min && new_pos.y < y_max {
                enemy_transform.translation += movement;
            }
        }
    }
    for bullet_transform in &bullet_query {
        for (collider_entity, transform, maybe_enemy) in &enemy_query {
            let bullet_size = bullet_transform.scale.truncate();
            let collision = collide(
                bullet_transform.translation,
                bullet_size * 5.,
                transform.translation,
                transform.scale.truncate() * 5.,
            );
            if let Some(collision) = collision {
                collision_events.send_default();

                if maybe_enemy.is_some() {
                    commands.entity(collider_entity).despawn();
                }
            }
        }
    }
}

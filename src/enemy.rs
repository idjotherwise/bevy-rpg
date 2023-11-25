use crate::actions::Actions;
use crate::menu::Score;
use crate::player::Player;
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
    pub collider: Collider,
    pub direction_timer: Timer,
}

// TODO: Move to own submodule

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);
#[derive(Resource)]
pub struct SpawnTimerModifier(pub Timer);

/// Enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
            .insert_resource(SpawnTimer(Timer::from_seconds(5., TimerMode::Repeating)))
            .insert_resource(SpawnTimerModifier(Timer::from_seconds(
                20.,
                TimerMode::Repeating,
            )))
            .add_event::<CollisionEvent>()
            .add_systems(Update, move_enemy.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                update_spawn_timer.run_if(in_state(GameState::Playing)),
            );
    }
}

fn update_spawn_timer(
    time: Res<Time>,
    mut modify_timer: ResMut<SpawnTimerModifier>,
    mut timer: ResMut<SpawnTimer>,
) {
    if modify_timer.0.tick(time.delta()).just_finished() {
        timer.0 = Timer::from_seconds(timer.0.duration().as_secs_f32() / 2., TimerMode::Repeating)
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
                texture: textures.character.clone(),
                ..Default::default()
            })
            .insert(Enemy {
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                    .normalize(),
                health: 10,
                collider: Collider,
                direction_timer: Timer::from_seconds(rng.gen_range(1.0..2.0), TimerMode::Repeating),
            });
    };
}

fn move_enemy(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, Option<&mut Enemy>), With<Enemy>>,
    player_query: Query<&Transform, (Without<Enemy>, With<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = 32.;
    let speed = 90.;
    let x_min = -(window.width() / 2.0) + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = -(window.height() / 2.0) + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;
    for (_, mut enemy_transform, enemy) in &mut enemy_query {
        if let Some(mut enemy) = enemy {
            let movement = Vec3::new(
                enemy.direction.x * speed * time.delta_seconds(),
                enemy.direction.y * speed * time.delta_seconds(),
                0.,
            );
            let new_pos = enemy_transform.translation + movement;
            if new_pos.x > x_min && new_pos.x < x_max && new_pos.y > y_min && new_pos.y < y_max {
                enemy_transform.translation += movement;
            }
            enemy.direction_timer.tick(time.delta());
            if enemy.direction_timer.finished() {
                if actions.player_movement.is_none() {
                    let mut rng = rand::thread_rng();
                    let new_direction =
                        Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
                    enemy.direction = new_direction;
                } else {
                    let player_pos = player_query.single();
                    let new_direction =
                        Vec2::new(player_pos.translation.x, player_pos.translation.y).normalize();
                    enemy.direction = new_direction;
                };
            }
        }
    }
    for bullet_transform in &bullet_query {
        for (collider_entity, transform, maybe_enemy) in &enemy_query {
            let bullet_size = bullet_transform.scale.truncate();
            let collision = collide(
                bullet_transform.translation,
                bullet_size * 10.,
                transform.translation,
                transform.scale.truncate() * 5.,
            );
            if collision.is_some() {
                collision_events.send_default();

                // TODO: Decrease durability of bullet until it despawns
                // TODO: Increase a score counter when enemy is hit
                score.score += 1;
                if maybe_enemy.is_some() {
                    commands.entity(collider_entity).despawn();
                }
            }
        }
    }
}

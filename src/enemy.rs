use crate::actions::Actions;
use crate::item::Damage;
use crate::menu::Score;
use crate::player::{Experience, Player};
use crate::{item::Bullet, loading::TextureAssets, GameState};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub struct EnemyPlugin;

const INITIAL_SPAWN_TIMER: f32 = 5.0;

// #[derive(Component)]
// pub struct Collider;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub level: i32,
    // pub collider: Collider,
    pub direction_timer: Timer,
}

impl Enemy {
    pub fn new(level: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize(),
            level,
            // collider: Collider,
            direction_timer: Timer::from_seconds(rng.gen_range(1.0..2.0), TimerMode::Repeating),
        }
    }
}

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

impl SpawnTimer {
    pub fn reset(&mut self) {
        self.set(INITIAL_SPAWN_TIMER);
    }
    pub fn new(time: f32) -> Self {
        Self(Timer::from_seconds(time, TimerMode::Repeating))
    }
    pub fn set(&mut self, time: f32) {
        self.0 = Timer::from_seconds(time, TimerMode::Repeating);
    }
    pub fn halve(&mut self) {
        eprintln!("Halving duration");
        self.set(self.0.duration().as_secs_f32() / 2.);
    }
}

#[derive(Resource)]
pub struct SpawnTimerModifier(pub Timer);

/// Enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
            .insert_resource(SpawnTimer::new(INITIAL_SPAWN_TIMER))
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
        timer.halve()
    }
}

fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut timer: ResMut<SpawnTimer>,
    // TODO: Can this be a resource instead?
    player_query: Query<(&Transform, &mut Player), (Without<Enemy>, With<Player>)>,
    enemies_query: Query<&Enemy>,
) {
    // Cap number of enemies at 50
    if enemies_query.iter().count() > 50 {
        return;
    }

    let current_level = player_query.single().1.level;
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let new_enemy_level = rng.gen_range(current_level.value - 1..current_level.value + 3);
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
            .insert(Enemy::new(new_enemy_level));
    };
}

fn move_enemy(
    time: Res<Time>,
    actions: Res<Actions>,
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, Option<&mut Enemy>), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Player), (Without<Enemy>, With<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    bullet_query: Query<&Transform, (With<Damage>, Without<Enemy>)>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = 32.;
    let x_min = -(window.width() / 2.0) + half_enemy_size;
    let x_max = window.width() / 2.0 - half_enemy_size;
    let y_min = -(window.height() / 2.0) + half_enemy_size;
    let y_max = window.height() / 2.0 - half_enemy_size;
    let (player_pos, mut player) = player_query.single_mut();
    for (_, mut enemy_transform, enemy) in &mut enemy_query {
        if let Some(mut enemy) = enemy {
            let speed = 20.0 * enemy.level as f32;
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
                    let new_direction = Vec2::new(
                        player_pos.translation.x - enemy_transform.translation.x,
                        player_pos.translation.y - enemy_transform.translation.y,
                    )
                    .normalize();
                    enemy.direction = new_direction;
                };
            }
        }
    }
    for bullet_transform in &bullet_query {
        for (collider_entity, transform, maybe_enemy) in &enemy_query {
            let bullet_size = bullet_transform.scale.truncate();
            let collision = Aabb2d::new(
                bullet_transform.translation.truncate(),
                bullet_size * 10. / 2.,
            )
            .intersects(&Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() * 5. / 2.,
            ));
            if collision {
                collision_events.send_default();

                // TODO: Decrease durability of bullet until it despawns
                score.score += 1;
                // TODO: Make monster have experience value
                player.add_experience(Experience(1));
                if maybe_enemy.is_some() {
                    commands.entity(collider_entity).despawn_recursive();
                }
            }
        }
    }
}

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{enemy::Enemy, loading::TextureAssets, player::Player, GameState};

use super::Damage;

#[derive(Component)]
pub struct HomingMissile {
    target: Vec3,
    speed: f32,
    pub animation_timer: HomingAnimationTimer,
    lifetime: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct HomingAnimationTimer(Timer);

pub struct HomingMissilePlugin;
// Want to make a weapon which travells directly towards the nearest enemy and then follows it
fn spawn_homing_missile(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    enemies: Query<(&Transform, &Enemy)>,
    bullets: Query<&Damage>,
) {
    // Only spawn a missile if the user presses the E key and there are no more than 100 bullets
    if !keyboard_input.pressed(KeyCode::KeyE) || bullets.iter().count() > 100 {
        return;
    }

    let player_transform = player_query.single();
    let mut closest_enemy = None;
    let mut closest_distance = f32::INFINITY;
    for (enemy_transform, _) in enemies.iter() {
        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);
        if distance < closest_distance {
            closest_enemy = Some(enemy_transform);
            closest_distance = distance;
        }
    }
    if let Some(closest_enemy) = closest_enemy {
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform::from_translation(player_transform.translation)
                    .with_scale(Vec3::new(1.5, 1.5, 1.)),
                // sprite: TextureAtlasSprite::new(0),
                atlas: TextureAtlas {
                    layout: textures.shuriken_layout.clone(),
                    index: 0,
                },
                texture: textures.shuriken.clone(),
                ..Default::default()
            })
            .insert(HomingMissile {
                target: closest_enemy.translation,
                speed: 100.,
                animation_timer: HomingAnimationTimer(Timer::from_seconds(
                    0.1,
                    TimerMode::Repeating,
                )),
                lifetime: 10.,
            })
            .insert(Damage);
    }
}

// Update the missile to move towards the target
fn move_homing(
    time: Res<Time>,
    mut homing_missile_query: Query<(
        &mut Transform,
        &mut HomingMissile,
        Entity,
        &mut TextureAtlas,
    )>,
    mut commands: Commands,
) {
    for (mut missile_transform, mut missile, entity, mut atlas) in homing_missile_query.iter_mut() {
        // Adjust the missile's position towards the targets latest position
        let direction = missile.target - missile_transform.translation;
        let distance = direction.length();
        let velocity = direction.normalize() * missile.speed * time.delta_seconds();

        missile.lifetime -= time.delta_seconds();
        missile.animation_timer.tick(time.delta());
        // Update the missiles animation
        if missile.animation_timer.finished() {
            atlas.index = (atlas.index + 1) % 2;
        }

        if distance < velocity.length() {
            missile_transform.translation = missile.target;
        } else {
            missile_transform.translation += velocity;
        }
        // Despawn recursively when lifetime is 0
        if missile.lifetime <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for HomingMissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_homing_missile
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(Update, move_homing.run_if(in_state(GameState::Playing)));
    }
}

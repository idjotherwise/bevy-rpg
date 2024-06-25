use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, transform::commands};

use crate::{actions::Actions, enemy::Enemy, loading::TextureAssets, player::Player, GameState};

use super::Damage;

pub struct GranadePlugin;

#[derive(Component)]
pub struct Granade {
    pub target: Vec3,
    pub speed: f32,
    // pub animation_timer: GranadeAnimationTimer,
    pub lifetime: f32,
}

#[derive(Component)]
struct Explosion;

impl Plugin for GranadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_granade
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(Update, move_granade.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            remove_explosions
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_millis(5000))),
        );
    }
}

fn remove_explosions(mut commands: Commands, query: Query<(Entity, &Explosion)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_granade(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<(&Transform, &Player), With<Player>>,
    granades_query: Query<&Damage>,
) {
    if !keyboard_input.pressed(KeyCode::KeyR) {
        return;
    }
    // Only allow 1 granade at a time
    if granades_query.iter().count() > 0 {
        return;
    }

    let player_transform = player.single().0;

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(player_transform.translation)
                .with_scale(Vec3::new(0.5, 0.5, 1.)),
            texture: textures.bevy.clone(),
            ..Default::default()
        })
        .insert(Granade {
            target: Vec3::new(1., 1., 0.),
            speed: 100.,
            lifetime: 5.,
        })
        .insert(Damage);
}

fn move_granade(
    time: Res<Time>,
    mut commands: Commands,
    mut granade_query: Query<(&mut Transform, &mut Granade, Entity), With<Damage>>,
    enemies_query: Query<(&Transform, Entity, &Enemy), Without<Damage>>,
    textures: Res<TextureAssets>,
) {
    for (mut granade_transform, mut granade, granade_entity) in granade_query.iter_mut() {
        let direction = granade.target - granade_transform.translation;
        let distance = direction.length();
        let velocity = direction.normalize() * granade.speed * time.delta_seconds();
        if distance < velocity.length() {
            granade_transform.translation = granade.target;
            granade.lifetime = 0.;
            let explosion_bundle = SpriteBundle {
                transform: Transform::from_translation(granade_transform.translation),
                texture: textures.cactus.clone(),
                ..Default::default()
            };
            commands.spawn(explosion_bundle).insert(Explosion);
        } else {
            granade_transform.translation += velocity;
        }
        granade.lifetime -= time.delta_seconds();
        if granade.lifetime <= 0. {
            // Spawn an explosion and despawn all nearby enemies
            for (enemy_transform, enemy_entity, _) in enemies_query.iter() {
                if enemy_transform
                    .translation
                    .distance(granade_transform.translation)
                    < 1000.
                {
                    commands.entity(enemy_entity).despawn_recursive();
                }
            }

            commands.entity(granade_entity).despawn_recursive();
        }
    }
}

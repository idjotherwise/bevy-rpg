use std::time::Duration;

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub struct BulletPlugin;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub animation_timer: Timer,
}

/// Bullet related stuff like movement
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_bullet
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(Update, move_bullet.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_bullet(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    actions: Res<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    player: Query<(&Transform, &Player), With<Player>>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let bullet_direction = if actions.player_movement.is_none() {
        player.single().1.direction
    } else {
        actions.player_movement.unwrap()
    };
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(
                player.single().0.translation.x,
                player.single().0.translation.y,
                0.,
            )),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.shuriken.clone(),
            ..default()
        })
        .insert(Bullet {
            lifetime: 10.,
            speed: 200.,
            direction: bullet_direction,
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        });
}

fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity, &mut TextureAtlasSprite)>,
) {
    // TODO: Animate shuriken sprite when it is in
    for (mut bullet_transform, mut bullet, entity, mut sprite) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        bullet.animation_timer.tick(time.delta());
        if bullet.animation_timer.finished() {
            sprite.index = (sprite.index + 1) % 2;
        }
        // TODO: reduce speed to 0 then to -1 for boomerang effect. Or add curved effect by modifying the direction vec
        let moving = bullet.direction * bullet.speed * time.delta_seconds();
        bullet_transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

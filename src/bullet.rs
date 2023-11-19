use crate::actions::Actions;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

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
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(4., 2.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                player.single().0.translation.x,
                player.single().0.translation.y,
                0.,
            )),
            ..default()
        })
        .insert(Bullet {
            lifetime: 10.,
            speed: 200.,
            direction: bullet_direction,
        });
}

fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
) {
    // TODO: Animate shuriken sprite when it is in
    for (mut bullet_transform, mut bullet, entity) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        // TODO: reduce speed to 0 then to -1 for boomerang effect. Or add curved effect by modifying the direction vec
        let moving = bullet.direction * bullet.speed * time.delta_seconds();
        bullet_transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

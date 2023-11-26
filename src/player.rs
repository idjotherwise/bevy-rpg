use crate::{
    actions::Actions,
    bullet::Bullet,
    enemy::{Enemy, SpawnTimer},
    loading::TextureAssets,
    menu::{leaderboard::PlayerName, Leaderboard, Score},
    GameState,
};
use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};
use rand::seq::SliceRandom;

pub struct PlayerPlugin;

// #[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
// enum PlayerState {
//     #[default]
//     Hidden,
//     Moving,
// }

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
            .add_systems(OnExit(GameState::Playing), finish_level);
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 200., 1.))
                .with_scale(Vec3::new(2., 2., 1.)),
            texture: textures.cactus.clone(),
            ..Default::default()
        })
        .insert(Player {
            direction: Vec2::new(1., 0.).normalize(),
        });
}

#[allow(clippy::too_many_arguments)]
fn move_player(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Handle<Image>), With<Player>>,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut collision_event: EventWriter<Death>,
) {
    if actions.player_movement.is_none() {
        *player_query.single_mut().2 = textures.cactus.clone();
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
    let (mut player_transform, mut player, mut handle) = player_query.single_mut();
    *handle = textures.ninja.clone();
    player.direction = actions.player_movement.unwrap();
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
            let msgs = vec![
                "The ninjas got to you!",
                "Oh no you got hit again :(",
                "Did you try running away from the ninjas?",
                "Press Space to throw your shuriken!",
                "That was great, but you can do better!",
                "You need to practice turning into a cactus when you are still.",
            ];

            collision_event.send(Death {
                message: msgs
                    .choose(&mut rand::thread_rng())
                    .expect("No death message found")
                    .to_string(),
            });
            next_state.set(GameState::Menu);
        }
    }
}
fn finish_level(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    q_enemy: Query<Entity, With<Enemy>>,
    q_bullets: Query<Entity, With<Bullet>>,
    q_camera: Query<Entity, With<Camera2d>>,
    player_name: Res<PlayerName>,
    mut leaderboard: ResMut<Leaderboard>,
    mut score: ResMut<Score>,
    mut timer: ResMut<SpawnTimer>,
) {
    let maybe_name = &*player_name.0;
    let name = if maybe_name == "" {
        "Anonymous".to_string()
    } else {
        maybe_name.to_string()
    };
    leaderboard.add_score(name, score.score);
    score.score = 0;
    timer.reset();
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

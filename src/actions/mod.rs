use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
// use bevy::window::PrimaryWindow;

use crate::actions::game_control::{get_movement, GameControl};
use crate::player::Player;
use crate::GameState;

// use self::game_control::get_shoot;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (
                set_movement_actions.run_if(in_state(GameState::Playing)),
                // set_gun_actions.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub spawn_bullet: Option<bool>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position() {
        let (camera, camera_transform) = camera.single();
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
        {
            let diff = touch_position - player.single().translation.xy();
            if diff.length() > FOLLOW_EPSILON {
                player_movement = diff.normalize();
            }
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}

// pub fn set_gun_actions(
//     mut actions: ResMut<Actions>,
//     mut mycoords: ResMut<WorldCoords>,
//     keyboard_input: Res<Input<KeyCode>>,
//     player: Query<&Transform, With<Player>>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//     q_windows: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let (camera, camera_transform) = q_camera.single();
//     let window = q_windows.single();

//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         mycoords.0 = world_position;

//     //     let shoot_direction = get_shoot(GameControl::Shoot, &keyboard_input);
//     //     if shoot_direction != 0.0 {
//     //         if let Some(p) = Some(world_position) {
//     //             actions.bullet_movement = Some(p);
//     //         } else {
//     //             actions.bullet_movement = None;
//     //         }
//     //     } else {
//     //         actions.bullet_movement = None;
//     //     }
//     // }
// }

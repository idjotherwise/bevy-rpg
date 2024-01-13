use crate::player::Player;
use crate::{menu::Score, GameState};
use bevy::prelude::*;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ui)
            .add_systems(Update, update_score.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_level.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_ui);
    }
}

#[derive(Component)]
struct UIScore;
#[derive(Component)]
struct UILevel;
#[derive(Component)]
struct UIHud;

fn setup_ui(mut commands: Commands, score_q: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            },
            UIHud,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Score: {}", score_q.score),
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                UIScore,
                UIHud,
            ));
            parent.spawn((
                TextBundle::from_section(
                    format!("Level: {}, Exp: {}/{}", 1, 0, 10),
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                UILevel,
                UIHud,
            ));
        });
}

fn update_score(mut text_q: Query<&mut Text, With<UIScore>>, score_q: Res<Score>) {
    if score_q.is_changed() {
        for mut text in text_q.iter_mut() {
            text.sections[0].value = format!("Score {}", score_q.score)
        }
    }
}
fn update_level(
    mut text_q: Query<&mut Text, With<UILevel>>,
    player_q: Query<&Player>,
    score_q: Res<Score>,
) {
    let player = player_q.single();
    if score_q.is_changed() {
        for mut text in text_q.iter_mut() {
            text.sections[0].value = format!(
                "Level: {}, Exp: {}/{}",
                player.level.value, player.exp.0, player.level.exp_max
            );
        }
    }
}

fn cleanup_ui(mut commands: Commands, ui: Query<Entity, With<UIHud>>) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

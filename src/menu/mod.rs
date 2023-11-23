use bevy::prelude::*;

use crate::loading::TextureAssets;
pub use crate::menu::leaderboard::Leaderboard;
pub use crate::menu::leaderboard::Score;
use crate::player::Death;
use crate::GameState;

mod leaderboard;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .init_resource::<Score>()
            .insert_resource(Leaderboard::default())
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
pub struct MainCamera;

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    leaderboard: Res<Leaderboard>,
    mut message: EventReader<Death>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    if kbd.just_pressed(KeyCode::Return) {
        println!("Playing as: {}", &*string);
        string.clear();
    }
    if kbd.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in evr_char.iter() {
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }

    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        if message.is_empty() {
                            "Play".to_string()
                        } else {
                            "Restart".to_string()
                        },
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    left: Val::Px(10.),
                    height: Val::Percent(90.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            for score in &leaderboard.leaderboard {
                if score.score > 0 {
                    children.spawn({
                        TextBundle::from_section(
                            format!("Score: {}", score.score),
                            TextStyle {
                                font_size: 18.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                    });
                }
            }
        });

    if message.is_empty() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        bottom: Val::Px(5.),
                        width: Val::Percent(100.),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                },
                Menu,
            ))
            .with_children(|children| {
                children
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(170.0),
                                height: Val::Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceAround,
                                padding: UiRect::all(Val::Px(5.)),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        },
                        ButtonColors {
                            normal: Color::NONE,
                            ..default()
                        },
                        OpenLink("https://bevyengine.org"),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Made with Bevy",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                        parent.spawn(ImageBundle {
                            image: textures.bevy.clone().into(),
                            style: Style {
                                width: Val::Px(32.),
                                ..default()
                            },
                            ..default()
                        });
                    });
                children
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(170.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        },
                        ButtonColors {
                            normal: Color::NONE,
                            hovered: Color::rgb(0.25, 0.25, 0.25),
                        },
                        OpenLink("https://github.com/idjotherwise"),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Source",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                        parent.spawn(ImageBundle {
                            image: textures.bevy.clone().into(),
                            style: Style {
                                width: Val::Px(32.),
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
    } else {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        bottom: Val::Px(5.),
                        width: Val::Percent(100.),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                },
                Menu,
            ))
            .with_children(|children| {
                children.spawn(TextBundle::from_section(
                    message.iter().next().unwrap().message.to_string(),
                    TextStyle {
                        font_size: 15.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });

        message.clear();
    };
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    touch: Res<Touches>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if touch.first_pressed_position().is_some() {
        next_state.set(GameState::Playing);
    };
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

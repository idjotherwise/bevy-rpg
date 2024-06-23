use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInputBundle, TextInputInactive, TextInputPlugin, TextInputSubmitEvent,
};

use crate::loading::TextureAssets;
pub use crate::menu::leaderboard::Leaderboard;
use crate::menu::leaderboard::NameText;
pub use crate::menu::leaderboard::Score;
use crate::player::Death;
use crate::GameState;

use self::leaderboard::PlayerName;

pub mod leaderboard;
pub struct MenuPlugin;

const BORDER_COLOR_ACTIVE: Color = Color::VIOLET;
const BORDER_COLOR_INACTIVE: Color = Color::NONE;
const BACKGROUND_COLOR_ACTIVE: Color = Color::DARK_GRAY;
const BACKGROUND_COLOR_INACTIVE: Color = Color::WHITE;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .init_resource::<Score>()
            .init_resource::<PlayerName>()
            .insert_resource(Leaderboard::default())
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(Update, set_name.run_if(in_state(GameState::Menu)))
            .add_systems(Update, focus.run_if(in_state(GameState::Menu)))
            .add_plugins(TextInputPlugin);
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
    kbd: Res<ButtonInput<KeyCode>>,
    mut string: Local<String>,
) {
    if kbd.just_pressed(KeyCode::Enter) {
        string.clear();
    }
    if kbd.just_pressed(KeyCode::Backspace) {
        string.pop();
    }
    for ev in evr_char.read() {
        if !ev
            .char
            .chars()
            .last()
            .expect("Invalid char entered")
            .is_control()
        {
            string.push(ev.char.chars().last().expect("Invalid character entered"));
        }
    }

    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Interaction::None,
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BorderColor(BORDER_COLOR_ACTIVE),
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                },
                TextInputBundle::default()
                    .with_text_style(TextStyle {
                        font_size: 40.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    })
                    .with_placeholder("Enter name..", None)
                    .with_inactive(true),
            ));
        });
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
                        "Play".to_string(),
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
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::Start,
                    left: Val::Px(20.),
                    height: Val::Percent(80.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            for (name, score) in &leaderboard.leaderboard {
                if score.score > 0 {
                    children.spawn({
                        TextBundle::from_section(
                            format!("{}: {}", name.0, score.score),
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
                    message.read().next().unwrap().message.to_string(),
                    TextStyle {
                        font_size: 15.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });

        message.clear();
    };
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Name: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 40.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            }),
        ]),
        NameText,
        Menu,
    ));
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn set_name(
    mut events: EventReader<TextInputSubmitEvent>,
    mut name: ResMut<PlayerName>,
    mut q_name_text: Query<&mut Text, With<NameText>>,
    mut text_input_query: Query<(
        &mut TextInputInactive,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
) {
    for event in events.read() {
        info!("{:?} Setting name to: {}", event.entity, event.value);
        name.set(&event.value);
        for mut text in &mut q_name_text {
            text.sections[1].value = format!("{}", name.0)
        }
        for (mut inactive, mut border_color, mut background_color) in &mut text_input_query {
            inactive.0 = true;
            *border_color = BORDER_COLOR_INACTIVE.into();
            *background_color = BACKGROUND_COLOR_INACTIVE.into();
        }
    }
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(
        Entity,
        &mut TextInputInactive,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color, mut background_color) in
                &mut text_input_query
            {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                    *background_color = BACKGROUND_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                    *background_color = BACKGROUND_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

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

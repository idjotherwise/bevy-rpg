#![allow(clippy::type_complexity)]

mod actions;
mod enemy;
mod item;
mod level;
mod loading;
mod menu;
mod player;
mod ui;
use crate::actions::ActionsPlugin;
use crate::enemy::EnemyPlugin;
use crate::item::ItemPlugin;
use crate::level::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::prelude::*;
// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // add_state is renamed init_state in 0.13
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            LevelPlugin,
            ActionsPlugin,
            PlayerPlugin,
            UIPlugin,
            ItemPlugin,
            EnemyPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                WorldInspectorPlugin::new(),
            ));
        }
    }
}

use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "bevy_pixel_dark.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "character.png")]
    pub character: Handle<Image>,
    #[asset(path = "enemy.png")]
    pub monster: Handle<Image>,
}

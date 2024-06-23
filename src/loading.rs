use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "bevy_pixel_dark.png")]
    pub bevy: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 2, rows = 1))]
    pub shuriken_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler=nearest))]
    #[asset(path = "shuriken.png")]
    pub shuriken: Handle<Image>,
    #[asset(path = "character.png")]
    pub character: Handle<Image>,
    #[asset(path = "cactus.png")]
    pub cactus: Handle<Image>,
    #[asset(path = "enemy.png")]
    pub ninja: Handle<Image>,
}

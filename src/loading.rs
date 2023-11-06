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
    #[asset(path = "bolt.png")]
    pub bolt: Handle<Image>,
    // can handle padding with `padding_x` and `padding_y`
    // can handle space between top left corner and first sprite with `offset_x` and `offset_y`
    // #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
    // #[asset(path = "pack/#1 - Transparent Icons.png")]
    // pub icon_atlas: Handle<TextureAtlas>,
}

// fn draw_atlas(
//     mut commands: Commands,
//     texture_assets: Res<TextureAssets>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
// ) {
//     commands.spawn(Camera2dBundle::default());
//     let atlas = texture_atlases
//         .get(&texture_assets.icon_atlas)
//         .expect("Failed to find the atlas");
//     commands.spawn(SpriteBundle {
//         texture: atlas.texture.clone(),
//         transform: Transform::from_xyz(0.0, -150., 0.),
//         ..Default::default()
//     });
//     commands
//         .spawn(SpriteSheetBundle {
//             transform: Transform {
//                 translation: Vec3::new(0., 150., 0.0),
//                 ..Default::default()
//             },
//             sprite: TextureAtlasSprite::new(0),
//             texture_atlas: texture_assets.icon_atlas.clone(),
//             ..Default::default()
//         })
//         .insert(AnimationTimer(Timer::from_seconds(
//             0.1,
//             TimerMode::Repeating,
//         )));
// }

// #[derive(Component)]
// struct AnimationTimer(Timer);

// fn animate_sprite_system(
//     time: Res<Time>,
//     mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
// ) {
//     for (mut timer, mut sprite) in &mut query {
//         timer.0.tick(time.delta());
//         if timer.0.finished() {
//             sprite.index = (sprite.index + 1) % 8;
//         }
//     }
// }

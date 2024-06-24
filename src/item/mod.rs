pub use self::bullet::Bullet;
use self::bullet::BulletPlugin;
use bevy::prelude::*;

mod bullet;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BulletPlugin);
    }
}

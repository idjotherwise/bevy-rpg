pub use self::bullet::Bullet;
use self::bullet::BulletPlugin;
use self::granade::GranadePlugin;
use self::homing_missile::HomingMissilePlugin;
use bevy::prelude::*;

mod bullet;
mod granade;
mod homing_missile;

pub struct ItemPlugin;

#[derive(Component)]
pub struct Damage;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BulletPlugin, HomingMissilePlugin, GranadePlugin));
    }
}

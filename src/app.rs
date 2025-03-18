use crate::{bricks, player};
use bevy::{DefaultPlugins, app::App};

pub fn plugin(app: &mut App) {
    app.add_plugins((DefaultPlugins, bricks::plugin, player::plugin));
}

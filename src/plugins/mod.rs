mod assets;
mod input;
mod settings;

use self::settings::SettingsPlugin;
use crate::prelude::*;
use assets::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_wasm_window_resize::WindowResizePlugin;

/// Plugins required for displaying the game on a client device
pub struct ClientPlugins;

/// Initialize necessary client components
struct ClientInitPlugin;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ClientInitPlugin)
            .add(input::InputPlugin)
            .add(WindowResizePlugin)
    }
}

/// Initialize anything else needed for the client
impl Plugin for ClientInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}

mod assets;
mod input;
mod settings;

use self::settings::SettingsPlugin;
use crate::prelude::*;
use assets::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_wasm_window_resize::WindowResizePlugin;

/// Plugins required for core game functionality
pub struct CorePlugins;

/// Plugins required for displaying the game on a client device
pub struct ClientPlugins;

/// Initialize necessary core components
struct CoreInitPlugin;

/// Initialize necessary client components
struct ClientInitPlugin;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreInitPlugin)
            .add(AssetsPlugin)
            .add(SettingsPlugin)
    }
}

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ClientInitPlugin)
            .add(input::InputPlugin)
            .add(WindowResizePlugin)
    }
}

/// Initialize anything that is required for the core game to run
impl Plugin for CoreInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugins(DefaultPlugins.set(AssetPlugin::default()));
    }
}

/// Initialize anything else needed for the client
impl Plugin for ClientInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}

mod assets;
mod input;
mod settings;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::prelude::AppState;

/// Plugins required for displaying the game on a client device
pub struct ClientPlugins;

/// Initialize necessary client components
struct ClientInitPlugin;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ClientInitPlugin)
            .add(assets::AssetsPlugin)
            .add(settings::SettingsPlugin)
            .add(input::InputPlugin)
    }
}

/// Initialize anything else needed for the client
impl Plugin for ClientInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).init_state::<AppState>();
    }
}

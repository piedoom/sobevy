use bevy::prelude::*;

use crate::resources::Settings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, react_to_settings_change);
    }
}

/// If the settings file is changed, this system will react to those changes. We
/// use this for a few reasons:
/// 1. Adding the settings file as a resource. It's not as ergonomic to read an
///    asset as it is to read a regular `Resource` for use in other systems.
/// 2. We might want to do some app/window level changes, like quality
///    adjustments or (as shown here) changing the window mode
fn react_to_settings_change(
    mut cmd: Commands,
    mut settings_asset_events: EventReader<AssetEvent<Settings>>,
    mut window: Query<&mut Window>,
    settings_assets: Res<Assets<Settings>>,
) {
    for ev in settings_asset_events.read() {
        if let AssetEvent::Modified { id } | AssetEvent::LoadedWithDependencies { id } = ev {
            println!("Reloading settings...");
            if let Some(settings) = settings_assets.get(*id) {
                // Add the settings as a resource
                cmd.insert_resource(settings.clone());

                // Adjust the window mode
                window.iter_mut().for_each(|mut window| {
                    window.mode = settings.window.mode;
                });
            }
        }
    }
}

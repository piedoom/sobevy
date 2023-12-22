use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::Settings;

/// Assets loaded by [`bevy_asset_loader`]
#[derive(AssetCollection, Resource)]
pub struct Library {
    /// [`Settings`] `.ron` file
    #[asset(key = "settings")]
    pub settings: Handle<Settings>,
}

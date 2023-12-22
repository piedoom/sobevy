use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::prelude::*;

/// Loads all assets
pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Settings>::new(&["settings.ron"]))
            // Continue to the main game state once everything is loaded in, so
            // we can be sure all assets are loaded first
            .add_loading_state(
                LoadingState::new(AppState::preloading()).continue_to_state(AppState::loading()),
            )
            // This file specifies our collection of assets as strings, so that
            // assets can be modified or moved without the need of recompilation
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                AppState::preloading(),
                "default.assets.ron",
            )
            // Load all assets specified in our dynamic file
            .add_collection_to_loading_state::<_, Library>(AppState::preloading());
    }
}

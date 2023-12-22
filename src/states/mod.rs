use bevy::prelude::*;

/// Controls the state of our application
#[derive(Default, States, Debug, Clone, Eq)]
pub enum AppState {
    /// The starting state of the application where all necessary files and
    /// assets are preloaded before moving on to the loading stage
    #[default]
    Preloading,
    /// The state where specific assets should be prepared. For example, loading
    /// a level makes sense to put in this state.
    Loading,
    /// The main application state
    Main,
}

// Discard data so we can use the gamestate to hold relevant information
impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::hash::Hash for AppState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl AppState {
    /// AppState::Preloading { .. }
    #[inline(always)]
    pub fn preloading() -> Self {
        Self::Preloading
    }
    /// AppState::Loading { .. }
    #[inline(always)]
    pub fn loading() -> Self {
        Self::Loading
    }
    /// AppState::Main { .. }
    #[inline(always)]
    pub fn main() -> Self {
        Self::Main
    }
}

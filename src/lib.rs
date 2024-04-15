//! Game library

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![deny(missing_docs)]

mod components;
mod plugins;
mod resources;
mod states;
mod util;

/// Contains the most common types for our application
pub mod prelude {
    use super::*;
    #[allow(unused_imports)]
    pub use {components::*, plugins::*, resources::*, states::*, util::*};
}

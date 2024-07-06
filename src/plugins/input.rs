use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionState<Action>>()
            .insert_resource(
                InputMap::<Action>::default(), // Insert the control map here
            )
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Resource)]
pub enum Action {
    // Add actions here
}

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
  MoveLeft,
  MoveRight,
  Shoot,
}

impl Action {
  pub fn default_input_map() -> InputMap<Self> {
    let mut input_map = InputMap::default();

    input_map.insert(Self::MoveLeft, KeyCode::ArrowLeft);
    input_map.insert(Self::MoveRight, KeyCode::ArrowRight);
    input_map.insert(Self::Shoot, KeyCode::ArrowUp);
    input_map.insert(Self::Shoot, KeyCode::Space);

    input_map
  }
}

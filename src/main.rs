use avian2d::prelude::*;
use bevy::{
  prelude::*,
  render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
  },
};

pub mod alien;
pub mod input;
pub mod player;
pub mod resolution;

pub const SPACING: f32 = 32.0;

fn main() {
  let default_plugins = DefaultPlugins
    .set(WindowPlugin {
      primary_window: Some(Window {
        title: "Space Invaders".to_string(),
        position: WindowPosition::Centered(MonitorSelection::Primary),
        resolution: (800.0, 600.0).into(),
        ..Default::default()
      }),
      ..Default::default()
    })
    .set(ImagePlugin::default_nearest())
    .set(RenderPlugin {
      render_creation: RenderCreation::Automatic(WgpuSettings {
        backends: Some(Backends::VULKAN),
        ..default()
      }),
      ..default()
    });

  App::new()
    .add_plugins((
      default_plugins,
      resolution::ResolutionPlugin,
      alien::AlienPlugin,
      player::PlayerPlugin,
      PhysicsPlugins::default(),
      // PhysicsDebugPlugin::default(),
    ))
    .insert_resource(Gravity(Vec2::ZERO))
    .add_systems(Startup, set_up)
    .add_systems(Update, close_on_esc)
    .run();
}

fn set_up(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn close_on_esc(
  mut commands: Commands,
  focused_windows: Query<(Entity, &Window)>,
  input: Res<ButtonInput<KeyCode>>,
) {
  for (window, focus) in focused_windows.iter() {
    if !focus.focused {
      continue;
    }

    if input.just_pressed(KeyCode::Escape) {
      commands.entity(window).despawn();
    }
  }
}

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{resolution::Resolution, SPACING};

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_aliens);
    app.add_systems(Update, update_aliens);
  }
}

#[derive(Component)]
pub struct Alien;

const ROWS: u32 = 10;
const COLUMNS: u32 = 5;

fn spawn_aliens(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  resolution: Res<Resolution>,
) {
  let texture = asset_server.load("alien.png");

  let x_offset = Vec3::X * ROWS as f32 * SPACING * 0.5;
  let y_offset = Vec3::Y * COLUMNS as f32 * SPACING;
  let top_offset = Vec3::Y * resolution.screen_dimensions.y * 0.5;

  for row in 0..ROWS {
    for column in 0..COLUMNS {
      let initial_translation = Vec3::new(row as f32 * SPACING, column as f32 * SPACING, 0.0);

      // Aliens appear in the top middle of the screen
      let translation = initial_translation - x_offset - y_offset + top_offset;

      let transform =
        Transform::from_translation(translation).with_scale(Vec3::splat(resolution.pixel_ratio));

      let sprite_bundle = SpriteBundle {
        transform,
        texture: texture.clone(),
        ..Default::default()
      };

      commands.spawn((
        Alien,
        RigidBody::Dynamic,
        Collider::circle(5.0),
        Restitution::new(0.2),
        Mass(200.0),
        LinearVelocity(Vec2::new(50.0, 0.0)),
        Health(3),
        sprite_bundle,
      ));
    }
  }
}

fn update_aliens(
  alien_query: Query<&mut Transform, With<Alien>>,
  mut velocity_query: Query<&mut LinearVelocity, With<Alien>>,
  resolution: Res<Resolution>,
) {
  let aliens = alien_query.iter();
  let mut velocities = velocity_query.iter_mut();

  for alien in aliens {
    if alien.translation.x.abs() >= resolution.screen_dimensions.x * 0.5 - SPACING {
      for mut velocity in &mut velocities {
        velocity.0.x *= -1.0;
      }
      break;
    }
  }
}

#[derive(Component)]
pub struct Health(pub u32);

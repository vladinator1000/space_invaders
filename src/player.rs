use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{input::Action, resolution::Resolution, SPACING};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(InputManagerPlugin::<Action>::default())
      .add_systems(Startup, spawn_player)
      .add_systems(Update, (move_player, shoot));
  }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  resolution: Res<Resolution>,
) {
  let texture = asset_server.load("player.png");

  let transform = Transform::from_translation(Vec3::new(
    0.0,
    -resolution.screen_dimensions.y * 0.5 + SPACING,
    0.0,
  ))
  .with_scale(Vec3::splat(resolution.pixel_ratio));

  let sprite = SpriteBundle {
    transform,
    texture,
    ..Default::default()
  };

  commands.spawn((
    Player,
    InputManagerBundle::with_map(Action::default_input_map()),
    RigidBody::Dynamic,
    Collider::circle(5.0),
    LinearVelocity(Vec2::ZERO),
    sprite,
  ));
}

const PLAYER_SPEED: f32 = 80.0;
const MISSILE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Missile;

fn move_player(
  mut player_query: Query<(&ActionState<Action>, &Transform, &mut LinearVelocity), With<Player>>,
) {
  let (action, transform, mut velocity) = player_query.single_mut();

  if action.pressed(&Action::MoveLeft) {
    velocity.0.x = -PLAYER_SPEED;
  } else if action.pressed(&Action::MoveRight) {
    velocity.0.x = PLAYER_SPEED;
  } else {
    velocity.0.x = 0.0;
  }
}

fn shoot(
  mut commands: Commands,
  player_query: Query<(&ActionState<Action>, &Transform), With<Player>>,
  asset_server: Res<AssetServer>,
  resolution: Res<Resolution>,
) {
  let (action, player_transform) = player_query.single();

  if action.just_pressed(&Action::Shoot) {
    let translation = player_transform.translation + Vec3::new(0.0, SPACING, 0.0);
    let transform =
      Transform::from_translation(translation).with_scale(Vec3::splat(resolution.pixel_ratio));

    let sprite = SpriteBundle {
      transform,
      texture: asset_server.load("bullet.png"),
      ..Default::default()
    };

    commands.spawn((
      Missile,
      RigidBody::Dynamic,
      Collider::circle(2.0),
      LinearVelocity(Vec2::new(0.0, MISSILE_SPEED)),
      sprite,
    ));
  }
}

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
  alien::{Alien, Health},
  input::Action,
  resolution::Resolution,
  SPACING,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(InputManagerPlugin::<Action>::default())
      .add_systems(Startup, spawn_player)
      .add_systems(Update, (move_player, shoot, deal_damage));
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
    Health(1),
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
      Restitution::new(0.8),
      LinearVelocity(Vec2::new(0.0, MISSILE_SPEED)),
      sprite,
      Health(3),
    ));
  }
}

fn deal_damage(
  mut commands: Commands,
  mut alien_query: Query<(Entity, &mut Health, &CollidingEntities), With<Alien>>,
  mut missile_query: Query<&mut Health, (With<Missile>, Without<Alien>)>,
) {
  for (alien_entity, mut alien_health, colliding_entities) in &mut alien_query {
    for colliding_entity in colliding_entities.iter() {
      let colliding_entity = *colliding_entity;

      if let Ok(mut missile_health) = missile_query.get_mut(colliding_entity) {
        if alien_health.0 > 0 {
          alien_health.0 -= 1;
        }

        if alien_health.0 <= 0 {
          commands.entity(alien_entity).despawn();
        }

        if missile_health.0 > 0 {
          missile_health.0 -= 1;
        }

        if missile_health.0 <= 0 {
          commands.entity(colliding_entity).despawn();
        }
      }
    }
  }
}

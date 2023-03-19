use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    collider::Collider,
    stick::{Stick, StickPickedEvent, STICK_SIZE},
};

pub const PLAYER_WIDTH: f32 = 15.0;
pub const PLAYER_SPEED: f32 = 3.0;

pub const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_WIDTH);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(move_player_system)
            .add_system(check_player_stick_collision_system);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteBundle,
}

impl PlayerBundle {
    fn new(location: Vec3) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: location,
                    scale: Vec3::new(PLAYER_WIDTH, PLAYER_WIDTH, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::ALICE_BLUE,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle::new(Vec3::new(0.0, 0.0, 0.0)),
        Player,
        Collider,
    ));
}

fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= PLAYER_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += PLAYER_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += PLAYER_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= PLAYER_SPEED;
        }
    }
}

fn check_player_stick_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Player, &Transform, &Collider)>,
    mut stick_query: Query<(&Stick, &Transform, Entity, &Collider)>,
    mut stick_picked_events: EventWriter<StickPickedEvent>,
) {
    let player = player_query.single_mut();
    for (_stick, stick_transform, stick_entity, _stick_collider) in stick_query.iter_mut() {
        let collision = collide(
            player.1.translation,
            PLAYER_SIZE,
            stick_transform.translation,
            STICK_SIZE,
        );
        if collision.is_some() {
            commands.entity(stick_entity).despawn();
            stick_picked_events.send(StickPickedEvent);
        }
    }
}

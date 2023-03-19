use bevy::prelude::*;

use crate::stick::{Stick, StickPickedEvent, STICK_HEIGHT, STICK_WIDTH};

pub const PLAYER_WIDTH: f32 = 15.0;
pub const PLAYER_SPEED: f32 = 3.0;

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

fn setup_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(PLAYER_WIDTH, PLAYER_WIDTH, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                ..default()
            },
            ..default()
        },
        Player,
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
    mut player_query: Query<(&Player, &Transform)>,
    mut stick_query: Query<(&Stick, &Transform, Entity)>,
    mut stick_picked_events: EventWriter<StickPickedEvent>,
) {
    for (_player, player_transform) in player_query.iter_mut() {
        for (_stick, stick_transform, stick_entity) in stick_query.iter_mut() {
            if player_transform.translation.x + PLAYER_WIDTH / 2.0
                > stick_transform.translation.x - STICK_WIDTH / 2.0
                && player_transform.translation.x - PLAYER_WIDTH / 2.0
                    < stick_transform.translation.x + STICK_WIDTH / 2.0
                && player_transform.translation.y + PLAYER_WIDTH / 2.0
                    > stick_transform.translation.y - STICK_HEIGHT / 2.0
                && player_transform.translation.y - PLAYER_WIDTH / 2.0
                    < stick_transform.translation.y + STICK_HEIGHT / 2.0
            {
                commands.entity(stick_entity).despawn();
                stick_picked_events.send(default());
            }
        }
    }
}

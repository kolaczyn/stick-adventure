use bevy::prelude::*;

use crate::{
    collider::Collider, common::get_rand_screen_pos::get_rand_screen_pos, events::SpawnEnemyEvent,
};

pub const ENEMY_WIDTH: f32 = 100.0;
pub const ENEMY_HEIGHT: f32 = 400.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy_system);
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    sprite: SpriteBundle,
}

impl EnemyBundle {
    fn new(location: Vec3) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: location,
                    scale: Vec3::new(ENEMY_WIDTH, ENEMY_HEIGHT, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn spawn_enemy_system(
    mut commands: Commands,
    mut spawn_enemy_event_reader: EventReader<SpawnEnemyEvent>,
) {
    for _ in spawn_enemy_event_reader.iter() {
        let location = get_rand_screen_pos();
        commands.spawn((EnemyBundle::new(location), Enemy, Collider));
    }
}

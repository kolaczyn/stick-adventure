use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    common::{events::SpawnEnemyEvent, get_rand_screen_pos::get_rand_screen_pos},
    systems::collider::Collider,
};

use super::player::{Player, PLAYER_WIDTH};

pub const ENEMY_WIDTH: f32 = 100.0;
pub const ENEMY_HEIGHT: f32 = 400.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy_system)
            .add_system(check_player_collision_system);
    }
}

#[derive(Component)]
pub struct Enemy {
    active: bool,
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite: SpriteBundle,
}

impl EnemyBundle {
    fn new(location: Vec3, active: bool) -> Self {
        let color = if active {
            Color::RED.with_a(0.8)
        } else {
            Color::RED.with_a(0.35)
        };
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: location,
                    scale: Vec3::new(ENEMY_WIDTH, ENEMY_HEIGHT, 0.0),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
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
        let active = rand::random();
        commands.spawn((
            EnemyBundle::new(location, active),
            Enemy { active },
            Collider,
        ));
    }
}

fn check_player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Player, &Transform, Entity)>,
    mut enemy_query: Query<(&Enemy, &Transform)>,
) {
    for (_player, player_transform, player_entity) in player_query.iter_mut() {
        for (enemy, enemy_transform) in enemy_query.iter_mut() {
            if enemy.active {
                let player_pos = player_transform.translation;
                let enemy_pos = enemy_transform.translation;
                let player_size = Vec2::new(PLAYER_WIDTH, PLAYER_WIDTH);
                let enemy_size = Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT);
                if collide(player_pos, player_size, enemy_pos, enemy_size).is_some() {
                    // TODO send events and react accordingly
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
}

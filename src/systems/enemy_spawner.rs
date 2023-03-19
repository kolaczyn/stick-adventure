use bevy::prelude::*;

use crate::common::events::SpawnEnemyEvent;

const ENEMY_SPAWN_RATE_SECS: f32 = 1.0;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnerTimer>()
            .add_system(enemy_spawner_system);
    }
}

#[derive(Resource)]
struct EnemySpawnerTimer {
    timer: Timer,
}

impl Default for EnemySpawnerTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_RATE_SECS, TimerMode::Repeating),
        }
    }
}

// trigger the spawnEnemy event every ENEMY_SPAWN_RATE_SECS
fn enemy_spawner_system(
    time: Res<Time>,
    mut _commands: Commands,
    mut timer: ResMut<EnemySpawnerTimer>,
    mut spawn_enemy_event_writer: EventWriter<SpawnEnemyEvent>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        println!("Spawning enemy");
        spawn_enemy_event_writer.send(SpawnEnemyEvent);
    }
}

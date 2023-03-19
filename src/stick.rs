use bevy::prelude::*;
use rand::Rng;

pub const STICK_WIDTH: f32 = 20.0;
pub const STICK_HEIGHT: f32 = 80.0;

pub struct StickPlugin;

impl Plugin for StickPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sticks)
            .add_system(spawn_new_stick);
    }
}

#[derive(Component)]
pub struct Stick;

impl Stick {
    pub fn get_random_stick_location() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-400.0..400.0);
        Vec3::new(x, y, 0.0)
    }
}

#[derive(Bundle)]
struct StickBundle {
    sprite_bundle: SpriteBundle,
}

#[derive(Default)]
pub struct StickPickedEvent;

impl StickBundle {
    fn new(location: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location,
                    scale: Vec3::new(STICK_WIDTH, STICK_HEIGHT, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BISQUE,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup_sticks(mut commands: Commands) {
    commands.spawn((StickBundle::new(Vec3::new(-100.0, 0.0, 0.0)), Stick));
    commands.spawn((StickBundle::new(Vec3::new(100.0, 0.0, 0.0)), Stick));
}

fn spawn_new_stick(mut stick_picked_event: EventReader<StickPickedEvent>, mut commands: Commands) {
    if !stick_picked_event.iter().next().is_none() {
        commands.spawn((StickBundle::new(Stick::get_random_stick_location()), Stick));
    }
}

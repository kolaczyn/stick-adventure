use bevy::prelude::*;

const PLAYER_WIDTH: f32 = 15.0;
const PLAYER_SPEED: f32 = 3.0;

const STICK_WIDTH: f32 = 20.0;
const STICK_HEIGHT: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_player)
        .add_startup_system(setup_sticks)
        .add_system(move_player_system)
        .add_system(check_player_stick_collision_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct StickBundle {
    sprite_bundle: SpriteBundle,
}

#[derive(Component)]
struct Stick;

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
                    color: Color::RED,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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

fn setup_sticks(mut commands: Commands) {
    commands.spawn((StickBundle::new(Vec3::new(-100.0, 0.0, 0.0)), Stick));
    commands.spawn((StickBundle::new(Vec3::new(100.0, 0.0, 0.0)), Stick));
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
            }
        }
    }
}

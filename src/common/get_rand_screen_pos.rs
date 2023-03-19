use bevy::prelude::Vec3;
use rand::Rng;

use super::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn get_rand_screen_pos() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
    let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
    Vec3::new(x, y, 0.0)
}

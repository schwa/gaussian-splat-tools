use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::*;

use nalgebra::Vector3;


pub fn shuffle_splats(splats: &mut Vec<UberSplat>) {
    let mut rng = thread_rng();
    splats.shuffle(&mut rng);
}

pub fn find_center(splats: &Vec<UberSplat>) -> Vector3<f32> {
    let mut center = Vector3::new(0.0, 0.0, 0.0);
    for splat in splats {
        center += splat.position;
    }
    center / splats.len() as f32
}

pub fn move_splats(splats: &mut Vec<UberSplat>, offset: Vector3<f32>) {
    for splat in splats {
        splat.position += offset;
    }
}

pub fn center_splats(splats: &mut Vec<UberSplat>) {
    let center = find_center(splats);
    let offset = -center;
    move_splats(splats, offset);
}

use crate::utils;
use na::Vector3;

#[derive(Debug)]
pub struct Particle {
    pub mass: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
}

impl Particle {
    pub fn new(
        mass: f32,
        position: Vector3<f32>,
        velocity: Vector3<f32>,
        acceleration: Vector3<f32>,
    ) -> Self {
        Self {
            mass,
            position,
            velocity,
            acceleration,
        }
    }

    pub fn random_ensemble(
        n: u32,
        mass: f32,
        x_lim: [f32; 2],
        y_lim: [f32; 2],
        z_lim: [f32; 2],
    ) -> Vec<Self> {
        let mut ensemble = vec![];

        for _ in 0..n {
            let position = utils::random_vec(x_lim, y_lim, z_lim);
            let velocity = Vector3::new(0.0, 0.0, 0.0);
            let acceleration = Vector3::new(0.0, 0.0, 0.0);

            let particle = Particle::new(mass, position, velocity, acceleration);

            ensemble.push(particle);
        }

        ensemble
    }
}

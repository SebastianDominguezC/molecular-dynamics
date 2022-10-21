use crate::particle::Particle;
use na::Vector3;

pub fn harmonic_2d(k: f32, particle: &Particle) -> Vector3<f32> {
    let x = -k * particle.position.x;
    let y = -k * particle.position.y;
    Vector3::new(x, y, 0.0)
}

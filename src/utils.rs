use rand;
use rand::Rng;

use na::Vector3;

use crate::particle::Particle;

// A random vec
pub fn random_vec(x_lim: [f32; 2], y_lim: [f32; 2], z_lim: [f32; 2]) -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(x_lim[0] + 1.0..x_lim[1]);
    let y = rng.gen_range(y_lim[0] + 1.0..y_lim[1]);
    let z = rng.gen_range(z_lim[0] + 1.0..z_lim[1]);

    Vector3::new(x, y, z)
}

fn particle_ke(particle: &Particle) -> f32 {
    0.5 * particle.mass * particle.velocity.magnitude().powi(2)
}

fn ensemble_energy(ensemble: &Vec<Particle>) -> f32 {
    let mut energy = 0.0;
    for particle in ensemble {
        energy += particle_ke(particle);
    }
    energy
}

pub fn ensemble_temperature(kb: f32, ensemble: &Vec<Particle>) -> f32 {
    let avg_ke = ensemble_energy(ensemble) / ensemble.len() as f32;
    (2.0 * avg_ke) / (3.0 * kb)
}

use na::Vector3;
use rand;
use rand_distr::{Distribution, Normal};

use crate::forces::drag::viscocity_coef;

// Fluctation-dissipation: https://en.wikipedia.org/wiki/Fluctuation-dissipation_theorem
// Langevin: https://en.wikipedia.org/wiki/Langevin_dynamics
// Fokker-Planck: https://en.wikipedia.org/wiki/Fokker%E2%80%93Planck_equation
fn fd_coef(kb: f32, viscosity: f32, sig: f32, temperature: f32) -> f32 {
    let gamma = viscocity_coef(viscosity, sig);
    2.0 * gamma * kb * temperature
}

// Maybe add a Boltzmann dist
fn fd_langevin_gaussian(kb: f32, viscosity: f32, sig: f32, temperature: f32) -> f32 {
    let standard_normal = Normal::new(0.0, 1.0).expect("Not a valid distribution: Gaussian");
    let v = standard_normal.sample(&mut rand::thread_rng());
    let coef = fd_coef(kb, viscosity, sig, temperature).sqrt();
    v * coef
}

pub fn fd_random_gaussian_flux(
    kb: f32,
    viscosity: f32,
    sig: f32,
    temperature: f32,
) -> Vector3<f32> {
    let x = fd_langevin_gaussian(kb, viscosity, sig, temperature);
    let y = fd_langevin_gaussian(kb, viscosity, sig, temperature);
    let z = fd_langevin_gaussian(kb, viscosity, sig, temperature);

    Vector3::new(x, y, z)
}

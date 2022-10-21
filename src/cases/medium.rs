use crate::dynamics::viscous::viscous_molecular_dynamic;
use crate::particle::Particle;
use crate::utils::ensemble_temperature;
use na::Vector3;

pub fn main() {
    // Particles
    let n: u32 = 1;

    // Box dimensions
    let l = 2.6;
    let x_lim = [-l, l];
    let y_lim = [-l, l];
    let z_lim = [-l, l];

    // Particles
    // let ensemble = Particle::random_ensemble(n, 1.0, x_lim, y_lim, z_lim);
    let ensemble = vec![Particle::new(
        1.0,
        Vector3::zeros(),
        Vector3::zeros(),
        Vector3::zeros(),
    )];

    // Lennard Jones potential coefficients
    let sig = 1.0;
    let eps = 1.0;

    // Time for simulation
    let t = 10.0;
    let dt = 0.0001;
    let steps = (t / dt) as i32;

    // Boltzmann constant
    let kb = 1.0;

    // Viscosity of medium
    let viscosity = 1.0;

    // Temperature of system
    let temperature = 100.0;
    let temperature = temperature + ensemble_temperature(kb, &ensemble);

    // Data output, relative to base directory
    let data_path = String::from("./data/medium/");

    // Sim
    let ensemble = viscous_molecular_dynamic(
        n,
        dt,
        sig,
        eps,
        kb,
        viscosity,
        temperature,
        x_lim,
        y_lim,
        z_lim,
        steps,
        ensemble,
        data_path,
    );

    println!("{:#?}", ensemble);
}

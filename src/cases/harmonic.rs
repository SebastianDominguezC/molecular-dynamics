use crate::dynamics::harmonic_damped::harmonic_damped;
use crate::particle::Particle;
use crate::utils::ensemble_temperature;
use na::Vector3;

pub fn main() {
    // Particles
    let n: u32 = 1;

    // Particle
    let mut ensemble = vec![Particle::new(
        1.0,
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::zeros(),
    )];
    ensemble[0].velocity = Vector3::new(1.0, 1.0, 0.0);

    // Time for simulation
    let t = 20.0;
    let dt = 0.0001;
    let steps = (t / dt) as i32;

    // spring constant
    let k = 1.0;

    // Boltzmann constant
    let kb = 1.0;

    // Viscosity of medium
    let viscosity = 10.0;

    // Temperature of system
    let temperature = 10.0;
    let temperature = temperature + ensemble_temperature(kb, &ensemble);

    // Data output, relative to base directory
    let data_path = String::from("./data/harmonic/");

    // Sim
    let ensemble = harmonic_damped(
        n,
        dt,
        k,
        kb,
        viscosity,
        temperature,
        steps,
        ensemble,
        data_path,
    );

    println!("{:#?}", ensemble);
}

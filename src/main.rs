extern crate nalgebra as na;

mod dynamics;
mod forces;
mod particle;
mod utils;

use dynamics::simple::simple_molecular_dynamic;
use dynamics::viscous::viscous_molecular_dynamic;
use particle::Particle;
use utils::ensemble_temperature;

fn main() {
    simple();
    medium();
}

fn simple() {
    // Particles
    let n: u32 = 10;

    // Box dimensions
    let l = 4.0;
    let x_lim = [-l, l];
    let y_lim = [-l, l];
    let z_lim = [-l, l];

    // Particles
    let ensemble = Particle::random_ensemble(n, 1.0, x_lim, y_lim, z_lim);

    // Lennard Jones potential coefficients
    let sig = 1.0;
    let eps = 1.0;

    // Time for simulation
    let t = 10.0;
    let dt = 0.00001;
    let steps = (t / dt) as i32;

    // Data output, relative to base directory
    let data_path = String::from("./data/simple/");

    // Sim
    let ensemble = simple_molecular_dynamic(
        n, dt, sig, eps, x_lim, y_lim, z_lim, steps, ensemble, data_path,
    );

    println!("{:#?}", ensemble);
}

fn medium() {
    // Particles
    let n: u32 = 10;

    // Box dimensions
    let l = 4.0;
    let x_lim = [-l, l];
    let y_lim = [-l, l];
    let z_lim = [-l, l];

    // Particles
    let ensemble = Particle::random_ensemble(n, 1.0, x_lim, y_lim, z_lim);

    // Lennard Jones potential coefficients
    let sig = 1.0;
    let eps = 1.0;

    // Time for simulation
    let t = 20.0;
    let dt = 0.0001;
    let steps = (t / dt) as i32;

    // Boltzmann constant
    let kb = 1.0;

    // Viscosity of medium
    let viscosity = 0.25;

    // Temperature of system
    let temperature = 100.0;
    let temperature = temperature + ensemble_temperature(kb, &ensemble);

    // Data output, relative to base directory
    let data_path = String::from("./data/visc/");

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

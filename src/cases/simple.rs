use crate::dynamics::simple::simple_molecular_dynamic;
use crate::particle::Particle;

pub fn main() {
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
    let t = 1.0;
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

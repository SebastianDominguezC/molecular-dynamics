use csv::Writer;
use std::vec;

use crate::forces;
use crate::particle::Particle;
use forces::drag::{viscocity_coef, viscosity_drag};
use forces::fluctuations::fd_random_gaussian_flux;
use forces::lennard_jones::{lj_force_atoms, lj_force_walls};

pub fn viscous_molecular_dynamic(
    n: u32,
    dt: f32,
    sig: f32,
    eps: f32,
    kb: f32,
    viscosity: f32,
    temperature: f32,
    x_lim: [f32; 2],
    y_lim: [f32; 2],
    z_lim: [f32; 2],
    steps: i32,
    mut ensemble: Vec<Particle>,
    data_path: String,
) -> Vec<Particle> {
    // For data saving
    let mut wtr_x = Writer::from_path(data_path.clone() + "x.csv").expect("cant write");
    let mut wtr_y = Writer::from_path(data_path.clone() + "y.csv").expect("cant write");
    let mut wtr_z = Writer::from_path(data_path + "z.csv").expect("cant write");

    // Gamma viscosity coef
    let gamma = viscocity_coef(viscosity, sig);

    for j in 0..steps {
        // Coordinates of every particle per time step
        let mut x = vec![];
        let mut y = vec![];
        let mut z = vec![];

        for i in 0..n {
            // Current particle
            let particle = ensemble.get_mut(i as usize).expect("not a particle");
            let mass = particle.mass;
            let alpha = 1.0 - (gamma * dt) / (2.0 * mass);

            // Save position
            x.push(particle.position.x.to_string());
            y.push(particle.position.y.to_string());
            z.push(particle.position.z.to_string());

            // Update position
            let pos = particle.position
                + particle.velocity * dt * alpha
                + 0.5 * particle.acceleration * dt.powi(2);

            particle.position = pos;

            // Calculate acting force
            let f_flux = fd_random_gaussian_flux(kb, viscosity, sig, temperature);
            let f_drag = viscosity_drag(viscosity, sig, particle.velocity);
            let f_ext = lj_force_walls(sig, eps, particle, x_lim, y_lim, z_lim);
            let f_int = lj_force_atoms(sig, eps, i as usize, &ensemble);

            let f_tot = f_int + f_ext + f_drag + f_flux;

            // Redefine particle because of memory issue: probably can be fixed...
            let particle = ensemble.get_mut(i as usize).expect("not a particle");

            let curr_acc = particle.acceleration;
            let next_acc = f_tot / particle.mass;

            // update velocity and acceleration
            let vel = particle.velocity;
            particle.velocity = (1.0 / alpha) * (vel * alpha + 0.5 * (curr_acc + next_acc) * dt);
            particle.acceleration = next_acc;
        }

        // Write to buffer
        wtr_x.write_record(x).expect("Cant write to buffer X");
        wtr_y.write_record(y).expect("Cant write to buffer Y");
        wtr_z.write_record(z).expect("Cant write to buffer Z");

        // Just a print to see progress
        if j % 1000 == 0 {
            println!("{}", j);
        }
    }

    // Flush data "save and wipe it"
    wtr_x.flush().expect("Cant write to X");
    wtr_y.flush().expect("Cant write to Y");
    wtr_z.flush().expect("Cant write to Z");

    ensemble
}

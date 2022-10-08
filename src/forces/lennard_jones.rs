use crate::particle::Particle;
use na::Vector3;

// Lennard-Jones force based on radius: https://es.wikipedia.org/wiki/Potencial_de_Lennard-Jones
pub fn lj_force(sig: f32, eps: f32, r: f32) -> f32 {
    let t1 = 12.0 * sig.powi(12) / r.powi(13);
    let t2 = 6.0 * sig.powi(6) / r.powi(7);
    4.0 * eps * (t1 - t2)
}

// Lennard-Jones force between two particles
pub fn lj_force_atom(sig: f32, eps: f32, p1: &Particle, p2: &Particle) -> Vector3<f32> {
    let dir = p2.position - p1.position;
    let r = dir.magnitude();

    // Truncate force, for simplyfing calculations
    if r > 2.5 * sig {
        return Vector3::zeros();
    }

    -dir * lj_force(sig, eps, r)
}

// Lennard-Jones force a single particle receives from the ensemble
pub fn lj_force_atoms(sig: f32, eps: f32, i: usize, ensemble: &Vec<Particle>) -> Vector3<f32> {
    let mut net_f = Vector3::zeros();
    let p1 = ensemble.get(i).expect("Error, no particle");

    for (j, p) in ensemble.iter().enumerate() {
        // Exclude itself from calculation
        if i == j {
            continue;
        }
        let f = lj_force_atom(sig, eps, p1, p);
        net_f += f;
    }
    net_f
}

// Lennard-Jones force from the walls
pub fn lj_force_walls(
    sig: f32,
    eps: f32,
    particle: &Particle,
    x_lim: [f32; 2],
    y_lim: [f32; 2],
    z_lim: [f32; 2],
) -> Vector3<f32> {
    let pos = particle.position;
    let crit = 2.5 * eps;
    let mut f = Vector3::zeros();

    // All these ifs can probably be reduced...
    if pos.x < x_lim[0] + crit {
        let r = pos.x - x_lim[0];
        let r = r.abs();
        f += lj_force(sig, eps, r) * Vector3::x();
    }
    if pos.x > x_lim[1] - crit {
        let r = pos.x - x_lim[1];
        let r = r.abs();
        f += -lj_force(sig, eps, r) * Vector3::x();
    }

    if pos.y < y_lim[0] + crit {
        let r = pos.y - y_lim[0];
        let r = r.abs();
        f += lj_force(sig, eps, r) * Vector3::y();
    }
    if pos.y > y_lim[1] - crit {
        let r = pos.y - y_lim[1];
        let r = r.abs();
        f += -lj_force(sig, eps, r) * Vector3::y();
    }

    if pos.z < z_lim[0] + crit {
        let r = pos.z - z_lim[0];
        let r = r.abs();
        f += lj_force(sig, eps, r) * Vector3::z();
    }
    if pos.z > z_lim[1] - crit {
        let r = pos.z - z_lim[1];
        let r = r.abs();
        f += -lj_force(sig, eps, r) * Vector3::z();
    }

    f
}

use na::Vector3;
use std::f32::consts::PI;

// Viscocity: https://en.wikipedia.org/wiki/Viscosity
pub fn viscocity_coef(viscosity: f32, sig: f32) -> f32 {
    1.0 / (6.0 * PI * sig * viscosity)
}

pub fn viscosity_drag(viscosity: f32, sig: f32, vel: Vector3<f32>) -> Vector3<f32> {
    let gamma = viscocity_coef(viscosity, sig);
    -gamma * vel
}

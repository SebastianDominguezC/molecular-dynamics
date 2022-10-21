extern crate nalgebra as na;

mod cases;
mod dynamics;
mod forces;
mod particle;
mod utils;
fn main() {
    cases::simple::main();
    cases::medium::main();
    cases::harmonic::main();
}

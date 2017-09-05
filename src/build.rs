#[macro_use]
pub extern crate serde_derive;
pub extern crate toml;

pub mod types;

mod build_atoms;
mod build_trivial;

fn main() {
    build_atoms::atom_main();
    build_trivial::trivial_main();
}

mod algorithms;
mod analysis;
mod cli;
mod core;
mod modulo;
mod playbooks;
mod render;

// Input: -
// Calc:  Einstieg in die CLI
// Output: -
fn main() {
    cli::repl::run();
}

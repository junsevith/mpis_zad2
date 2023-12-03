mod dovecote;
mod simulation;
mod revfn;

fn main() {
    let mut sim = simulation::InitCond::new(1000);
    println!("{:?}", sim.run_multiple(50).0);
}

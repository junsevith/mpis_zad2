mod dovecote;
mod simulation;

fn main() {
    let mut sim = simulation::InitCond::new(10000);
    let res = sim.run_multiple(50);
    println!("{:?}", res.0);
}

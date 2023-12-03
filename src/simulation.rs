use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;

/// Stores initial conditions for the simulation
pub(crate) struct InitCond {
    size: usize,
    rng: Pcg64Mcg,
    range: Uniform<usize>,
}

impl InitCond {
    pub fn new(size: usize) -> Self {
        let rng = Pcg64Mcg::from_entropy();
        let range = Uniform::new(0, size);

        return InitCond {
            size,
            rng,
            range,
        };
    }

    pub fn run_sim(&mut self) -> crate::dovecote::DoveCote {
        let mut dovecote = crate::dovecote::DoveCote::new(self.size);
        while dovecote.all_two < self.size {
            let box_num = self.range.sample(&mut self.rng);
            dovecote.throw(box_num);
        }
        dovecote.wrap_up();
        return dovecote;
    }

    pub fn run_multiple(&mut self, times: usize) -> ([f64; 3], Vec<crate::dovecote::DoveCote>) {
        let mut results = Vec::new();
        let mut averages: [f64; 3] = [0.0; 3];
        for _i in 0..times {
            let dovecote = self.run_sim();
            averages[0] += dovecote.first_collision as f64;
            averages[1] += dovecote.all_one as f64;
            averages[2] += dovecote.all_two as f64;

            results.push(dovecote);
        }
        averages[0] /= times as f64;
        averages[1] /= times as f64;
        averages[2] /= times as f64;
        return (averages, results);
    }
}
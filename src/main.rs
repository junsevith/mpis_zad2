mod dovecote;
mod simulation;
mod chart;
mod extract;
mod functions;

fn main() {
    let sim_range = (1000..=100000usize).step_by(1000);
    let mut results = Vec::new();

    for i in sim_range.clone()  {
        let mut sim = simulation::InitCond::new(i);
        results.push(sim.run_multiple(50));
    }
    let fx_range = (sim_range.clone().next().unwrap() as f64)..(sim_range.clone().last().unwrap() as f64);

    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::b, functions::identity, "first_collision");
    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::b, functions::div_n, "first_collision_n");
    chart::draw_chart(fx_range.clone(), 5.0, &results, extract::b, functions::div_sqrtn, "first_collision_sqrt");

    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::u, functions::identity, "empty_boxes");
    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::u, functions::div_n, "empty_boxes_n");

    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::c, functions::identity, "all_one");
    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::c, functions::div_n, "all_one_n");
    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::c, functions::div_nln, "all_one_ln");
    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::c, functions::div_nsq, "all_one_sq");

    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::d, functions::identity, "all_two");
    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::d, functions::div_n, "all_two_n");
    chart::draw_chart(fx_range.clone(), 2.0, &results, extract::d, functions::div_nln, "all_two_ln");
    chart::draw_chart(fx_range.clone(), 1.1, &results, extract::d, functions::div_nsq, "all_two_sq");

    chart::draw_chart(fx_range.clone(), 4.0, &results, extract::dc, functions::identity, "all_delta");
    chart::draw_chart(fx_range.clone(), 4.0, &results, extract::dc, functions::div_n, "all_delta_n");
    chart::draw_chart(fx_range.clone(), 4.0, &results, extract::dc, functions::div_nln, "all_delta_ln");
    chart::draw_chart(fx_range.clone(), 4.0, &results, extract::dc, functions::div_nlnln, "all_delta_lnln");


}

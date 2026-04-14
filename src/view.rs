use comfy_table::{Table, presets::NOTHING};

use crate::models::{SimulationResult, WargameScenario};

/// Uses `comfy-table` to output the results of the simulations to the terminal.
pub fn print_result_table(sim_config: &WargameScenario, sim_result: &SimulationResult) {
    let mut table = Table::new();
    table.set_header(vec!["ITERATIONS", "SUCCESS COUNT", "SUCCESS %"]);

    table.load_preset(NOTHING);

    table.add_row(vec![
        sim_result.total_iterations.to_string(),
        sim_result.success_count.to_string(),
        sim_result.success_probability.to_string(),
    ]);

    println!("------------------------------------------------------------------------------");
    println!("Scenario: {}", sim_config.simulation.description);
    println!(
        "Success threshold: {}",
        sim_config.simulation.success_threshold
    );
    println!("------------------------------------------------------------------------------");
    println!("");
    println!("{}", table);
}

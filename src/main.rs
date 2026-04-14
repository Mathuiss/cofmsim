use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::mpsc,
};

use clap::{Arg, Command};
use csv::WriterBuilder;

use crate::{
    engine::SimulationEngine,
    error::SimError,
    models::{BattleResult, WargameScenario},
};

pub mod engine;
pub mod error;
pub mod models;
pub mod view;
pub mod writer;

fn main() -> Result<(), SimError> {
    let cmd = Command::new("cofmsim")
        .about(
            r#"
          /$$$$$$   /$$$$$$  /$$$$$$$$ /$$      /$$           /$$              
         /$$__  $$ /$$__  $$| $$_____/| $$$    /$$$          |__/              
        | $$  \__/| $$  \ $$| $$      | $$$$  /$$$$  /$$$$$$$ /$$ /$$$$$$/$$$$ 
        | $$      | $$  | $$| $$$$$   | $$ $$/$$ $$ /$$_____/| $$| $$_  $$_  $$
        | $$      | $$  | $$| $$__/   | $$  $$$| $$|  $$$$$$ | $$| $$ \ $$ \ $$
        | $$    $$| $$  | $$| $$      | $$\  $ | $$ \____  $$| $$| $$ | $$ | $$
        |  $$$$$$/|  $$$$$$/| $$      | $$ \/  | $$ /$$$$$$$/| $$| $$ | $$ | $$
        \______/  \______/ |__/      |__/     |__/|_______/ |__/|__/ |__/ |__/

        Stochastic Correlation of Forces and Means Simulator."#,
        )
        .version("1.1")
        .arg(
            Arg::new("scenario")
                .default_value("scenario.toml")
                .required(false)
                .help("Path to the scenario file."),
        )
        .arg(
            Arg::new("output")
                .required(false)
                .short('o')
                .long("output")
                .help("Path to the output file. Example: ./output.csv"),
        );
    let matches = cmd.get_matches();

    // Load cli args
    let scenario = matches.get_one::<String>("scenario").unwrap();
    let output_file = matches.get_one::<String>("output");

    // Load scenario file
    let mut file_content = String::new();
    BufReader::new(File::open(PathBuf::from(scenario))?).read_to_string(&mut file_content)?;
    let sim_config = toml::from_str::<WargameScenario>(&file_content)?;

    // Apply engine settings based on output args
    let engine;
    if let Some(file) = output_file {
        let mut output_path = PathBuf::from(file.trim());

        // Check extension
        if !output_path.ends_with(".csv") {
            output_path.add_extension("csv");
        }

        // Set up MPSC channels for async CSV writing thread
        let (tx, rx) = mpsc::channel::<BattleResult>();

        // Spawn engine with MPSC transmitter
        engine = SimulationEngine::new(&sim_config, Some(tx));

        // Spawn CSV writer thread
        writer::spawn_writer(rx, WriterBuilder::new().from_path(output_path)?);
    } else {
        // Spawn engine without MPSC transmitter
        engine = SimulationEngine::new(&sim_config, None);
    }

    // Run the simulations
    let result = engine.run();

    // Print results to screen
    view::print_result_table(&sim_config, &result);

    Ok(())
}

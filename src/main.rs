use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::mpsc,
};

use clap::{Arg, Command};
use csv::WriterBuilder;

use crate::{
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
        .version("1.1.3")
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
    let result = if let Some(file) = output_file {
        let mut output_path = PathBuf::from(file.trim());

        // Check extension
        if let None = output_path.extension() {
            output_path.add_extension("csv");
        }

        // Set up MPSC channels for async CSV writing thread
        let (tx, rx) = mpsc::channel::<BattleResult>();

        // Spawn CSV writer thread
        let handle = writer::spawn_writer(rx, WriterBuilder::new().from_path(output_path)?);

        // Run simulations
        let result = engine::run(&sim_config, Some(tx));

        // Wait to finish write operations
        _ = handle.join();

        result
    } else {
        // Run simulations
        engine::run(&sim_config, None)
    };

    // Print results to screen
    view::print_result_table(&sim_config, &result);

    Ok(())
}

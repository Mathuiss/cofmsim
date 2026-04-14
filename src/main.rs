use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use clap::{Arg, Command};

use crate::{engine::SimulationEngine, error::SimError, models::WargameScenario};

pub mod engine;
pub mod error;
pub mod models;
pub mod view;

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
        .version("1.0")
        .arg(
            Arg::new("scenario")
                .default_value("scenario.toml")
                .required(false)
                .help("Path to the scenario file."),
        );

    let matches = cmd.get_matches();

    let scenario = matches.get_one::<String>("scenario").unwrap();
    let mut file_content = String::new();
    BufReader::new(File::open(PathBuf::from(scenario))?).read_to_string(&mut file_content)?;

    let sim_config = toml::from_str::<WargameScenario>(&file_content)?;
    let engine = SimulationEngine::new(&sim_config);

    let result = engine.run();
    view::print_result_table(&sim_config, &result);

    Ok(())
}

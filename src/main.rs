use clap::Parser;
use log::{error, info};
use std::error::Error;

mod engines;

/// Command Line Arguments
#[derive(Parser, Debug)]
#[command(version, about="Differential Tester for WASI implementations", long_about = None)]
struct CLI {
    /// Enumerated CLI arguments (for single-run configs)
    #[arg(short = 'a', long = "args", num_args=0..)]
    runargs: Option<Vec<String>>,

    /// File to read arguments from (for multi-run configs)
    #[arg(short, long)]
    runfile: Option<String>,

    /// Wasm binary file to test
    #[arg(short = 'f', long = "file")]
    infile: String,
}

impl CLI {
    /// Log the command line arguments
    fn print(&self) {
        info!("Input file: {}", self.infile);
        if let Some(runfile) = &self.runfile {
            info!("RunFile: {:?}", runfile);
        } else if let Some(runargs) = &self.runargs {
            info!("RunArgs: {:?}", runargs);
        } else {
            error!("No runfile or runargs provided");
            panic!();
        }
    }
}

/// Program entrypoint
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();
    let cli = CLI::parse();
    cli.print();
    info!("Starting differential testing...");
    info!("Exiting process");
    engines::engine_variants();
    Ok(())
    // Err("Error in process".into())
}

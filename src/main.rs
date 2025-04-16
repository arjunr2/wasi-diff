use clap::Parser;
use log::info;
use std::error::Error;

mod engines;

/// Command Line Arguments
#[derive(Parser, Debug)]
#[command(version, about="Differential Tester for WASI implementations", long_about = None)]
struct CLI {
    /// File to read arguments from (for multi-run configs)
    #[arg(short, long)]
    runfile: Option<String>,

    /// Run Commands (Wasm binary + Args) to test
    #[arg(short, long, num_args=1..)]
    command: Vec<String>,
}

impl CLI {
    /// Log the command line arguments
    fn print(&self) {
        info!("Input file: {}", self.command[0]);
        if let Some(runfile) = &self.runfile {
            info!("RunFile: {:?}", runfile);
        } else {
            info!("RunArgs: {:?}", self.command);
        }
    }
}

/// Program entrypoint
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();
    let cli = CLI::parse();
    cli.print();
    info!("Starting differential testing...");
    engines::dispatch_all(&cli.command);
    info!("Exiting process");
    Ok(())
    // Err("Error in process".into())
}

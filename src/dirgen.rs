use anyhow::Result;
use clap::Parser;
use ftzz::{Generator, NumFilesWithRatio};
use log::{debug, info};
use rand::{Rng, SeedableRng, rngs::StdRng};
use random_string;
use std::fs::{self};
use std::num::NonZeroU64;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use walkdir::WalkDir;

use std::time::{SystemTime, UNIX_EPOCH};

/// Get the current time in seconds since the UNIX epoch
fn get_current_time_seconds() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs()
}

/// Command Line Arguments
#[derive(Parser, Debug)]
#[command(version, about="Random Filetree Generation with permissions", long_about = None)]
struct CLI {
    /// Number of files to generate
    #[arg(short = 'n', long)]
    files: u32,

    /// Average bytes per file
    #[arg(short = 'x', long, default_value_t = 4096)]
    bytes_per_file: u32,

    /// Max directory depth
    #[arg(short = 'd', long, default_value_t = 5)]
    max_depth: u32,

    /// Average number of files per directory
    #[arg(short = 'r', long)]
    ftd_ratio: u32,

    /// Seed
    #[arg(short = 's', long, default_value_t = get_current_time_seconds())]
    seed: u64,

    root_dir: String,
}

impl CLI {
    /// Print all arguments
    fn print(&self) {
        debug!("Root Dir: {}", self.root_dir);
        debug!("Num Files: {}", self.files);
        debug!("Bytes per file: {}", self.bytes_per_file);
        debug!("Max depth: {}", self.max_depth);
        debug!("Average files per directory: {}", self.ftd_ratio);
        info!("Seed: {}", self.seed);
    }
}

/// Generate a random file tree with FTZZ passthrough
fn run_ftzz(cli: &CLI) -> Result<&str> {
    let generator = Generator::builder()
        .root_dir(cli.root_dir.parse()?)
        .num_files_with_ratio(NumFilesWithRatio::new(
            NonZeroU64::new(cli.files as u64).unwrap(),
            NonZeroU64::new(cli.ftd_ratio as u64).unwrap(),
        )?)
        .files_exact(false)
        .bytes_exact(false)
        .seed(cli.seed)
        .num_bytes(cli.bytes_per_file as u64 * cli.files as u64)
        .max_depth(cli.max_depth)
        .build();

    let mut output = String::new();
    generator.generate(&mut output).unwrap();
    info!(target: &format!("{}::ftzz", module_path!()), "{}", output);
    Ok(&cli.root_dir)
}

/// Randomize directory permissions and filenames
fn randomize_dir(root_dir: &str, seed: u64) -> Result<()> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let filename_charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._-";

    for entry in WalkDir::new(&root_dir).contents_first(true) {
        let entry = entry?;
        // Don't rename root
        if entry.file_type().is_dir() && entry.path() == Path::new(root_dir) {
            continue;
        }
        // Set permissions
        let new_perm = rng.random_range(0..=0o777);
        let perm = PermissionsExt::from_mode(new_perm);
        fs::set_permissions(entry.path(), perm)?;
        // Rename file
        let mut path = entry.clone().into_path();
        let file_rename = random_string::generate_rng(1..256, filename_charset);
        path.pop();
        path.push(file_rename.as_str());
        fs::rename(entry.path(), &path)?;

        debug!("{:?} | {}", &path, new_perm);
    }

    // Walk directory tree again to generate map
    let all_files: String = WalkDir::new(&root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| format!("{}\n", e.path().to_str().unwrap()))
        .collect::<String>();

    let map_path = Path::new(root_dir).join("treemap.txt");
    info!("Writing tree map to {:?}", map_path);
    fs::write(map_path, all_files)?;

    Ok(())
}

/// Entrypoint
fn main() -> Result<()> {
    env_logger::builder().format_timestamp_millis().init();
    let cli = CLI::parse();
    cli.print();
    if cli.seed == 0 {}
    let root_dir = run_ftzz(&cli)?;
    randomize_dir(root_dir, cli.seed)?;
    Ok(())
}

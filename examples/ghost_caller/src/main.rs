use clap::Parser;
use std::fmt::Display;
use std::path::PathBuf;
use uka_shiori::dll::ShioriCaller;
use uka_shiori::types::v3::{Request, Version};

#[derive(Parser)]
pub struct Args {
    #[arg(help = "Path to the ghost file")]
    path: PathBuf,

    #[arg(long, help = "Version of the ghost")]
    version: Version,

    #[arg(long, required = true, help = "Identifier of the ghost")]
    id: String,
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ghost {} --id {}", self.path.display(), self.id)
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let mut builder = Request::builder();

    let request = builder.build()?;

    let caller = unsafe { ShioriCaller::open(args.path.as_path()) }.expect("failed to open ghost");

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

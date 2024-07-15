use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(value_name = "input file")]
    pub file: PathBuf,

    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub debug: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help="Disable scan  optimization ([>], [<])")]
    pub f_no_optimize_scan: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help="Disable clear optimization ([-])")]
    pub f_no_optimize_clear: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help="Disable loop  optimization ([[...]])")]
    pub f_no_optimize_loops: bool,
}

pub fn parse_args() -> Cli {
    let cli = Cli::parse();
    cli
}

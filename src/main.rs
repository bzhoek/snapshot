use clap::Parser;
use env_logger::Env;
use env_logger::Target::Stdout;
use log::debug;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// URL to compare snapshot of
  #[arg(short, long)]
  url: String,
}

fn main() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info,headless_chrome=warn")).target(Stdout).init();
  debug!("Debug logging");

  let args = Args::parse();
}

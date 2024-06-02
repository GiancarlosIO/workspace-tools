use clap::{Args, Parser};
use napi::bindgen_prelude::*;

#[derive(Debug, Parser, Clone)]
#[command(
  author = "Giancarlos Isasi (giancarlo.isasi@gmail.com",
  about = "A set of tools to manage your workspace",
  long_about = None,
  version = "0.0.1",
)]
#[command(propagate_version = true)]
pub struct Cli {
  /// Setup prettier
  // #[command(flatten)]
  // prettier: Prettier,
  #[arg(short, long)]
  pub name: Option<bool>,
}

// #[derive(Debug, Args)]
// #[command(propagate_version = true)]
// pub struct Prettier {
//   // setup prettier
//   #[command(flatten)]
//   pub setup: PrettierSetupOptions,
// }

// #[derive(Debug, Args)]
// pub struct PrettierSetupOptions {
//   /// use a template
//   #[arg(short = 't', long)]
//   pub template: Option<String>,
// }

impl Cli {
  pub fn new(args: Vec<String>) -> Result<Self> {
    match Cli::try_parse_from(args) {
      Ok(cli) => Ok(cli),
      Err(_) => Err(napi::Error::from_reason(
        "Failed to parse the CLI arguments. Make sure to use the CLI with valid syntax and arguments.",
      )),
    }
  }

  pub fn run(&self) {
    println!("{:?}", self);
  }
}

use clap::{Args, Parser, Subcommand};
use napi::bindgen_prelude::Result;

/// Note: flatten will delegate all the commands (includes version, help) to the commamnd/field defined.
/// That's why when you use this the --help and --version command doesn't work anymore in the root only in subcommands

#[derive(Debug, Parser)]
#[command(
  author = "Giancarlos Isasi (giancarlo.isasi@gmail.com",
  about = "A set of tools to manage your workspace.",
  long_about = None,
  version = "0.0.1",
  // disable_colored_help = true,
  // disable_help_flag = true,
  // disable_help_subcommand = true,
  // disable_version_flag = true,
  ignore_errors = false,
  propagate_version = true,
  // this is require when using the Clap::parse_from method
  // without this, clap will use the first element of the Vec<String>
  // as the binary_name (the runnable command that is being used)
  no_binary_name = true
)]
pub struct Cli {
  // Setup prettier
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Run tasks related to prettier
  Prettier(PrettierArgs),
  /// Run eslint tasks
  Eslint,

  /// Run Jest tasks
  Jest,

  /// Configure all workspaces using prettier, eslint and jest
  Configure {
    /// The folder path of the template files that we are going to use to configure the workspace.
    #[arg(short = 't', long)]
    templates: Option<String>,
  },
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PrettierArgs {
  /// Configure prettier
  #[command(subcommand)]
  pub command: PrettierCommands,
}

#[derive(Subcommand, Debug)]
pub enum PrettierCommands {
  /// Configure prettier by creating a prettier.config.js and .prettierignore file in each workspace.
  Configure {
    /// specify a path to a template files. This folder must contain the prettier.config.js and .prettierignore files.
    #[arg(short = 't', long)]
    template: Option<String>,
  },
}

pub fn create_cli(args: Vec<String>) -> Result<Cli> {
  // Arguments are coming from bin.mjs
  let cli = Cli::parse_from(args);

  Ok(cli)
}

pub fn run(cli: Cli) -> Result<()> {
  println!("{:?}", cli);

  match cli.command {
    Commands::Prettier(args) => {
      println!("prettier args {:?}", args);
      println!("You are running the prettier command")
    }
    _ => {
      println!("running other commands")
    }
  }

  Ok(())
}

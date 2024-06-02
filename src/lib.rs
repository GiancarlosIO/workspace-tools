#![deny(clippy::all)]

use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

mod cli;

use crate::cli::Cli;

#[napi]
pub async fn run_cli(args: Vec<String>) -> Result<()> {
  let cli = Cli::new(args)?;

  cli.run();

  Ok(())
}

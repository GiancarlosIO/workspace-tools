#![deny(clippy::all)]

use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

mod cli;

use crate::cli::{create_cli, run};

#[napi]
pub async fn run_cli(args: Vec<String>) -> Result<()> {
  create_cli(args).and_then(run)
}

#!/usr/bin/env node

import { runCli } from "./index.js";
const args = process.argv.slice(2);

runCli(args).catch((e) => {
  console.error("Ups! there was an error when trying to run the CLI.");
  console.error(e);
  process.exit(1);
});

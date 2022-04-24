#![allow(unused)]

use std::{thread, time};
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  /// The path to the file to read
  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> Result<()>  {
  let bar = ProgressBar::new(100);
  for _ in 0..100 {
    thread::sleep(time::Duration::from_millis(10));
    bar.inc(1);
  }
  bar.finish();

  let args = Cli::parse();
  let path = &args.path;
  let content = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file: `{}`", path.display()))?;

  println!("pattern: {}", args.pattern);
  println!("{}", content);
  Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("rst")?;

  cmd.arg("foobar").arg("test/file/doesnt/exist");
  cmd.assert()
    .failure()
    .stderr(predicate::str::contains("could not read file"));

  Ok(())
}

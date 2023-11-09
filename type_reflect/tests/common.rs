pub const OUTPUT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/output");

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use std::process::Command;
pub struct OutputLocation {
    path: PathBuf,
}

fn run_command(command: &str) -> Result<()> {

    println!("Running command:\n\t{}", command);

    let mut parts = command.split_whitespace();
    let command = parts.next().expect("no command given");
    let args = parts.collect::<Vec<&str>>();

    let mut child = Command::new(command).args(&args).spawn()?; // Spawn the command as a child process

    let status = child.wait()?; // Wait for the command to complete

    if status.success() {
    } else {
        bail!("Command failed: {}", command)
    }
    Ok(())
}

impl OutputLocation {
    pub fn ts_path(&self) -> PathBuf {
        self.path.with_extension("ts")
    }
    fn js_path(&self) -> PathBuf {
        self.path.with_extension("js")
    }
    fn clean(&self) {
        remove_file(self.ts_path());
        remove_file(self.js_path());
    }

    pub fn run_ts(&self) -> Result<()> {
        run_command(format!("tsc {}", self.ts_path().to_str().unwrap()).as_str())?;
        run_command(format!("node {}", self.js_path().to_str().unwrap()).as_str())?;
        Ok(())
    }
}

fn remove_file(path: PathBuf) {
    match fs::remove_file(&path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error removing file: [{:?}]: {}", &path, e),
    }
}

pub fn init_path(name: &str) -> OutputLocation {
    let mut base_path: PathBuf = PathBuf::from(OUTPUT_DIR);
    base_path.push(name);
    let output = OutputLocation { path: base_path };
    output.clean();
    output
}

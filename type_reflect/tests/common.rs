pub const OUTPUT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/output");

pub const TESTING_PREFIX: &'static str = r#"

function assertThrows(fn: ()=>void, message: string) {
  try {
    fn();
  } catch (e) {
    console.log(`error thrown: ${e}`);
    return;
  }
  throw new Error(message);
}

function assertDoesNotThrow<T>(fn: ()=>T, message: string) {
  try {
    return fn();
  } catch (e) {
    console.error(message);
    throw e;
  }
}

"#;

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use std::process::Command;
pub struct OutputLocation {
    path: PathBuf,
    filename: String,
}

fn run_command(dir: &str, command: &str) -> Result<()> {
    println!("Running command:\n\t{}", command);

    let mut parts = command.split_whitespace();
    let command = parts.next().expect("no command given");
    let args = parts.collect::<Vec<&str>>();

    let mut child = Command::new(command).args(&args).current_dir(dir).spawn()?; // Spawn the command as a child process

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
    fn jest_path(&self) -> PathBuf {
        self.path.with_extension("test.ts")
    }
    fn clean(&self) {
        remove_file(self.ts_path());
        remove_file(self.jest_path());
    }

    pub fn run_ts(&self) -> Result<()> {
        println!("");
        run_command(
            OUTPUT_DIR,
            format!("yarn jest {}", self.jest_path().to_str().unwrap()).as_str(),
        )?;
        Ok(())
    }

    pub fn write_jest(&self, imports: &str, content: &str) -> Result<()> {
        fs::write(
            self.jest_path(),
            format!(
                "import {{ {imports} }} from './{file}'\n\n{content}",
                content = content,
                imports = imports,
                file = self.filename
            ),
        )?;
        Ok(())
    }
}

fn remove_file(path: PathBuf) {
    match fs::remove_file(&path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error removing file: [{:?}]: {}", &path, e),
    }
}

pub fn init_path(scope: &str, name: &str) -> OutputLocation {
    let mut base_path: PathBuf = PathBuf::from(OUTPUT_DIR);
    base_path.push("src");
    base_path.push(scope);
    if !base_path.exists() {
        match fs::create_dir(&base_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error creating directory [{:?}]: {}", &base_path, e);
            }
        }
    };

    base_path.push(name);
    let output = OutputLocation {
        path: base_path,
        filename: name.to_string(),
    };
    output.clean();
    output
}

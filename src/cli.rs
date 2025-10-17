use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::error::SatukitanError;
use crate::interpreter::Interpreter;
use crate::repl;

#[derive(Parser)]
#[command(
    name = "satukitan",
    version,
    about = "Satukitan scripting language interpreter"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run a Satukitan script file (.sample.st)
    Run { file: PathBuf },
    /// Start an interactive REPL session
    Repl,
}

pub fn run() -> Result<(), SatukitanError> {
    let cli = Cli::parse();
    let mut interpreter = Interpreter::new();

    match cli.command {
        Some(Command::Run { file }) => run_file(&mut interpreter, file),
        Some(Command::Repl) | None => repl::start(&mut interpreter),
    }
}

fn run_file(interpreter: &mut Interpreter, path: PathBuf) -> Result<(), SatukitanError> {
    validate_extension(path.as_path())?;
    let source = fs::read_to_string(&path)?;
    let value = interpreter.eval_str(&source)?;
    if !value.is_nil() {
        println!("{}", value);
    }
    Ok(())
}

fn validate_extension(path: &Path) -> Result<(), SatukitanError> {
    let is_valid = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.ends_with(".st"))
        .unwrap_or(false);

    if is_valid {
        Ok(())
    } else {
        Err(SatukitanError::Eval(format!(
            "expected a .st file, got {}",
            path.display()
        )))
    }
}

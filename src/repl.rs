use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::error::SatukitanError;
use crate::interpreter::Interpreter;
use crate::value::Value;

pub fn start(interpreter: &mut Interpreter) -> Result<(), SatukitanError> {
    let mut rl = DefaultEditor::new()
        .map_err(|err| SatukitanError::Eval(format!("repl init error: {err}")))?;

    loop {
        match rl.readline("satukitan> ") {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if matches!(trimmed, "exit" | "quit") {
                    break;
                }
                rl.add_history_entry(trimmed).ok();
                match interpreter.eval_str(trimmed) {
                    Ok(value) => display_value(value),
                    Err(err) => eprintln!("Error: {err}"),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Err(SatukitanError::Eval(format!("REPL failure: {err}"))),
        }
    }

    Ok(())
}

fn display_value(value: Value) {
    if !value.is_nil() {
        println!("{}", value);
    }
}

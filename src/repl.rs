use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use rustyline::{Editor, history::DefaultHistory};
use std::fmt::Write;

use crate::error::SatukitanError;
use crate::interpreter::Interpreter;
use crate::value::Value;

struct KeywordInfo {
    name: &'static str,
    annotation: Option<&'static str>,
}

const KEYWORD_INFOS: &[KeywordInfo] = &[
    KeywordInfo {
        name: "#ta",
        annotation: Some("(10)"),
    },
    KeywordInfo {
        name: "ditas",
        annotation: Some("(num num -> bool)"),
    },
    KeywordInfo {
        name: "ditasgata",
        annotation: Some("(num num -> bool)"),
    },
    KeywordInfo {
        name: "fanitas",
        annotation: Some("(list -> list)"),
    },
    KeywordInfo {
        name: "fityes",
        annotation: Some("(num num -> bool)"),
    },
    KeywordInfo {
        name: "fityesgata",
        annotation: Some("(num num -> bool)"),
    },
    KeywordInfo {
        name: "ga",
        annotation: Some("(false)"),
    },
    KeywordInfo {
        name: "gakas",
        annotation: Some("(symbol value -> value)"),
    },
    KeywordInfo {
        name: "gakasdenu",
        annotation: Some("(symbol (params) body -> function)"),
    },
    KeywordInfo {
        name: "gatas",
        annotation: Some("(value value -> bool)"),
    },
    KeywordInfo {
        name: "kenus",
        annotation: Some("(bool bool -> bool)"),
    },
    KeywordInfo {
        name: "matyes",
        annotation: Some("(num num -> num)"),
    },
    KeywordInfo {
        name: "me",
        annotation: Some("(true)"),
    },
    KeywordInfo {
        name: "nitas",
        annotation: Some("(num num -> num)"),
    },
    KeywordInfo {
        name: "nobu",
        annotation: Some("(bool then else -> value)"),
    },
    KeywordInfo {
        name: "ra",
        annotation: Some("(2)"),
    },
    KeywordInfo {
        name: "rakas",
        annotation: Some("(list -> num)"),
    },
    KeywordInfo {
        name: "re",
        annotation: Some("(4)"),
    },
    KeywordInfo {
        name: "ri",
        annotation: Some("(5)"),
    },
    KeywordInfo {
        name: "ritas",
        annotation: Some("(num num -> num)"),
    },
    KeywordInfo {
        name: "ro",
        annotation: Some("(3)"),
    },
    KeywordInfo {
        name: "ru",
        annotation: Some("(1)"),
    },
    KeywordInfo {
        name: "rv",
        annotation: Some("(0)"),
    },
    KeywordInfo {
        name: "rya",
        annotation: Some("(6)"),
    },
    KeywordInfo {
        name: "rye",
        annotation: Some("(9)"),
    },
    KeywordInfo {
        name: "ryo",
        annotation: Some("(8)"),
    },
    KeywordInfo {
        name: "ryu",
        annotation: Some("(7)"),
    },
    KeywordInfo {
        name: "sipus",
        annotation: Some("(value -> nil)"),
    },
    KeywordInfo {
        name: "teses",
        annotation: Some("(bool bool -> bool)"),
    },
];

const MAX_HINT_SUGGESTIONS: usize = 5;
const HEADER_COLOR: &str = "[38;5;39m";
const CANDIDATE_COLOR: &str = "[38;5;214m";
const DIVIDER_COLOR: &str = "[38;5;240m";

#[derive(Clone)]
struct SatukitanHelper {
    keywords: &'static [KeywordInfo],
}

impl Default for SatukitanHelper {
    fn default() -> Self {
        Self {
            keywords: KEYWORD_INFOS,
        }
    }
}

impl SatukitanHelper {
    fn fragment_start(&self, line: &str, pos: usize) -> usize {
        line[..pos]
            .char_indices()
            .rev()
            .find(|&(_, ch)| is_boundary(ch))
            .map(|(idx, ch)| idx + ch.len_utf8())
            .unwrap_or(0)
    }

    fn matching_keywords(&self, fragment: &str) -> Vec<&'static KeywordInfo> {
        self.keywords
            .iter()
            .filter(|info| info.name.starts_with(fragment))
            .collect::<Vec<_>>()
    }

    fn candidate_pairs(&self, fragment: &str) -> Vec<Pair> {
        self.matching_keywords(fragment)
            .into_iter()
            .map(|info| Pair {
                display: self.label_for(info),
                replacement: info.name.to_string(),
            })
            .collect()
    }

    fn label_for(&self, info: &KeywordInfo) -> String {
        if let Some(annotation) = info.annotation {
            format!("{}{}", info.name, annotation)
        } else {
            info.name.to_string()
        }
    }

    fn format_hint(&self, fragment: &str, matches: &[&'static KeywordInfo]) -> Option<String> {
        if matches.is_empty() {
            return None;
        }
        if matches.len() == 1 && matches[0].name == fragment {
            return None;
        }

        let mut buffer = String::new();
        buffer.push('\n');

        buffer.push_str(HEADER_COLOR);
        buffer.push_str("候補:");
        buffer.push_str("\x1b[0m ");

        for (idx, info) in matches.iter().take(MAX_HINT_SUGGESTIONS).enumerate() {
            if idx > 0 {
                buffer.push_str(DIVIDER_COLOR);
                buffer.push_str(" | ");
                buffer.push_str("\x1b[0m");
            }
            buffer.push_str(CANDIDATE_COLOR);
            buffer.push_str(&self.label_for(info));
            buffer.push_str("\x1b[0m");
        }

        if matches.len() > MAX_HINT_SUGGESTIONS {
            let remaining = matches.len() - MAX_HINT_SUGGESTIONS;
            let _ = write!(buffer, " {}… (+{}件)\x1b[0m", DIVIDER_COLOR, remaining);
        }

        Some(buffer)
    }
}

impl Completer for SatukitanHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let start = self.fragment_start(line, pos);
        let fragment = &line[start..pos];
        Ok((start, self.candidate_pairs(fragment)))
    }
}

impl Hinter for SatukitanHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if pos < line.len() {
            return None;
        }
        let start = self.fragment_start(line, pos);
        let fragment = &line[start..pos];
        if fragment.is_empty() {
            return None;
        }
        let matches = self.matching_keywords(fragment);
        self.format_hint(fragment, &matches)
    }
}

impl Highlighter for SatukitanHelper {}

impl Validator for SatukitanHelper {}

impl Helper for SatukitanHelper {}

pub fn start(interpreter: &mut Interpreter) -> Result<(), SatukitanError> {
    let mut rl = Editor::<SatukitanHelper, DefaultHistory>::new()
        .map_err(|err| SatukitanError::Eval(format!("repl init error: {err}")))?;
    rl.set_helper(Some(SatukitanHelper::default()));

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

fn is_boundary(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | '"')
}

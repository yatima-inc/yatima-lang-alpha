use std::io::{self, Write};
use rustyline::{
  error::ReadlineError,
  Cmd,
  Config,
  EditMode,
  Editor,
  KeyEvent,
};

use yatima_utils::repl::{
  run_repl,
  Repl,
  error::ReplError
};


struct RustyLineRepl {
  rl: Editor<()>
}

impl Repl for RustyLineRepl {
  fn readline(&mut self, prompt: &str) -> Result<String, ReplError> {
    self.rl.readline(prompt).map_err(|e| match e {
      ReadlineError::Interrupted => ReplError::Interrupted,
      ReadlineError::Eof => ReplError::Eof,
      _ => ReplError::Other(e.to_string())
    })
  }

  fn println(&self, s: String) {
    let mut out = io::stdout();
    out.write(s.as_bytes()).unwrap();
    out.write("\n".as_bytes()).unwrap();
  }

  fn load_history(&mut self) {
    if self.rl.load_history("history.txt").is_err() {
      println!("No previous history.");
    }
  }

  fn add_history_entry(&mut self, s: &str) {
    self.rl.add_history_entry(s);
  }

  fn save_history(&mut self) {
    self.rl.save_history("history.txt").unwrap();
  }
}

pub fn main() {
  let config = Config::builder().edit_mode(EditMode::Vi).build();
  let mut rl = Editor::<()>::with_config(config);
  rl.bind_sequence(KeyEvent::alt('l'), Cmd::Insert(1, String::from("λ ")));
  rl.bind_sequence(KeyEvent::alt('a'), Cmd::Insert(1, String::from("∀ ")));
  run_repl(&mut RustyLineRepl { rl : rl });
}
use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};

pub struct ShellEditor {
    inner: DefaultEditor,
    history_path: String,
}

impl ShellEditor {
    pub fn new(history_path: String) -> Result<Self> {
        let mut inner = DefaultEditor::new()?;
        let _ = inner.load_history(&history_path);
        Ok(Self {
            inner,
            history_path,
        })
    }

    pub fn read_line(&mut self, prompt: &str) -> Result<Option<String>> {
        match self.inner.readline(prompt) {
            Ok(line) => Ok(Some(line)),
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub fn add_history(&mut self, line: &str) {
        if !line.trim().is_empty() {
            let _ = self.inner.add_history_entry(line);
        }
    }

    pub fn save(&mut self) -> Result<()> {
        self.inner.save_history(&self.history_path)?;
        Ok(())
    }
}

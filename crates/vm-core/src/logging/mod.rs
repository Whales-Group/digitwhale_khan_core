#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub level: LogLevel,
    pub component: &'static str,
    pub message: String,
}

pub trait Logger {
    fn log(&mut self, event: LogEvent);
}

pub struct StdoutLogger {
    min_level: LogLevel,
}

impl StdoutLogger {
    pub fn new(min_level: LogLevel) -> Self {
        Self { min_level }
    }
}

impl Default for StdoutLogger {
    fn default() -> Self {
        Self::new(LogLevel::Info)
    }
}

impl Logger for StdoutLogger {
    fn log(&mut self, event: LogEvent) {
        if event.level >= self.min_level {
            println!("[{:#?}] {}: {}", event.level, event.component, event.message);
        }
    }
}

#[derive(Default)]
pub struct VecLogger {
    events: Vec<LogEvent>,
}

impl VecLogger {
    pub fn events(&self) -> &[LogEvent] {
        &self.events
    }
}

impl Logger for VecLogger {
    fn log(&mut self, event: LogEvent) {
        self.events.push(event);
    }
}

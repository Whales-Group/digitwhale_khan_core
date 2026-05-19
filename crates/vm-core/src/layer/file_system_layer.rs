use std::path::{Component, Path, PathBuf};

use crate::{Memory, SYSCALL_CAT, SYSCALL_LS};

use super::{Layer, LayerError, LayerEvent};

pub struct FileSystemLayer {
    root: PathBuf,
}

impl FileSystemLayer {
    pub fn from_dotenv(path: &str) -> Self {
        let root = std::fs::read_to_string(path)
            .ok()
            .and_then(|raw| parse_env_value(&raw, "JEVAN_ROOT"))
            .unwrap_or_else(|| ".usr".to_string());
        Self { root: PathBuf::from(root) }
    }

    fn validate(&self, syscall_id: u16, args: &[String]) -> Result<(), LayerError> {
        if syscall_id != SYSCALL_LS && syscall_id != SYSCALL_CAT {
            return Ok(());
        }

        let target = args.first().map(String::as_str).unwrap_or(".");
        let joined = self.root.join(target);
        if !is_relative_safe(Path::new(target)) || !path_is_under(&self.root, &joined) {
            return Err(blocked("path traversal or absolute path is not allowed"));
        }
        if syscall_id == SYSCALL_CAT && target == "." {
            return Err(blocked("cat expects a file path"));
        }
        Ok(())
    }
}

impl Layer for FileSystemLayer {
    fn name(&self) -> &'static str {
        "fs"
    }

    fn on_event(&mut self, event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        if let LayerEvent::BeforeSyscall { syscall_id, args } = event {
            self.validate(*syscall_id, args)?;
        }
        Ok(())
    }
}

fn blocked(reason: &str) -> LayerError {
    LayerError::Blocked {
        layer: "fs",
        reason: reason.to_string(),
    }
}

fn parse_env_value(raw: &str, key: &str) -> Option<String> {
    raw.lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix(&format!("{key}=")).map(str::to_string))
}

fn is_relative_safe(path: &Path) -> bool {
    !path.is_absolute()
        && path
            .components()
            .all(|c| !matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
}

fn path_is_under(root: &Path, joined: &Path) -> bool {
    let root_components: Vec<_> = root.components().collect();
    let joined_components: Vec<_> = joined.components().collect();
    joined_components.starts_with(&root_components)
}

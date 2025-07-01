use anyhow::Result;
use crate::Database;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::Path;
use std::sync::mpsc;
use std::time::SystemTime;
use std::fs;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    db: Database,
}

impl FileWatcher {
    pub fn new(db: Database) -> Result<(Self, mpsc::Receiver<notify::Result<Event>>)> {
        let (tx, rx) = mpsc::channel();
        
        let watcher = notify::recommended_watcher(tx)?;
        
        Ok((FileWatcher { watcher, db }, rx))
    }

    pub fn watch(&mut self, path: &Path) -> Result<()> {
        println!("ファイル監視開始: {}", path.display());
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    pub fn handle_event(&self, event: Event) -> Result<()> {
        if let Some(file_event) = parse_file_event(event) {
            match file_event {
                FileEvent::Created(paths) => {
                    for path in paths {
                        if path.is_file() {
                            self.add_file_to_index(&path)?;
                            println!("ファイル追加: {}", path.display());
                        }
                    }
                }
                FileEvent::Deleted(paths) => {
                    for path in paths {
                        self.remove_file_from_index(&path)?;
                        println!("ファイル削除: {}", path.display());
                    }
                }
                FileEvent::Modified(paths) => {
                    for path in paths {
                        if path.is_file() {
                            self.update_file_in_index(&path)?;
                            println!("ファイル更新: {}", path.display());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn add_file_to_index(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let modified_time = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            self.db.insert_file(&path_str, &name, size, modified_time)?;
        }
        Ok(())
    }

    fn remove_file_from_index(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();
        self.db.remove_file(&path_str)?;
        Ok(())
    }

    fn update_file_in_index(&self, path: &Path) -> Result<()> {
        self.add_file_to_index(path)
    }
}

pub fn parse_file_event(event: Event) -> Option<FileEvent> {
    match event.kind {
        EventKind::Create(_) => Some(FileEvent::Created(event.paths)),
        EventKind::Remove(_) => Some(FileEvent::Deleted(event.paths)), 
        EventKind::Modify(_) => Some(FileEvent::Modified(event.paths)),
        _ => None,
    }
}

#[derive(Debug)]
pub enum FileEvent {
    Created(Vec<std::path::PathBuf>),
    Modified(Vec<std::path::PathBuf>),
    Deleted(Vec<std::path::PathBuf>),
}
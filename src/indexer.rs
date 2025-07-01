use anyhow::Result;
use crate::Database;
use std::fs;
use std::time::SystemTime;
use walkdir::WalkDir;

pub struct Indexer {
    db: Database,
}

impl Indexer {
    pub fn new(db: Database) -> Self {
        Indexer { db }
    }

    pub fn build_index(&self, root_path: &str) -> Result<()> {
        println!("インデックスをクリア中...");
        self.db.clear_all()?;

        println!("ファイルをスキャン中: {}", root_path);
        let mut count = 0;

        for entry in WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let metadata = match fs::metadata(path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = metadata.len();
            let modified_time = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if let Err(e) = self.db.insert_file(&path_str, &name, size, modified_time) {
                eprintln!("ファイル追加エラー {}: {}", path_str, e);
                continue;
            }

            count += 1;
            if count % 1000 == 0 {
                println!("処理済み: {} ファイル", count);
            }
        }

        println!("インデックス作成完了: {} ファイル", count);
        Ok(())
    }
}
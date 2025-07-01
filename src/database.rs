use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                size INTEGER,
                modified_time INTEGER,
                created_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_files_name ON files(name)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_files_path ON files(path)",
            [],
        )?;

        Ok(())
    }

    pub fn insert_file(&self, path: &str, name: &str, size: u64, modified_time: u64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO files (path, name, size, modified_time) VALUES (?1, ?2, ?3, ?4)",
            params![path, name, size as i64, modified_time as i64],
        )?;
        Ok(())
    }

    pub fn search_files(&self, query: &str, case_sensitive: bool) -> Result<Vec<String>> {
        let sql = if case_sensitive {
            "SELECT path FROM files WHERE name LIKE ?1 OR path LIKE ?1 ORDER BY name"
        } else {
            "SELECT path FROM files WHERE name LIKE ?1 COLLATE NOCASE OR path LIKE ?1 COLLATE NOCASE ORDER BY name"
        };

        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params![search_pattern], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_stats(&self) -> Result<(i64, i64)> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*), SUM(size) FROM files")?;
        let row = stmt.query_row([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, Option<i64>>(1)?.unwrap_or(0)))
        })?;
        Ok(row)
    }

    pub fn remove_file(&self, path: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM files WHERE path = ?1",
            params![path],
        )?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.conn.execute("DELETE FROM files", [])?;
        Ok(())
    }
}
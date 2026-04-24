use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub status: String,
    pub codec: String,
    pub output_path: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresetRow {
    pub id: String,
    pub name: String,
    pub spec: String,
    pub flags: Vec<String>,
    pub hotkey: Option<String>,
    pub is_default: bool,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> rusqlite::Result<()> {
        let sql = include_str!("../migrations/001_init.sql");
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(sql)?;
        Ok(())
    }

    pub fn list_history(&self, limit: u32, offset: u32) -> rusqlite::Result<Vec<HistoryItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, url, title, status, codec, output_path, started_at, finished_at, error
             FROM downloads
             ORDER BY COALESCE(finished_at, started_at) DESC
             LIMIT ?1 OFFSET ?2",
        )?;
        let rows = stmt.query_map(params![limit, offset], |r| {
            Ok(HistoryItem {
                id: r.get(0)?,
                url: r.get(1)?,
                title: r.get(2)?,
                status: r.get(3)?,
                codec: r.get(4)?,
                output_path: r.get(5)?,
                started_at: r.get(6)?,
                finished_at: r.get(7)?,
                error: r.get(8)?,
            })
        })?;
        rows.collect()
    }

    pub fn find_completed_by_url(&self, url: &str) -> rusqlite::Result<Option<HistoryItem>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, url, title, status, codec, output_path, started_at, finished_at, error
             FROM downloads
             WHERE url = ?1 AND status = 'done'
             ORDER BY finished_at DESC LIMIT 1",
            params![url],
            |r| {
                Ok(HistoryItem {
                    id: r.get(0)?,
                    url: r.get(1)?,
                    title: r.get(2)?,
                    status: r.get(3)?,
                    codec: r.get(4)?,
                    output_path: r.get(5)?,
                    started_at: r.get(6)?,
                    finished_at: r.get(7)?,
                    error: r.get(8)?,
                })
            },
        )
        .optional()
    }

    pub fn insert_download(
        &self,
        id: &str,
        url: &str,
        title: &str,
        codec: &str,
        started_at: i64,
    ) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO downloads (id, url, title, status, codec, started_at)
             VALUES (?1, ?2, ?3, 'active', ?4, ?5)",
            params![id, url, title, codec, started_at],
        )?;
        Ok(())
    }

    pub fn update_done(&self, id: &str, path: &str, finished_at: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE downloads SET status='done', output_path=?2, finished_at=?3 WHERE id=?1",
            params![id, path, finished_at],
        )?;
        Ok(())
    }

    pub fn update_error(&self, id: &str, message: &str, finished_at: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE downloads SET status='error', error=?2, finished_at=?3 WHERE id=?1",
            params![id, message, finished_at],
        )?;
        Ok(())
    }

    pub fn update_cancelled(&self, id: &str, finished_at: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE downloads SET status='cancelled', finished_at=?2 WHERE id=?1",
            params![id, finished_at],
        )?;
        Ok(())
    }

    pub fn list_presets(&self) -> rusqlite::Result<Vec<PresetRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, format_spec, flags, hotkey, is_default FROM presets ORDER BY is_default DESC, name ASC",
        )?;
        let rows = stmt.query_map([], |r| {
            let flags_json: String = r.get(3)?;
            let flags: Vec<String> = serde_json::from_str(&flags_json).unwrap_or_default();
            Ok(PresetRow {
                id: r.get(0)?,
                name: r.get(1)?,
                spec: r.get(2)?,
                flags,
                hotkey: r.get(4)?,
                is_default: r.get::<_, i64>(5)? != 0,
            })
        })?;
        rows.collect()
    }

    pub fn upsert_preset(&self, p: &PresetRow) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        let flags_json = serde_json::to_string(&p.flags).unwrap_or_else(|_| "[]".into());
        conn.execute(
            "INSERT INTO presets (id, name, format_spec, flags, hotkey, is_default)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                format_spec=excluded.format_spec,
                flags=excluded.flags,
                hotkey=excluded.hotkey,
                is_default=excluded.is_default",
            params![
                p.id,
                p.name,
                p.spec,
                flags_json,
                p.hotkey,
                p.is_default as i64
            ],
        )?;
        Ok(())
    }

    pub fn delete_preset(&self, id: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM presets WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> rusqlite::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |r| r.get::<_, String>(0),
        )
        .optional()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Database {
        Database::open(":memory:").expect("in-memory db")
    }

    #[test]
    fn migration_seeds_default_presets() {
        let db = fresh();
        let presets = db.list_presets().unwrap();
        assert!(presets
            .iter()
            .any(|p| p.id == "archive-av1" && p.is_default));
        assert!(presets
            .iter()
            .any(|p| p.hotkey.as_deref() == Some("\u{2318}1")));
    }

    #[test]
    fn history_lifecycle() {
        let db = fresh();
        db.insert_download("id1", "https://example.com/v", "Test", "av1", 1000)
            .unwrap();
        let hist = db.list_history(10, 0).unwrap();
        assert_eq!(hist.len(), 1);
        assert_eq!(hist[0].status, "active");

        db.update_done("id1", "/tmp/test.mkv", 2000).unwrap();
        let done = db.find_completed_by_url("https://example.com/v").unwrap();
        assert!(done.is_some());
        let row = done.unwrap();
        assert_eq!(row.status, "done");
        assert_eq!(row.output_path.as_deref(), Some("/tmp/test.mkv"));
    }

    #[test]
    fn error_and_cancel_statuses() {
        let db = fresh();
        db.insert_download("x", "u", "t", "c", 1).unwrap();
        db.update_error("x", "boom", 2).unwrap();
        let h = db.list_history(10, 0).unwrap();
        assert_eq!(h[0].status, "error");

        db.insert_download("y", "v", "t", "c", 1).unwrap();
        db.update_cancelled("y", 2).unwrap();
        let h = db.list_history(10, 0).unwrap();
        let cancelled = h.iter().find(|r| r.id == "y").unwrap();
        assert_eq!(cancelled.status, "cancelled");
    }

    #[test]
    fn preset_crud() {
        let db = fresh();
        let p = PresetRow {
            id: "custom".into(),
            name: "custom".into(),
            spec: "ba/b".into(),
            flags: vec!["--embed-metadata".into()],
            hotkey: None,
            is_default: false,
        };
        db.upsert_preset(&p).unwrap();
        assert!(db.list_presets().unwrap().iter().any(|x| x.id == "custom"));
        db.delete_preset("custom").unwrap();
        assert!(!db.list_presets().unwrap().iter().any(|x| x.id == "custom"));
    }

    #[test]
    fn preset_flags_roundtrip_as_json() {
        let db = fresh();
        let p = PresetRow {
            id: "flg".into(),
            name: "n".into(),
            spec: "s".into(),
            flags: vec!["--a".into(), "--b".into(), "val".into()],
            hotkey: Some("\u{2318}9".into()),
            is_default: false,
        };
        db.upsert_preset(&p).unwrap();
        let got = db.list_presets().unwrap();
        let fetched = got.iter().find(|x| x.id == "flg").unwrap();
        assert_eq!(fetched.flags, p.flags);
        assert_eq!(fetched.hotkey, p.hotkey);
    }

    #[test]
    fn settings_upsert() {
        let db = fresh();
        db.set_setting("k", "v1").unwrap();
        assert_eq!(db.get_setting("k").unwrap(), Some("v1".into()));
        db.set_setting("k", "v2").unwrap();
        assert_eq!(db.get_setting("k").unwrap(), Some("v2".into()));
        assert_eq!(db.get_setting("missing").unwrap(), None);
    }
}

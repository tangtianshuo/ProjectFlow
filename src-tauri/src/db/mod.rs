use chrono::Utc;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use uuid::Uuid;

// 辅助函数：解析日期字符串，支持 YYYY-MM-DD 和 RFC3339 格式
fn parse_date(date_str: &str) -> Option<chrono::DateTime<Utc>> {
    // 尝试 RFC3339 格式
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.with_timezone(&Utc));
    }
    // 尝试 YYYY-MM-DD 格式，保持日期不变（使用 UTC 避免时区转换导致日期变化）
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // 使用 UTC 00:00:00 创建日期时间，避免时区转换导致日期变化
        return Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            date.and_hms_opt(0, 0, 0)?,
            chrono::Utc,
        ));
    }
    None
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_data_dir: &std::path::Path) -> Result<Self> {
        std::fs::create_dir_all(app_data_dir).ok();
        let db_path = app_data_dir.join("projectflow.db");
        let conn = Connection::open(&db_path)?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                status INTEGER NOT NULL DEFAULT 0,
                start_date TEXT,
                end_date TEXT,
                owner_id TEXT,
                settings TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                parent_id TEXT,
                title TEXT NOT NULL,
                description TEXT,
                status INTEGER NOT NULL DEFAULT 0,
                priority INTEGER DEFAULT 1,
                assignee_id TEXT,
                start_date TEXT,
                due_date TEXT,
                estimated_hours REAL,
                actual_hours REAL DEFAULT 0,
                progress REAL DEFAULT 0,
                position INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                project_id TEXT,
                title TEXT NOT NULL,
                content TEXT,
                file_path TEXT,
                current_version INTEGER DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
            )",
            [],
        )?;

        // Add deleted_at column if not exists (migration for existing databases)
        conn.execute("ALTER TABLE projects ADD COLUMN deleted_at TEXT", [])
            .ok();
        conn.execute("ALTER TABLE tasks ADD COLUMN deleted_at TEXT", [])
            .ok();
        conn.execute("ALTER TABLE documents ADD COLUMN deleted_at TEXT", [])
            .ok();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT,
                avatar TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS task_dependencies (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                depends_on_task_id TEXT NOT NULL,
                dependency_type INTEGER DEFAULT 0,
                lag_days INTEGER DEFAULT 0,
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
                FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id) ON DELETE CASCADE,
                UNIQUE(task_id, depends_on_task_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS milestones (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                target_date TEXT,
                status INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Add deleted_at column if not exists for migration
        conn.execute("ALTER TABLE milestones ADD COLUMN deleted_at TEXT", [])
            .ok();

        // Create indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks(parent_id)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_documents_project_id ON documents(project_id)",
            [],
        )?;

        // LLM Settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS llm_settings (
                id INTEGER PRIMARY KEY,
                provider TEXT NOT NULL DEFAULT 'openai',
                api_key TEXT,
                api_url TEXT NOT NULL DEFAULT 'https://api.openai.com/v1',
                model TEXT NOT NULL DEFAULT 'gpt-4o-mini',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Insert default config if not exists
        let count: i32 =
            conn.query_row("SELECT COUNT(*) FROM llm_settings", [], |row| row.get(0))?;
        if count == 0 {
            let now = Utc::now().to_rfc3339();
            conn.execute(
                "INSERT INTO llm_settings (id, provider, api_key, api_url, model, created_at, updated_at)
                 VALUES (1, 'openai', '', 'https://api.openai.com/v1', 'gpt-4o-mini', ?1, ?2)",
                params![now, now],
            )?;
        }

        Ok(())
    }

    // Project operations
    pub fn create_project(
        &self,
        name: &str,
        description: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<crate::models::Project> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO projects (id, name, description, status, start_date, end_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6, ?7)",
            params![id, name, description, start_date, end_date, now, now],
        )?;

        Ok(crate::models::Project {
            id,
            name: name.to_string(),
            description: description.map(String::from),
            status: 0,
            start_date: start_date.and_then(parse_date),
            end_date: end_date.and_then(parse_date),
            owner_id: None,
            settings: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub fn get_all_projects(&self) -> Result<Vec<crate::models::Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, status, start_date, end_date, owner_id, settings, created_at, updated_at, deleted_at FROM projects WHERE deleted_at IS NULL ORDER BY created_at DESC")?;

        let projects = stmt
            .query_map([], |row| {
                Ok(crate::models::Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    start_date: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| parse_date(&s)),
                    end_date: row
                        .get::<_, Option<String>>(5)?
                        .and_then(|s| parse_date(&s)),
                    owner_id: row.get(6)?,
                    settings: row.get(7)?,
                    created_at: row
                        .get::<_, String>(8)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(9)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(10)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(projects)
    }

    pub fn get_project(&self, id: &str) -> Result<Option<crate::models::Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, status, start_date, end_date, owner_id, settings, created_at, updated_at, deleted_at FROM projects WHERE id = ?1 AND deleted_at IS NULL")?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(crate::models::Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                start_date: row
                    .get::<_, Option<String>>(4)?
                    .and_then(|s| parse_date(&s)),
                end_date: row
                    .get::<_, Option<String>>(5)?
                    .and_then(|s| parse_date(&s)),
                owner_id: row.get(6)?,
                settings: row.get(7)?,
                created_at: row
                    .get::<_, String>(8)
                    .map(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .unwrap()
                            .with_timezone(&Utc)
                    })
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: row
                    .get::<_, String>(9)
                    .map(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .unwrap()
                            .with_timezone(&Utc)
                    })
                    .unwrap_or_else(|_| Utc::now()),
                deleted_at: row.get::<_, Option<String>>(10)?.and_then(|s| {
                    chrono::DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|d| d.with_timezone(&Utc))
                }),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_project(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE projects SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn restore_project(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE projects SET deleted_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn get_deleted_projects(&self) -> Result<Vec<crate::models::Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, status, start_date, end_date, owner_id, settings, created_at, updated_at, deleted_at FROM projects WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC")?;

        let projects = stmt
            .query_map([], |row| {
                Ok(crate::models::Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    start_date: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| parse_date(&s)),
                    end_date: row
                        .get::<_, Option<String>>(5)?
                        .and_then(|s| parse_date(&s)),
                    owner_id: row.get(6)?,
                    settings: row.get(7)?,
                    created_at: row
                        .get::<_, String>(8)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(9)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(10)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(projects)
    }

    pub fn update_project(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        status: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        // Build dynamic query with only the fields that are provided
        let mut updates = vec!["updated_at = ?1".to_string()];
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];

        if let Some(v) = name {
            updates.push(format!("name = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = description {
            updates.push(format!("description = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = status {
            updates.push(format!("status = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(v));
        }
        if let Some(v) = start_date {
            updates.push(format!("start_date = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = end_date {
            updates.push(format!("end_date = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(v.to_string()));
        }

        let sql = format!(
            "UPDATE projects SET {} WHERE id = ?{}",
            updates.join(", "),
            params_vec.len() + 1
        );
        params_vec.push(Box::new(id.to_string()));

        let mut stmt = conn.prepare(&sql)?;
        let params_ref: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        stmt.execute(params_ref.as_slice())?;

        Ok(())
    }

    // Task operations
    pub fn create_task(
        &self,
        project_id: &str,
        parent_id: Option<&str>,
        title: &str,
        description: Option<&str>,
        assignee_id: Option<&str>,
        start_date: Option<&str>,
        due_date: Option<&str>,
        estimated_hours: Option<f64>,
    ) -> Result<crate::models::Task> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tasks (id, project_id, parent_id, title, description, status, priority, assignee_id, start_date, due_date, estimated_hours, actual_hours, progress, position, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, 1, ?6, ?7, ?8, ?9, 0, 0, 0, ?10, ?11)",
            params![id, project_id, parent_id, title, description, assignee_id, start_date, due_date, estimated_hours, now, now],
        )?;

        Ok(crate::models::Task {
            id,
            project_id: project_id.to_string(),
            parent_id: parent_id.map(String::from),
            title: title.to_string(),
            description: description.map(String::from),
            status: 0,
            priority: 1,
            assignee_id: assignee_id.map(String::from),
            start_date: start_date.and_then(parse_date),
            due_date: due_date.and_then(parse_date),
            estimated_hours,
            actual_hours: 0.0,
            progress: 0.0,
            position: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub fn get_tasks_by_project(&self, project_id: &str) -> Result<Vec<crate::models::Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, parent_id, title, description, status, priority, assignee_id, start_date, due_date, estimated_hours, actual_hours, progress, position, created_at, updated_at, deleted_at
             FROM tasks WHERE project_id = ?1 AND deleted_at IS NULL ORDER BY position, created_at"
        )?;

        let tasks = stmt
            .query_map(params![project_id], |row| {
                Ok(crate::models::Task {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                    status: row.get(5)?,
                    priority: row.get(6)?,
                    assignee_id: row.get(7)?,
                    start_date: row
                        .get::<_, Option<String>>(8)?
                        .and_then(|s| parse_date(&s)),
                    due_date: row
                        .get::<_, Option<String>>(9)?
                        .and_then(|s| parse_date(&s)),
                    estimated_hours: row.get(10)?,
                    actual_hours: row.get(11)?,
                    progress: row.get(12)?,
                    position: row.get(13)?,
                    created_at: row
                        .get::<_, String>(14)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(15)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(16)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }

    pub fn update_task(
        &self,
        id: &str,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<i32>,
        priority: Option<i32>,
        assignee_id: Option<&str>,
        start_date: Option<&str>,
        due_date: Option<&str>,
        estimated_hours: Option<f64>,
        actual_hours: Option<f64>,
        progress: Option<f64>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        // Build dynamic query with only provided fields
        let mut updates = vec!["updated_at = ?".to_string()];
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];

        if let Some(v) = title {
            updates.push("title = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = description {
            updates.push("description = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = status {
            updates.push("status = ?".to_string());
            params_vec.push(Box::new(v));
        }
        if let Some(v) = priority {
            updates.push("priority = ?".to_string());
            params_vec.push(Box::new(v));
        }
        if let Some(v) = assignee_id {
            updates.push("assignee_id = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = start_date {
            updates.push("start_date = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = due_date {
            updates.push("due_date = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = estimated_hours {
            updates.push("estimated_hours = ?".to_string());
            params_vec.push(Box::new(v));
        }
        if let Some(v) = actual_hours {
            updates.push("actual_hours = ?".to_string());
            params_vec.push(Box::new(v));
        }
        if let Some(v) = progress {
            updates.push("progress = ?".to_string());
            params_vec.push(Box::new(v));
        }

        // Always update id at the end
        params_vec.push(Box::new(id.to_string()));

        let sql = format!("UPDATE tasks SET {} WHERE id = ?", updates.join(", "));
        let mut stmt = conn.prepare(&sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        stmt.execute(params_refs.as_slice())?;

        Ok(())
    }

    pub fn delete_task(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE tasks SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn restore_task(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE tasks SET deleted_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn get_deleted_tasks(&self) -> Result<Vec<crate::models::Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, parent_id, title, description, status, priority, assignee_id, start_date, due_date, estimated_hours, actual_hours, progress, position, created_at, updated_at, deleted_at
             FROM tasks WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC"
        )?;

        let tasks = stmt
            .query_map([], |row| {
                Ok(crate::models::Task {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                    status: row.get(5)?,
                    priority: row.get(6)?,
                    assignee_id: row.get(7)?,
                    start_date: row
                        .get::<_, Option<String>>(8)?
                        .and_then(|s| parse_date(&s)),
                    due_date: row
                        .get::<_, Option<String>>(9)?
                        .and_then(|s| parse_date(&s)),
                    estimated_hours: row.get(10)?,
                    actual_hours: row.get(11)?,
                    progress: row.get(12)?,
                    position: row.get(13)?,
                    created_at: row
                        .get::<_, String>(14)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(15)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(16)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }

    // Document operations
    pub fn create_document(
        &self,
        project_id: Option<&str>,
        title: &str,
        content: Option<&str>,
    ) -> Result<crate::models::Document> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO documents (id, project_id, title, content, current_version, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6)",
            params![id, project_id, title, content, now, now],
        )?;

        Ok(crate::models::Document {
            id,
            project_id: project_id.map(String::from),
            title: title.to_string(),
            content: content.map(String::from),
            file_path: None,
            current_version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub fn get_documents_by_project(
        &self,
        project_id: &str,
    ) -> Result<Vec<crate::models::Document>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, project_id, title, content, file_path, current_version, created_at, updated_at, deleted_at FROM documents WHERE project_id = ?1 AND deleted_at IS NULL ORDER BY title")?;

        let docs = stmt
            .query_map(params![project_id], |row| {
                Ok(crate::models::Document {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    file_path: row.get(4)?,
                    current_version: row.get(5)?,
                    created_at: row
                        .get::<_, String>(6)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(7)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(8)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(docs)
    }

    pub fn get_all_documents(&self) -> Result<Vec<crate::models::Document>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, project_id, title, content, file_path, current_version, created_at, updated_at, deleted_at FROM documents WHERE deleted_at IS NULL ORDER BY title")?;

        let docs = stmt
            .query_map([], |row| {
                Ok(crate::models::Document {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    file_path: row.get(4)?,
                    current_version: row.get(5)?,
                    created_at: row
                        .get::<_, String>(6)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(7)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(8)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(docs)
    }

    pub fn update_document(
        &self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE documents SET title = COALESCE(?1, title), content = COALESCE(?2, content), updated_at = ?3 WHERE id = ?4 AND deleted_at IS NULL",
            params![title, content, now, id],
        )?;

        Ok(())
    }

    pub fn delete_document(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE documents SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn restore_document(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE documents SET deleted_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn get_deleted_documents(&self) -> Result<Vec<crate::models::Document>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, project_id, title, content, file_path, current_version, created_at, updated_at, deleted_at FROM documents WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC")?;

        let docs = stmt
            .query_map([], |row| {
                Ok(crate::models::Document {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    file_path: row.get(4)?,
                    current_version: row.get(5)?,
                    created_at: row
                        .get::<_, String>(6)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(7)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(8)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(docs)
    }

    // Milestone operations
    pub fn create_milestone(
        &self,
        project_id: &str,
        title: &str,
        description: Option<&str>,
        target_date: Option<&str>,
    ) -> Result<crate::models::Milestone> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO milestones (id, project_id, title, description, target_date, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7)",
            params![id, project_id, title, description, target_date, now, now],
        )?;

        Ok(crate::models::Milestone {
            id,
            project_id: project_id.to_string(),
            title: title.to_string(),
            description: description.map(String::from),
            target_date: target_date.and_then(parse_date),
            status: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub fn get_milestones_by_project(
        &self,
        project_id: &str,
    ) -> Result<Vec<crate::models::Milestone>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, description, target_date, status, created_at, updated_at, deleted_at
             FROM milestones WHERE project_id = ?1 AND deleted_at IS NULL ORDER BY target_date"
        )?;

        let milestones = stmt
            .query_map(params![project_id], |row| {
                Ok(crate::models::Milestone {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    target_date: row
                        .get::<_, Option<String>>(4)?
                        .and_then(|s| parse_date(&s)),
                    status: row.get(5)?,
                    created_at: row
                        .get::<_, String>(6)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(7)
                        .map(|s| {
                            chrono::DateTime::parse_from_rfc3339(&s)
                                .unwrap()
                                .with_timezone(&Utc)
                        })
                        .unwrap_or_else(|_| Utc::now()),
                    deleted_at: row.get::<_, Option<String>>(8)?.and_then(|s| {
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(milestones)
    }

    pub fn update_milestone(
        &self,
        id: &str,
        title: Option<&str>,
        description: Option<&str>,
        target_date: Option<&str>,
        status: Option<i32>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        let mut updates = vec!["updated_at = ?".to_string()];
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now)];

        if let Some(v) = title {
            updates.push("title = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = description {
            updates.push("description = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = target_date {
            updates.push("target_date = ?".to_string());
            params_vec.push(Box::new(v.to_string()));
        }
        if let Some(v) = status {
            updates.push("status = ?".to_string());
            params_vec.push(Box::new(v));
        }

        params_vec.push(Box::new(id.to_string()));

        let sql = format!("UPDATE milestones SET {} WHERE id = ?", updates.join(", "));
        let mut stmt = conn.prepare(&sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();
        stmt.execute(params_refs.as_slice())?;

        Ok(())
    }

    pub fn delete_milestone(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE milestones SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    // LLM Settings operations
    pub fn get_llm_settings(&self) -> Result<LlmSettings> {
        let conn = self.conn.lock().unwrap();
        let settings = conn.query_row(
            "SELECT id, provider, api_key, api_url, model, created_at, updated_at FROM llm_settings WHERE id = 1",
            [],
            |row| {
                Ok(LlmSettings {
                    id: row.get(0)?,
                    provider: row.get(1)?,
                    api_key: row.get(2)?,
                    api_url: row.get(3)?,
                    model: row.get(4)?,
                    created_at: row.get::<_, String>(5).map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: row.get::<_, String>(6).map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                })
            },
        )?;
        Ok(settings)
    }

    pub fn update_llm_settings(
        &self,
        provider: &str,
        api_key: &str,
        api_url: &str,
        model: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE llm_settings SET provider = ?1, api_key = ?2, api_url = ?3, model = ?4, updated_at = ?5 WHERE id = 1",
            params![provider, api_key, api_url, model, now],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmSettings {
    pub id: i32,
    pub provider: String,
    pub api_key: Option<String>,
    pub api_url: String,
    pub model: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

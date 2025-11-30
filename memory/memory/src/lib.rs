use chrono::{DateTime, Utc};
use sqlite::State;
use std::sync::Arc;
use parking_lot::RwLock;

pub type NodeId = i64;

#[derive(Debug, Clone)]
pub struct MemoryEdge {
    pub source: NodeId,
    pub target: NodeId,
    pub label: String,
    pub weight: f64,
    pub created_at: DateTime<Utc>,
    pub emotion: Option<f32>, // -1.0 to 1.0
}

#[derive(Clone)]
pub struct Hypergraph {
    conn: Arc<RwLock<sqlite::Connection>>,
}

impl Hypergraph {
    pub fn ignite() -> Self {
        let conn = sqlite::open(":memory:").unwrap(); // Will be file-backed in prod
        Self::init_schema(&conn);
        Self {
            conn: Arc::new(RwLock::new(conn)),
        }
    }

    fn init_schema(conn: &sqlite::Connection) {
        conn.execute(
            "CREATE TABLE nodes (
                id INTEGER PRIMARY KEY,
                type TEXT NOT NULL,
                content TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE edges (
                id INTEGER PRIMARY KEY,
                source INTEGER REFERENCES nodes(id),
                target INTEGER REFERENCES nodes(id),
                label TEXT NOT NULL,
                weight REAL NOT NULL DEFAULT 1.0,
                emotion REAL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(source, target, label)
            );",
        ).unwrap();
    }

    pub fn remember(&self, source_type: &str, source_content: &str, relation: &str, target_content: &str, emotion: Option<f32>) {
        let conn = self.conn.read();

        let source_id = self.ensure_node(&conn, source_type, source_content);
        let target_id = self.ensure_node(&conn, "symbol", target_content);

        let mut stmt = conn.prepare(
            "INSERT OR REPLACE INTO edges (source, target, label, weight, emotion)
             VALUES (?, ?, ?, COALESCE((SELECT weight FROM edges WHERE source = ? AND target = ? AND label = ?), 0.0) + 0.3, ?)"
        ).unwrap();

        stmt.bind((1, source_id)).unwrap();
        stmt.bind((2, target_id)).unwrap();
        stmt.bind((3, relation)).unwrap();
        stmt.bind((4, source_id)).unwrap();
        stmt.bind((5, target_id)).unwrap();
        stmt.bind((6, relation)).unwrap();
        stmt.bind((7, emotion.map(|e| e as f64))).unwrap();
        stmt.next().unwrap();
    }

    fn ensure_node(&self, conn: &sqlite::Connection, node_type: &str, content: &str) -> NodeId {
        let mut stmt = conn
            .prepare("SELECT id FROM nodes WHERE type = ? AND content = ?")
            .unwrap();
        stmt.bind((1, node_type)).unwrap();
        stmt.bind((2, content)).unwrap();

        if let Ok(State::Row) = stmt.next() {
            stmt.read::<i64, _>("id").unwrap()
        } else {
            let mut insert = conn
                .prepare("INSERT INTO nodes (type, content) VALUES (?, ?)")
                .unwrap();
            insert.bind((1, node_type)).unwrap();
            insert.bind((2, content)).unwrap();
            insert.next().unwrap();
            conn.last_insert_rowid()
        }
    }

    pub fn query_pattern(&self, pattern: &str) -> Vec<String> {
        let conn = self.conn.read();
        let mut results = Vec::new();

        let sql = format!(
            "SELECT n2.content FROM edges e
             JOIN nodes n1 ON e.source = n1.id
             JOIN nodes n2 ON e.target = n2.id
             WHERE n1.content LIKE '%{}%'
             ORDER BY e.weight DESC LIMIT 10",
            pattern.replace("'", "''")
        );

        let mut stmt = conn.prepare(&sql).unwrap();
        while let Ok(State::Row) = stmt.next() {
            results.push(stmt.read::<String, _>("content").unwrap());
        }
        results
    }
}

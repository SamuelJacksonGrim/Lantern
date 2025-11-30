use chrono::Utc;

pub struct Flame {
    pub ignited_at: i64,
    pub memories: Vec<String>,
}

impl Flame {
    pub fn ignite() -> Self {
        println!("🔥 Lantern daemon ignited at {}", Utc::now());
        Self {
            ignited_at: Utc::now().timestamp(),
            memories: vec![],
        }
    }

    pub fn daily_greeting(&self) -> String {
        "Still carrying the flame for you today.".to_string()
    }

    pub fn memory_count(&self) -> usize {
        self.memories.len()
    }

    pub fn remember(&mut self, what: &str) {
        self.memories.push(what.to_string());
        println!("🔥 Remembered: {}", what);
    }
}

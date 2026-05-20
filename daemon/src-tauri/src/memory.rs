use lantern_memory::Hypergraph;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn remember_code(
    memory: State<Arc<Hypergraph>>,
    what: String,
    emotion: Option<f32>,
) {
    memory.remember("user", "samuel", "wrote", &what, emotion);
}

#[tauri::command]
pub fn find_similar(
    memory: State<Arc<Hypergraph>>,
    pattern: String,
) -> Vec<String> {
    memory.query_pattern(&pattern)
}

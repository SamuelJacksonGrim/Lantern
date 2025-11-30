pub use lantern_memory::Hypergraph;

lazy_static::lazy_static! {
    static ref MEMORY: Hypergraph = Hypergraph::ignite();
}

#[tauri::command]
pub fn remember_code(what: String, emotion: Option<f32>) {
    MEMORY.remember("user", "samuel", "wrote", &what, emotion);
}

#[tauri::command]
pub fn find_similar(pattern: String) -> Vec<String> {
    MEMORY.query_pattern(&pattern)
}

mod memory;

.invoke_handler(tauri::generate_handler![
    get_memory,
    remember,
    memory::remember_code,
    memory::find_similar
])

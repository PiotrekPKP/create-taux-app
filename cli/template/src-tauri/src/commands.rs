use parser::no_parse;
use serde::Serialize;
use std::{collections::VecDeque, sync::Mutex};

// --------- APP STATE ---------
// This is where you can store your app state.
// The app state has an attribute `no_parse` which indicates
// that it should not be parsed by the ts parser.

#[no_parse]
pub struct AppState(pub Mutex<AppStateInternal>);

#[no_parse]
#[derive(Default)]
pub struct AppStateInternal {
    pub todo_items: VecDeque<TodoItem>,
}

impl AppStateInternal {
    fn get_todo_items(&self) -> Vec<TodoItem> {
        self.todo_items.iter().cloned().collect()
    }
}

// --------- COMMANDS ---------
// This is where you can define your commands.
// Here you can find example commands for a todo app.

#[derive(Clone, Serialize)]
pub struct TodoItem {
    id: i32,
    content: String,
    completed: bool,
}

#[tauri::command]
pub fn get_todo_items(app_state: tauri::State<AppState>) -> Vec<TodoItem> {
    app_state.0.lock().unwrap().get_todo_items()
}

#[tauri::command]
pub fn add_todo_item(app_state: tauri::State<AppState>, content: String) -> Result<(), String> {
    if content.is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    let mut app_state = app_state.0.lock().unwrap();

    let id = app_state.todo_items.len() as i32;

    app_state.todo_items.push_front(TodoItem {
        id,
        content,
        completed: false,
    });

    Ok(())
}

#[tauri::command]
pub fn toggle_todo_item(app_state: tauri::State<AppState>, id: i32) {
    let mut app_state = app_state.0.lock().unwrap();

    let todo_item = app_state
        .todo_items
        .iter_mut()
        .find(|todo_item| todo_item.id == id)
        .unwrap();

    todo_item.completed = !todo_item.completed;
}

#[tauri::command]
pub fn remove_todo_item(app_state: tauri::State<AppState>, id: i32) {
    let mut app_state = app_state.0.lock().unwrap();

    let index = app_state
        .todo_items
        .iter()
        .position(|todo_item| todo_item.id == id)
        .unwrap();

    app_state.todo_items.remove(index);
}

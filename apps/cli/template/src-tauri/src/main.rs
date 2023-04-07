#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod parser;
mod utils;

use commands::*;
use utils::*;

// Path to the typescript file that will be generated
const TS_FILE_PATH: &str = "src/types/rust.ts";

// Path to the rust file containing files (this file is optional)
const TYPES_FILE_PATH: &str = "src-tauri/src/types.rs";

// Path to the rust file containing commands
const COMMANDS_FILE_PATH: &str = "src-tauri/src/commands.rs";

fn main() {
    tauri::Builder::default()
        .setup(setup_app)
        .manage(AppState(Default::default()))
        // Put your Rust commands here
        .invoke_handler(tauri::generate_handler![
            get_todo_items,
            add_todo_item,
            toggle_todo_item,
            remove_todo_item
        ])
        .run(tauri::generate_context!())
        .unwrap();
}

use crate::parser::generate_typescript_command_types;
use std::process::exit;
use tauri::App;

// This function is used to determine if the app is running in development mode
pub fn is_tauri_dev() -> bool {
    if let Ok(var) = std::env::var("TAURI_DEV") {
        var == "true"
    } else {
        false
    }
}

pub fn setup_app(app: &mut App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Generates typescript types during development
    if cfg!(debug_assertions) && is_tauri_dev() {
        generate_typescript_command_types();
    }

    match app.get_cli_matches() {
        Ok(matches) => {
            if let Some(subcommand) = matches.subcommand {
                match subcommand.name.as_str() {
                    // Command run on app build - do not remove!
                    // This is used to generate typescript types and is run during the build process
                    "generate" => {
                        generate_typescript_command_types();
                        exit(0);
                    }
                    _ => {
                        println!("Unknown subcommand!");
                        exit(1);
                    }
                }
            }
        }
        Err(_) => {}
    }

    Ok(())
}

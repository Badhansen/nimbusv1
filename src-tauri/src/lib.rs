// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetResponse {
    message: String,
}

#[tauri::command]
async fn greet(name: Option<String>) -> Result<String, String> {
    let name = name.unwrap_or_else(|| "World".to_string());
    let url = format!("http://localhost:8008/greet?name={}", name);

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<GreetResponse>().await {
                Ok(greet_response) => Ok(greet_response.message),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to call FastAPI: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        // Start the FastAPI server as a sidecar
        use tauri_plugin_shell::ShellExt;
        let _sidecar = app.shell()
            .sidecar("fastapi-server")
            .expect("failed to create sidecar")
            .args(["--port", "8008"])
            .spawn()
            .expect("failed to spawn sidecar");

        Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

mod lcu;
mod shaco;

use lcu::{
    get_game_path, get_mastery_champ_list, get_rank_point, get_summoner_honor, get_summoner_info,
    listen_client_start, start_client, start_game, start_listener,
};
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_game,
            listen_client_start,
            get_game_path,
            start_listener,
            start_client,
            get_summoner_info,
            get_rank_point,
            get_summoner_honor,
            get_mastery_champ_list
        ])
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

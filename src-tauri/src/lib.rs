// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let local_data_dir = app.path().app_local_data_dir()?;
            let stronghold_path = local_data_dir.join("stronghold");
            let _ = std::fs::create_dir_all(&stronghold_path);
            let salt_path = stronghold_path.join("salt.txt");
            // println!("{}", salt_path.to_string_lossy());
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

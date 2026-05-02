#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod storage;
mod master;

fn main() {
    storage::init_db().expect("Impossible d'initialiser la base de données");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // └── active le plugin Shell
        //     permet de lancer des processus externes
        .invoke_handler(tauri::generate_handler![
            crypto::tauri_encrypt,
            crypto::tauri_decrypt,
            storage::tauri_init_db,
            storage::tauri_save_session,
            storage::tauri_get_session,
            storage::tauri_delete_session,
            storage::tauri_list_sessions,
            master::tauri_is_first_launch,
            master::tauri_setup_master,
            master::tauri_verify_master,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement de BetterPigeon");
}
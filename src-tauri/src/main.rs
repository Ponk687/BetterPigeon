// Empêche l'ouverture d'une console Windows
// en mode release (inutile sur Linux mais bonne pratique)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Déclare le module crypto
// Rust va chercher ces fichiers automatiquement
mod crypto;
mod storage;
mod master;

fn main() {
    // Initialiser la base de données au lancement
    storage::init_db().expect("Impossible d'initialiser la base de données");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Modules crypto
            crypto::tauri_encrypt,
            crypto::tauri_decrypt,
            // Modules storage
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

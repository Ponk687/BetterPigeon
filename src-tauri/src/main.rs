// Empêche l'ouverture d'une console Windows
// en mode release (inutile sur Linux mais bonne pratique)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Déclare le module crypto
// Rust va chercher src/crypto.rs automatiquement
mod crypto;
mod storage;

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
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement de BetterPigeon");
}

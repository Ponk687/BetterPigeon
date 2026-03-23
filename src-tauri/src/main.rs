// Empêche l'ouverture d'une console Windows
// en mode release (inutile sur Linux mais bonne pratique)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Déclare le module crypto
// Rust va chercher src/crypto.rs automatiquement
mod crypto;

fn main() {
    tauri::Builder::default()
        // On enregistre nos deux commandes Tauri
        // Sans ça, Svelte ne peut pas les appeler
        .invoke_handler(tauri::generate_handler![
            crypto::tauri_encrypt,
            crypto::tauri_decrypt,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement de BetterPigeon");
}

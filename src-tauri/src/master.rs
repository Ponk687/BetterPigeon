use crate::crypto;
use rusqlite::Connection;
use std::path::PathBuf;

// Récupère le chemin de la base
// On réutilise la même DB que storage.rs
fn get_db_path() -> PathBuf {
    let mut path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("betterpigeon");
    std::fs::create_dir_all(&path)
        .expect("Impossible de créer le dossier de config");
    path.push("sessions.db");
    path
}

// Vérifie si c'est le premier lancement
// → la table master n'existe pas encore
pub fn is_first_launch() -> bool {
    let conn = match Connection::open(get_db_path()) {
        Ok(c) => c,
        Err(_) => return true,
        // └── si on ne peut pas ouvrir la Base de donnée
        //     on considère que c'est le premier lancement
    };

    // On cherche si la table "master" existe dans SQLite
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master 
         WHERE type='table' AND name='master'",
        [],
        |row| row.get::<_, i64>(0),
    )
    .map(|count| count > 0)
    // └── count > 0 = la table existe
    .unwrap_or(false);
    // └── en cas d'erreur → considère "pas de table"

    !exists
    // └── is_first_launch = PAS de table master
}


// Hache le mot de passe maître avec PBKDF2
// Retourne (hash, sel) pour les stocker
fn hash_master(password: &str) -> (String, String) {
    
    // Générer un sel aléatoire
    use aes_gcm::aead::OsRng;
    use aes_gcm::aead::rand_core::RngCore;
    
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    // └── sel unique à chaque création
    //     même mot de passe → hash différent ✅

    // Dériver la clé avec PBKDF2
    let key = crypto::derive_key(password, &salt);
    // └── on réutilise derive_key() de crypto.rs ✅

    // Encoder en base64 pour stocker en texte
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
    (
        BASE64.encode(key),   // le hash
        BASE64.encode(salt),  // le sel
    )
}

// Crée le mot de passe maître — appelé au premier lancement
pub fn setup_master(password: &str) -> Result<(), String> {

    // 1. Hacher le mot de passe
    let (hash, salt) = hash_master(password);

    // 2. Ouvrir la DB et créer la table master
    let conn = Connection::open(get_db_path())
        .map_err(|e| e.to_string())?;

    // 3. Créer la table si elle n'existe pas
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS master (
            id   INTEGER PRIMARY KEY,
            hash TEXT NOT NULL,
            salt TEXT NOT NULL
        );
    ").map_err(|e| e.to_string())?;
    // └── id INTEGER PRIMARY KEY :
    //     on aura toujours UNE seule ligne
    //     id = 1 toujours

    // 4. Stocker le hash et le sel
    conn.execute(
        "INSERT INTO master (id, hash, salt) VALUES (1, ?1, ?2)",
        rusqlite::params![hash, salt],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

// Vérifie le mot de passe au lancement
pub fn verify_master(password: &str) -> Result<bool, String> {

    // 1. Ouvrir la DB
    let conn = Connection::open(get_db_path())
        .map_err(|e| e.to_string())?;

    // 2. Récupérer le hash et le sel stockés
    let (stored_hash, stored_salt): (String, String) = conn.query_row(
        "SELECT hash, salt FROM master WHERE id = 1",
        [],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        // └── on récupère les deux colonnes en même temps
    ).map_err(|e| e.to_string())?;

    // 3. Décoder le sel depuis base64
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
    let salt = BASE64.decode(stored_salt)
        .map_err(|e| e.to_string())?;

    // 4. Recalculer le hash avec le même sel
    let key = crypto::derive_key(password, &salt);
    let computed_hash = BASE64.encode(key);

    // 5. Comparer les deux hash
    Ok(computed_hash == stored_hash)
    // └── true  → mot de passe correct ✅
    //     false → mot de passe incorrect ❌
}




#[tauri::command]
pub fn tauri_is_first_launch() -> bool {
    is_first_launch()
}

#[tauri::command]
pub fn tauri_setup_master(password: String) -> Result<(), String> {
    setup_master(&password)
}

#[tauri::command]
pub fn tauri_verify_master(password: String) -> Result<bool, String> {
    verify_master(&password)
}
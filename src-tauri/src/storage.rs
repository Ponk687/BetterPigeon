use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use crate::crypto;
// └── "crate" = notre propre projet
//     on importe crypto.rs qu'on a déjà écrit

// Structure représentant une session stockée
#[derive(Debug)]
// └── permet d'afficher la struct avec {:?}
//     utile pour le débogage

pub struct Session {        // pub pour etre utilisable dans les autres programmes
    pub platform: String,  // "signal", "matrix", etc.
    pub data: String,      // token chiffré en base64
}




//fonction qui retourne le chemin sous forme d'un buffer
fn get_db_path() -> PathBuf {
    // Trouve le dossier de config de l'OS
    let mut path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    //  └── unwrap_or_else = "si config_dir() retourne None
    //                        utilise le dossier courant (.)"
    //      None = l'équivalent Rust de null/undefined

    // Ajoute notre dossier app
    path.push("betterpigeon");
    //   └── Linux   : ~/.config/betterpigeon/
    //       Windows : C:\Users\X\AppData\Roaming\betterpigeon\
    //       macOS   : ~/Library/Application Support/betterpigeon/

    // Crée le dossier s'il n'existe pas
    std::fs::create_dir_all(&path)
        .expect("Impossible de créer le dossier de config");
    //   └── create_dir_all = mkdir -p en bash
    //       crée tous les dossiers parents si nécessaire

    // Ajoute le nom du fichier SQLite
    path.push("sessions.db");
    //   └── résultat final :
    //       ~/.config/betterpigeon/sessions.db

    path
}





// fonction qui créé ou ouvre le fichier SQLite qui servira à notre base de donnée
pub fn init_db() -> SqlResult<()> {
    // Ouvre (ou crée) le fichier SQLite
    let conn = Connection::open(get_db_path())?;
    //                                         │
    //                                         └── ? = si erreur retourne Err() immédiatement

    // Crée la table sessions si elle n'existe pas
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS sessions (     // langage SQL
            platform TEXT PRIMARY KEY,
            data     TEXT NOT NULL
        );
    ")?;
    //  └── TEXT PRIMARY KEY sur platform :
    //      → une seule session par plateforme
    //      → "signal" ne peut apparaître qu'une fois
    //      → INSERT sur "signal" existant = UPDATE auto

    Ok(())
    // └── () = "unit type" en Rust es l'équivalent de void en JS :  "cette fonction réussit sans retourner de valeur"
}

 //                                               sessions
 //                                               ┌──────────────┬─────────────────────────────┐
 //                                               │ platform     │ data                        │
 //                                               │ TEXT PK      │ TEXT NOT NULL               │
 //                         la table créée:       ├──────────────┼─────────────────────────────┤
 //                                               │ "signal"     │ "x7Fp9mK2qR8nL3wZ4tY6..."   │
 //                                               │ "matrix"     │ "kR3Lw8mN5pQ2xZ7vB9..."     │
 //                                               │ "whatsapp"   │ "9xKp2mR4nL7wZ3tY8..."      │
 //                                               └──────────────┴─────────────────────────────┘
 //                                                      │                    │
 //                                                      └── nom plateforme   └── token chiffré AES-256
 //                                                                en clair             jamais en clair !









// fonction qui sauvegarde une session en prenant en args une référence vers un string pour représenter la plateforme, une référence vers un string pour représenter le token, une référence vers un string pour représenter la mot de passe  et qui retourne la base de donné remplie avec cette session.
pub fn save_session(platform: &str, token: &str, password: &str) -> SqlResult<()> {
    
    // 1. Chiffrer le token AVANT de toucher la base
    //    on appelle notre crypto.rs déjà écrit
    let encrypted = crypto::encrypt(token, password)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
    //    └── map_err convertit une erreur crypto en erreur SQLite pour rester cohérent avec le type de retour SqlResult<()>


    // 2. Ouvrir la connexion à la base de donnée
    let conn = Connection::open(get_db_path())?;


    // 3. INSERT OR REPLACE :
    //    → si "signal" n'existe pas : INSERT
    //    → si "signal" existe déjà  : REPLACE
    //    jamais de doublon possible
    conn.execute(
        "INSERT OR REPLACE INTO sessions
         (platform, data) VALUES (?1, ?2)",
        //                        │    │
        //                        │    └── ?2 = encrypted
        //                        └── ?1 = platform
        //
        // Les ?1 ?2 sont des paramètres liés
        // JAMAIS de concaténation de strings en SQL !
        // → protège contre les injections SQL
        rusqlite::params![platform, encrypted],
    )?;

    Ok(())
}








//  fonction dont le but est de retrouver si la session existe déjà afin de la connécter. prend en args une référence vers un string pour représenter la plateforme (la nom de la messagerie);  une référence vers un string pour représenter la mot de passe;  et qui retourne la session ou si elle est innexistante via une jestion d'erreur
pub fn get_session(platform: &str, password: &str) -> SqlResult<Option<String>> {

    // 1. Ouvrir la connexion à la base
    let conn = Connection::open(get_db_path())?;

    // 2. Chercher la session chiffrée
    let mut stmt = conn.prepare(
        "SELECT data FROM sessions WHERE platform = ?1"
    )?;
    // prepare() compile la requête SQL. cest plus sécurisé et plus rapide que execut() pour les SELECT

    // 3. Exécuter et récupérer le résultat
    let result = stmt.query_row(rusqlite::params![platform], |row| row.get::<_, String>(0), );


    // 4. Gérer les cas possibles
    match result {
        Ok(encrypted) => {
            // Session trouvée → déchiffrer
            let decrypted = crypto::decrypt(&encrypted, password)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e))?;
            Ok(Some(decrypted))
            //    └── Some() = "j'ai une valeur"
        }

        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Pas de session → retourner None
            // ce n'est pas une erreur, juste "vide"
            Ok(None)
            // └── None = "pas de valeur, mais pas d'erreur"
        }

        Err(e) => {
            // Vraie erreur SQLite → la propager
            Err(e)
        }
    }
}









// fonction pour supprimer une session de la base de donnée qui prend en args une référence vers un string pour représenter la plateforme (le nom de la messagerie) et retourne la base sans cette session
pub fn delete_session(platform: &str) -> SqlResult<()> {

    // 1. Ouvrir la connexion
    let conn = Connection::open(get_db_path())?;

    // 2. Supprimer la session
    conn.execute(
        "DELETE FROM sessions WHERE platform = ?1", &[platform],
    )?;

    Ok(())
}







// fonction qui permet de lister les sessions enregistrée dans la base de donnée en retournant via une gestion d'erreur un tableau dynamique de string
pub fn list_sessions() -> SqlResult<Vec<String>> {

    // 1. Ouvrir la connexion à la base
    let conn = Connection::open(get_db_path())?;


    // 2. Préparer la requête
    let mut stmt = conn.prepare(
        "SELECT platform FROM sessions ORDER BY platform"
    )?;


    // 3. Récupérer toutes les lignes
    let platforms = stmt.query_map([], |row| row.get::<_, String>(0), )?.collect::<SqlResult<Vec<String>>>()?;  //query_map = itère sur chaque lignes du resultat      get(0) = colonne 0 = "platform"   collect() = transforme un itérateur en collection (ici Vec<String>)


    Ok(platforms)
}


















// les commandes Tauri


#[tauri::command]
pub fn tauri_init_db() -> Result<(), String> {
    init_db().map_err(|e| e.to_string())
    //         └── convertit SqlResult en Result<(), String>
    //             que Tauri sait sérialiser en JSON
}

#[tauri::command]
pub fn tauri_save_session(
    platform: String,
    token: String,
    password: String
) -> Result<(), String> {
    save_session(&platform, &token, &password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tauri_get_session(
    platform: String,
    password: String
) -> Result<Option<String>, String> {
    get_session(&platform, &password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tauri_delete_session(
    platform: String
) -> Result<(), String> {
    delete_session(&platform)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tauri_list_sessions() -> Result<Vec<String>, String> {
    list_sessions().map_err(|e| e.to_string())
}

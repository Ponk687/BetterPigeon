use aes_gcm::{           // "use" = import (comme import en JS)
    aead::{              // on importe depuis le sous-module "aead"
        Aead,            // un trait (interface) pour chiffrer
        KeyInit,         // un trait pour initialiser une clé
        OsRng,           // générateur aléatoire de l'OS
        rand_core::RngCore  // trait de base pour tout RNG
    },
    Aes256Gcm,           // l'algorithme AES-256-GCM lui-même
    Nonce,               // le "numéro utilisé une fois" (IV)
    Key                  // type représentant une clé crypto
};

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};


// Constantes de sécurité
const SALT_LEN: usize = 32;   // 2**32 = 256 bits de sel
const NONCE_LEN: usize = 12;  // 2**12 = 96 bits pour AES-GCM
const KEY_LEN: usize = 32;    // 2**32 = 256 bits pour AES-256
const PBKDF2_ROUNDS: u32 = 100_000; // 100 000 itérations (cela limite le nombre de tentative par secondes à 10 000 rendant le brut forces impossibles)






// fonction public qui prend en arrgs : password = référence vers un string; salt = tableau d'octets & = référence (pas de copie); et renvoit -> [u8; KEY_LEN] = retounre un tableau de 32 octets
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    
    // On crée un tableau de 32 zéros
    // C'est là qu'on va stocker la clé générée
    let mut key = [0u8; KEY_LEN];
    //  │    │     │
    //  │    │     └── [0u8; 32] = tableau de 32 octets
    //  │    │          initialisés à zéro
    //  │    │          u8 = entier non signé sur 8 bits (0-255)
    //  │    │
    //  │    └── mut = mutable, la variable peut être modifiée
    //  │         En Rust, tout est IMMUTABLE par défaut !
    //  │         Sans mut → erreur de compilation
    //  │
    //  └── let = déclarer une variable (comme const/let en JS)

    pbkdf2_hmac::<Sha256>(
    //           │
    //           └── ::<Sha256> = paramètre de type générique
    //               "utilise SHA-256 comme fonction de hachage"
    //               Rust vérifie ça à la compilation

        password.as_bytes(),
        //       │
        //       └── convertit &str en &[u8]
        //           les fonctions crypto travaillent
        //           en octets, pas en texte

        salt,            // le sel qu'on a passé en paramètre

        PBKDF2_ROUNDS,   // 100 000 itérations

        &mut key,
        //   │
        //   └── on passe une référence MUTABLE vers key
        //       pbkdf2_hmac va ÉCRIRE la clé dedans
        //       directement, sans créer de copie
    );

    key  // ← pas de "return" ni de ";"
         //   En Rust, la dernière expression
         //   EST la valeur retournée
}






// fonction publique POUR CHIFFRER qui prend en argument une donné en référence vers un sting et un passwrd en référence vers un string pour retourner un Result<String, String> soit le tout encrypté (Result<String, String> permet la gestion d'erreur en rust avec "Result = soit Ok(valeur) soit Err(message)  et  String = type du succès (ici un String base64)   et   String = type de l'erreur (ici un String)") 
pub fn encrypt(data: &str, password: &str) -> Result<String, String> {
    
    // 1. Générer un sel aléatoire (32 octets)
    let mut salt = [0u8; SALT_LEN]; // on fabrique le tableau mutable de 32 octets initié à 0
    OsRng.fill_bytes(&mut salt);    // via OsRng, on fait du random depuis l'ordinateur pour chaque case du tableau

    // 2. Générer un nonce aléatoire (12 octets)
    let mut nonce_bytes = [0u8; NONCE_LEN];   // on fabrique le tableau mutable de 32 octets initié à 0
    OsRng.fill_bytes(&mut nonce_bytes);       // via OsRng, on fait du random depuis l'ordinateur pour chaque case du tableau

    // 3. Dériver la clé depuis le mot de passe + sel
    let key_bytes = derive_key(password, &salt);          // avec la fonction précédente, on vient dériver la clée depuis le mdp et le sel
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);   // on récupere la clée

    // 4. Créer le chiffreur AES-256-GCM
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 5. Chiffrer les données
    let encrypted = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| e.to_string())?;   // avec gestion d'erreur

    // 6. Assembler : sel + nonce + données chiffrées
    let mut combined = Vec::new();
    combined.extend_from_slice(&salt);
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&encrypted);

    // 7. Encoder en base64 pour stocker en texte
    Ok(BASE64.encode(combined))

                                              //                                   [ sel 32 octets | nonce 12 octets | données chiffrées ]   
    //combined car ici On stocke TOUT dans un seul bloc ensuite passé en base64 : └───────────────────────────────────────────────────────┘   de cette maniere, au dechiffrement on sait exactement où couper : octets 0 à 31 = sel; octets 32 à 43 = nonce; octets 44+ = données
    //                                                                                  encode en base64  ---> "x7Fp9mK2qR8nL3wZ4tY6..."

    /*
    on passe en base 64 car: 
    Les octets bruts : [0xFF, 0x8B, 0x00, 0x3A...]
                    │
                    └── Contient des caractères invisibles,
                        des zéros, des caractères spéciaux
                        SQLite et JSON ne savent pas
                        les stocker proprement ❌

Base64 : "x7Fp9mK2qR8nL3wZ..."
          │
          └── Uniquement A-Z, a-z, 0-9, +, /
              100% stockable en texte
              dans SQLite, JSON, fichiers ✅
     */

}








// fonction publique POUR DECHIFFRER qui prend argument une référence à un string qui représente la donnée chiffrée et une référence à un string qui représente la mot de passe pour retrourné le résultat avec une gestion d'erreure via Result
pub fn decrypt(encrypted_b64: &str, password: &str) -> Result<String, String> {


    // 1. Décoder le base64 pour retrouver les octets bruts
    let combined = BASE64
        .decode(encrypted_b64)
        .map_err(|e| e.to_string())?;
    //  └── Si le base64 est corrompu → retourne Err()


    // 2. Vérifier que les données ne sont pas tronquées (corrompues)
    //    minimum = 32 (sel) + 12 (nonce) + 1 (au moins 1 octet de données)
    if combined.len() < SALT_LEN + NONCE_LEN + 1 {
        return Err("Données chiffrées trop courtes".to_string());
    }


    // 3. Découper le bloc en 3 parties
    //    On sait exactement où couper grâce aux constantes
    let salt = &combined[0..SALT_LEN];
    //                   │   │
    //                   │   └── 32 (exclusif)
    //                   └── 0 (inclusif)

    let nonce_bytes = &combined[SALT_LEN..SALT_LEN + NONCE_LEN];
    //                           │         │
    //                           └── 32    └── 44

    let encrypted = &combined[SALT_LEN + NONCE_LEN..];
    //                         └── 44 jusqu'à la fin


    // 4. Reconstruire la même clé avec le même sel
    //    PBKDF2 est déterministe :
    //    même password + même salt = toujours même clé
    let key_bytes = derive_key(password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);


    // 5. Recréer le chiffreur et le nonce
    let cipher = Aes256Gcm::new(key);
    let nonce  = Nonce::from_slice(nonce_bytes);


    // 6. Déchiffrer
    //    AES-GCM vérifie aussi l'intégrité automatiquement :
    //    si les données ont été modifiées → Err()
    let decrypted = cipher
        .decrypt(nonce, encrypted)
        .map_err(|_| "Déchiffrement échoué — mauvais mot de passe ou données corrompues".to_string())?;
    //             │
    //             └── On cache l'erreur interne (_)
    //                 pour ne pas fuiter d'infos techniques
    //                 à un attaquant potentiel


    // 7. Convertir les octets en String UTF-8
    String::from_utf8(decrypted)
        .map_err(|e| e.to_string())
    //  └── from_utf8 peut échouer si les octets
    //      ne forment pas du texte valide
    //      (ne devrait pas arriver en pratique)
}
















// Ces fonctions sont appelables depuis Svelte
// via window.__TAURI__.invoke('nom_fonction')

#[tauri::command]
// └── Cette annotation dit à Tauri :
//     "expose cette fonction au frontend"

pub fn tauri_encrypt(data: String, password: String) -> Result<String, String> {
    // On appelle simplement notre fonction encrypt avec une gestion d'erreur
    encrypt(&data, &password)
}




#[tauri::command]
pub fn tauri_decrypt(encrypted: String, password: String) -> Result<String, String> {
    // On appelle simplement notre fonction decrypt avec une gestion d'erreur
    decrypt(&encrypted, &password)
}
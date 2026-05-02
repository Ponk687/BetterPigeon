use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

// Lance le sidecar Node.js au démarrage
pub fn start_bridge(app: &tauri::App) -> Result<(), String> {

    let shell = app.shell();
    // └── accès au plugin Shell

    let (mut rx, child) = shell
        .sidecar("bridge")
        // └── cherche binaries/bridge-x86_64-unknown-linux-gnu
        .map_err(|e| e.to_string())?
        .spawn()
        .map_err(|e| e.to_string())?;
    // └── lance le processus et retourne :
    //     rx    = canal pour recevoir les événements
    //     child = handle du processus (pour l'arrêter)

    // Écouter les événements du sidecar dans un thread séparé
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // └── message reçu du sidecar
                    let text = String::from_utf8_lossy(&line);
                    log::info!("Bridge: {}", text);
                    // └── on logge pour l'instant
                    //     plus tard on enverra à Svelte
                }
                CommandEvent::Stderr(line) => {
                    let text = String::from_utf8_lossy(&line);
                    log::warn!("Bridge stderr: {}", text);
                }
                CommandEvent::Error(e) => {
                    log::error!("Bridge error: {}", e);
                }
                CommandEvent::Terminated(status) => {
                    log::warn!("Bridge terminated: {:?}", status);
                    break;
                }
                _ => {}
            }
        }
    });

    // Stocker le child dans l'état global de Tauri
    // pour pouvoir l'arrêter proprement plus tard
    app.manage(std::sync::Mutex::new(Some(child)));

    Ok(())
}
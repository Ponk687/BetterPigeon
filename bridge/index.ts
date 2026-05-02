// Point d'entrée du sidecar Node.js
// Ce process tourne en permanence en arrière-plan

import * as matrix from './matrix/index.js';

// Fonction pour envoyer un événement à Tauri
function sendToTauri(event: object): void {
  // stdout = canal de communication vers Tauri
  // JSON.stringify = convertit l'objet en texte
  process.stdout.write(JSON.stringify(event) + '\n');
}

// Fonction pour gérer les commandes reçues de Tauri
async function handleCommand(command: any): Promise<void> {
  try {
    switch (command.action) {

      case 'matrix_login': {
        // Connexion à Matrix avec login/password
        const credentials = await matrix.loginWithPassword(
          command.homeserverUrl,
          command.username,
          command.password
        );
        sendToTauri({
          event: 'matrix_login_success',
          credentials
        });
        break;
      }

      case 'matrix_connect': {
        // Reconnexion avec token existant
        await matrix.loginWithToken(command.credentials);
        sendToTauri({ event: 'matrix_connected' });
        break;
      }

      case 'matrix_get_rooms': {
        // Récupérer les salons
        const rooms = await matrix.getRooms();
        sendToTauri({ event: 'matrix_rooms', rooms });
        break;
      }

      case 'matrix_send_message': {
        // Envoyer un message
        await matrix.sendMessage(command.roomId, command.content);
        sendToTauri({ event: 'matrix_message_sent' });
        break;
      }

      case 'matrix_logout': {
        await matrix.logout();
        sendToTauri({ event: 'matrix_logged_out' });
        break;
      }

      default:
        sendToTauri({
          event: 'error',
          message: `Commande inconnue : ${command.action}`
        });
    }
  } catch (error) {
    // En cas d'erreur → on notifie Tauri sans crasher
    sendToTauri({
      event: 'error',
      message: String(error)
    });
  }
}

// Écouter les commandes de Tauri via stdin
// Les commandes arrivent ligne par ligne en JSON
let buffer = '';

process.stdin.on('data', (chunk: Buffer) => {
  buffer += chunk.toString();
  // └── on accumule les données dans un buffer
  //     car les données peuvent arriver en morceaux

  const lines = buffer.split('\n');
  // └── on découpe par lignes

  buffer = lines.pop() ?? '';
  // └── la dernière ligne peut être incomplète
  //     on la garde pour la prochaine fois

  for (const line of lines) {
    if (!line.trim()) continue;
    // └── on ignore les lignes vides

    try {
      const command = JSON.parse(line);
      handleCommand(command);
    } catch {
      sendToTauri({
        event: 'error',
        message: 'Commande JSON invalide'
      });
    }
  }
});

// Signaler à Tauri que le sidecar est prêt
sendToTauri({ event: 'ready' });
process.stderr.write('BetterPigeon bridge started\n');

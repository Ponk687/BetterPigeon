import * as sdk from 'matrix-js-sdk';
import type { MatrixCredentials, MatrixRoom, MatrixMessage } from './types.js';

// Instance globale du client Matrix
// undefined = pas encore connecté
let client: sdk.MatrixClient | undefined;

// Connexion avec login + mot de passe
export async function loginWithPassword(
  homeserverUrl: string,
  username: string,
  password: string
): Promise<MatrixCredentials> {

  // 1. Créer un client temporaire pour le login
    const tempClient = sdk.createClient({ baseUrl: homeserverUrl });
  //                                               └── ex: "https://matrix.org"

  // 2. Se connecter
  const response = await tempClient.login('m.login.password', {
    user: username,      // "@jules:matrix.org" ou "jules"
    password: password,
    initial_device_display_name: 'BetterPigeon'
    // └── nom affiché dans la liste des appareils connectés
  });

  // 3. Retourner les credentials à stocker
  return {
    homeserverUrl,
    accessToken: response.access_token,
    // └── token à chiffrer et stocker dans SQLite
    userId: response.user_id,
    deviceId: response.device_id,
  };
}

// Connexion avec un token existant (après redémarrage)
export async function loginWithToken(
  credentials: MatrixCredentials
): Promise<void> {

  // Créer le client avec le token stocké
  client = sdk.createClient({
    baseUrl: credentials.homeserverUrl,
    accessToken: credentials.accessToken,
    userId: credentials.userId,
    deviceId: credentials.deviceId,
  });

  // Démarrer la synchronisation avec le serveur
  await client.startClient({ initialSyncLimit: 20 });
  // └── initialSyncLimit = nombre de messages
  //     à récupérer au démarrage
}

// Récupérer la liste des salons
export async function getRooms(): Promise<MatrixRoom[]> {
  if (!client) throw new Error('Client Matrix non connecté');

  const rooms = client.getRooms();
  // └── retourne les salons déjà synchronisés

  return rooms.map(room => ({
    id: room.roomId,
    name: room.name,
    lastMessage: getLastMessage(room),
  }));
}

// Récupérer le dernier message d'un salon
function getLastMessage(room: sdk.Room): MatrixMessage | undefined {
  const timeline = room.getLiveTimeline().getEvents();
  for (let i = timeline.length - 1; i >= 0; i--) {
    const event = timeline[i];
    if (!event) continue;  // ← ajoute cette ligne
    if (event.getType() === 'm.room.message') {
      return {
        id: event.getId() ?? '',
        roomId: room.roomId,
        sender: event.getSender() ?? '',
        content: event.getContent().body ?? '',
        timestamp: event.getTs(),
      };
    }
  }
  return undefined;
}

// Envoyer un message
export async function sendMessage(
  roomId: string,
  content: string
): Promise<void> {
  if (!client) throw new Error('Client Matrix non connecté');

  await client.sendTextMessage(roomId, content);
}

// Déconnecter
export async function logout(): Promise<void> {
  if (!client) return;
  await client.logout();
  client = undefined;
}
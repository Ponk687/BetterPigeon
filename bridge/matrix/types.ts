// Un message Matrix
export interface MatrixMessage {
  id: string;          // identifiant unique du message
  roomId: string;      // identifiant du salon
  sender: string;      // adr matrix de l'expediteur ex : "@exemple:matrix.org"
  content: string;     // contenu du message
  timestamp: number;   // date en millisecondes
}

// Un salon (conversation)
export interface MatrixRoom {
  id: string;          // id du salon ex:"!abc123:matrix.org"
  name: string;        // "nom de la conversation"
  lastMessage?: MatrixMessage;  // dernier message
  // └── ? = optionnel, peut être undefined
}

// Les credentials de connexion
export interface MatrixCredentials {
  homeserverUrl: string;  // "https://matrix.org"
  accessToken: string;    // token de session
  userId: string;         // adr du compte ex : "@exemple:matrix.org"
  deviceId: string;       // identifiant de l'appareil
}
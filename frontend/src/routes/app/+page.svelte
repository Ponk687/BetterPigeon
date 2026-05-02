<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  let homeserver = 'https://matrix.org';
  let username = '';
  let password = '';
  let error = '';
  let loading = false;
  let connected = false;

  async function connectMatrix() {
    if (!username || !password) {
      error = 'Remplis tous les champs';
      return;
    }
    loading = true;
    error = '';
    try {
      // On envoie la commande au sidecar via Tauri
      // On implémentera ça dans la prochaine étape
      await new Promise(r => setTimeout(r, 1000));
      connected = true;
    } catch(e) {
      error = 'Erreur de connexion : ' + e;
    }
    loading = false;
  }
</script>

<div class="content">
  {#if !connected}
    <div class="connect-form">
      <div class="platform-header">
        <span class="icon">🟦</span>
        <h2>Connexion Matrix</h2>
      </div>

      <div class="field">
        <label>Serveur</label>
        <input
          type="text"
          bind:value={homeserver}
          placeholder="https://matrix.org"
        />
      </div>

      <div class="field">
        <label>Identifiant</label>
        <input
          type="text"
          bind:value={username}
          placeholder="@toi:matrix.org"
        />
      </div>

      <div class="field">
        <label>Mot de passe</label>
        <input
          type="password"
          bind:value={password}
          placeholder="••••••••"
          on:keydown={(e) => e.key === 'Enter' && connectMatrix()}
        />
      </div>

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <button on:click={connectMatrix} disabled={loading}>
        {loading ? 'Connexion...' : 'Se connecter'}
      </button>

      <p class="hint">
        Pas de compte ? Crée-en un sur
        <a href="https://app.element.io" target="_blank">element.io</a>
      </p>
    </div>

  {:else}
    <div class="connected">
      <span class="check">✅</span>
      <h2>Connecté à Matrix !</h2>
      <p>Les conversations arrivent bientôt...</p>
    </div>
  {/if}
</div>

<style>
  .content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .connect-form {
    background: #1a1a1a;
    border: 1px solid #2a2a2a;
    border-radius: 16px;
    padding: 2rem;
    width: 100%;
    max-width: 400px;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .platform-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .icon { font-size: 1.5rem; }

  h2 {
    margin: 0;
    font-size: 1.3rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  label {
    font-size: 0.85rem;
    color: #888;
  }

  input {
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 0.75rem 1rem;
    color: #fff;
    font-size: 1rem;
    outline: none;
    transition: border-color 0.2s;
  }

  input:focus { border-color: #0DBD8B; }

  button {
    background: #0DBD8B;
    color: #000;
    border: none;
    border-radius: 8px;
    padding: 0.75rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
    margin-top: 0.5rem;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button:hover:not(:disabled) { opacity: 0.85; }

  .error {
    color: #ff4444;
    font-size: 0.9rem;
    margin: 0;
  }

  .hint {
    font-size: 0.8rem;
    color: #666;
    text-align: center;
    margin: 0;
  }

  .hint a { color: #0DBD8B; }

  .connected {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .check { font-size: 3rem; }
</style>


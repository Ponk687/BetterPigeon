<script lang="ts">
  import { onMount } from 'svelte';

  let isFirstLaunch: boolean | null = null;
  let password = '';
  let error = '';
  let loading = false;

  onMount(() => {
    // On attend que __TAURI__ soit disponible
    const check = setInterval(async () => {
      if ((window as any).__TAURI__) {
        clearInterval(check);
        try {
          const result = await (window as any).__TAURI__.core.invoke('tauri_is_first_launch');
          console.log('isFirstLaunch:', result);
          isFirstLaunch = result;
        } catch(e) {
          console.error('Erreur:', e);
          isFirstLaunch = false;
        }
      }
    }, 100);
  });

  async function invoke(cmd: string, args?: object) {
    return (window as any).__TAURI__.core.invoke(cmd, args);
  }

  async function setupMaster() {
    if (password.length < 8) {
      error = 'Le mot de passe doit faire au moins 8 caractères';
      return;
    }
    loading = true;
    error = '';
    try {
      await invoke('tauri_setup_master', { password });
      isFirstLaunch = false;
    } catch (e) {
      error = 'Erreur : ' + e;
    }
    loading = false;
  }

  async function unlockApp() {
    if (!password) {
      error = 'Saisis ton mot de passe';
      return;
    }
    loading = true;
    error = '';
    try {
      const ok = await invoke('tauri_verify_master', { password });
      if (ok) {
        alert('✅ Bienvenue dans BetterPigeon !');
      } else {
        error = 'Mot de passe incorrect';
      }
    } catch (e) {
      error = 'Erreur : ' + e;
    }
    loading = false;
  }
</script>

<main>
  <div class="logo">🐦</div>
  <h1>BetterPigeon</h1>
  <p class="tagline">Stay connected. Leave the cage.</p>

  {#if isFirstLaunch === null}
    <!-- Chargement -->
    <div class="card">
      <p class="hint">Chargement...</p>
    </div>

  {:else if isFirstLaunch === true}
    <!-- Premier lancement -->
    <div class="card">
      <h2>Crée ton mot de passe maître</h2>
      <p class="hint">
        Ce mot de passe protège toutes tes sessions.<br>
        Il n'est stocké nulle part — ne l'oublie pas.
      </p>
      <input
        type="password"
        placeholder="Minimum 8 caractères"
        bind:value={password}
        on:keydown={(e) => e.key === 'Enter' && setupMaster()}
      />
      <button on:click={setupMaster} disabled={loading}>
        {loading ? 'Création...' : 'Créer'}
      </button>
    </div>

  {:else}
    <!-- Lancements suivants -->
    <div class="card">
      <h2>Bienvenue</h2>
      <input
        type="password"
        placeholder="Ton mot de passe maître"
        bind:value={password}
        on:keydown={(e) => e.key === 'Enter' && unlockApp()}
      />
      <button on:click={unlockApp} disabled={loading}>
        {loading ? 'Vérification...' : 'Ouvrir'}
      </button>
    </div>
  {/if}

  {#if error}
    <p class="error">{error}</p>
  {/if}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 1rem;
    font-family: sans-serif;
    background: #0f0f0f;
    color: #ffffff;
  }

  .logo { font-size: 4rem; }

  h1 {
    font-size: 2rem;
    margin: 0;
  }

  .tagline {
    color: #888;
    font-style: italic;
    margin: 0;
  }

  .card {
    background: #1a1a1a;
    border: 1px solid #2a2a2a;
    border-radius: 12px;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 320px;
    margin-top: 1rem;
  }

  h2 {
    margin: 0;
    font-size: 1.2rem;
    text-align: center;
  }

  .hint {
    font-size: 0.85rem;
    color: #888;
    text-align: center;
    margin: 0;
    line-height: 1.5;
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

  input:focus { border-color: #555; }

  button {
    background: #ffffff;
    color: #000000;
    border: none;
    border-radius: 8px;
    padding: 0.75rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
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
</style>
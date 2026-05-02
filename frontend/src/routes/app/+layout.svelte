<script lang="ts">
  import { page } from '$app/stores';

  const platforms = [
    { id: 'matrix',    icon: '🟦', name: 'Matrix',    color: '#0DBD8B' },
    { id: 'signal',    icon: '📶', name: 'Signal',    color: '#3A76F0' },
    { id: 'whatsapp',  icon: '💬', name: 'WhatsApp',  color: '#25D366' },
    { id: 'instagram', icon: '📸', name: 'Instagram', color: '#E1306C' },
    { id: 'facebook',  icon: '👤', name: 'Facebook',  color: '#1877F2' },
  ];

  let activePlatform = 'matrix';
</script>

<div class="app-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-logo">🐦</div>
    <nav class="platform-list">
      {#each platforms as platform}
        <button
          class="platform-btn"
          class:active={activePlatform === platform.id}
          on:click={() => activePlatform = platform.id}
          style="--color: {platform.color}"
        >
          <span class="platform-icon">{platform.icon}</span>
          <span class="platform-name">{platform.name}</span>
        </button>
      {/each}
    </nav>
  </aside>

  <!-- Zone principale -->
  <main class="main-area">
    <slot />
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    height: 100vh;
    background: #0f0f0f;
    color: #ffffff;
    font-family: sans-serif;
  }

  .sidebar {
    width: 200px;
    background: #111111;
    border-right: 1px solid #222;
    display: flex;
    flex-direction: column;
    padding: 1rem 0;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  .sidebar-logo {
    font-size: 2rem;
    text-align: center;
    padding: 0.5rem 0 1rem;
  }

  .platform-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 0.5rem;
  }

  .platform-btn {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.65rem 0.75rem;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: #888;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
    width: 100%;
    font-size: 0.9rem;
  }

  .platform-btn:hover {
    background: #1a1a1a;
    color: #fff;
  }

  .platform-btn.active {
    background: #1a1a1a;
    color: var(--color);
    font-weight: 600;
  }

  .platform-icon {
    font-size: 1.1rem;
  }

  .main-area {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>

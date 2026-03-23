<script lang="ts">
  const platforms = ['Signal', 'Matrix', 'WhatsApp', 'Instagram', 'Facebook'];
</script>

<main>
  <div class="logo">🐦</div>
  <h1>BetterPigeon</h1>
  <p class="tagline">Stay connected. Leave the cage.</p>

  <div class="platforms">
    {#each platforms as platform}
      <span class="badge">{platform}</span>
    {/each}
  </div>
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
  }

  .logo {
    font-size: 4rem;
  }

  h1 {
    font-size: 2rem;
    margin: 0;
  }

  .tagline {
    color: #666;
    font-style: italic;
  }

  .platforms {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: center;
    margin-top: 1rem;
  }

  .badge {
    background: #f0f0f0;
    border-radius: 999px;
    padding: 0.3rem 0.8rem;
    font-size: 0.85rem;
  }
</style>


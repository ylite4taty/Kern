<script>
  import { onMount } from 'svelte';

  let query = '';
  let results = [];
  let visible = true;

  async function search(q) {
    if (!q.trim()) {
      results = [];
      return;
    }

    try {
      const response = await fetch('http://localhost:3000/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ query: q }),
      });
      const data = await response.json();
      results = data.results ?? [];
    } catch (e) {
      results = ['daemon not connected'];
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      query = '';
      results = [];
    }
  }

  $: search(query);

  onMount(() => {
    document.getElementById('search-input')?.focus();
  });
</script>

<svelte:window on:keydown={handleKeydown} />

{#if visible}
  <div class="overlay">
    <div class="panel">
      <input
        id="search-input"
        type="text"
        bind:value={query}
        placeholder="search documentation..."
        autocomplete="off"
        spellcheck="false"
      />
      {#if results.length > 0}
        <ul class="results">
          {#each results as result}
            <li>{result}</li>
          {/each}
        </ul>
      {/if}
    </div>
  </div>
{/if}

<style>
  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    background: transparent;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
  }

  .overlay {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .panel {
    width: 600px;
    background: #111111ee;
    border: 1px solid #2a2a2a;
    border-radius: 8px;
    overflow: hidden;
    backdrop-filter: blur(12px);
  }

  input {
    width: 100%;
    padding: 16px 20px;
    background: transparent;
    border: none;
    outline: none;
    color: #e8e8e8;
    font-size: 16px;
    font-family: inherit;
    letter-spacing: 0.02em;
  }

  input::placeholder {
    color: #444;
  }

  .results {
    list-style: none;
    border-top: 1px solid #1e1e1e;
  }

  .results li {
    padding: 12px 20px;
    color: #aaa;
    font-size: 14px;
    border-bottom: 1px solid #1a1a1a;
    cursor: pointer;
    transition: background 0.1s;
  }

  .results li:hover {
    background: #1c1c1c;
    color: #e8e8e8;
  }

  .results li:last-child {
    border-bottom: none;
  }
</style>
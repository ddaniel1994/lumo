<script lang="ts">
  export let left = "Ready";
  export let right = "";
</script>

<footer class="statusbar">
  <span class="left">{left}</span>
  {#if right}
    <span class="right">{right}</span>
  {/if}
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 16px;
    background: var(--surface);
    border-top: 1px solid var(--border);
    font-size: 12px;
    color: var(--text-muted);
  }
</style>

<script lang="ts">
  export let title = "";
  export let disabled = false;
  export let label = "";
</script>

<button class="icon-btn" {title} {disabled}>
  <slot />
</button>

<style>
  .icon-btn {
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius);
    padding: 8px 12px;
    cursor: pointer;
    font-size: 14px;
    color: var(--text);
  }

  .icon-btn:hover {
    background: var(--bg);
    border-color: var(--border);
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>

<script lang="ts">
    import { user } from "$lib/firebase";
</script>
  
{#if $user}
  <slot />
{:else}
        <a class="btn btn-warning" href="/login">Sign in</a>
{/if}
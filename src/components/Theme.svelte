<script lang="ts">
  import { onMount } from "svelte";
  import { Sun, Moon } from "lucide-svelte";
  import { themeStore } from "../stores/themeStore";

  onMount(() => {
    themeStore.init();
  });
</script>

<div
  class="relative backdrop-blur-sm text-muted-foreground hover:text-foreground border border-border dark:border-accent p-1.5 bg-accent/30 flex justify-center items-center rounded-full cursor-pointer transition-colors duration-200 ease-in-out"
  on:click={themeStore.toggleTheme}
  role="presentation"
>
  {#if $themeStore}
    <Moon class="w-[16px] h-[16px]" />
  {:else}
    <Sun class="w-[16px] h-[16px]" />
  {/if}
</div>

<script lang="ts">
  import { onMount } from 'svelte';
  import { Sun, Moon } from "lucide-svelte";

  let isDark = false;
  let prefersDark:any;

  onMount(() => {
    prefersDark = window.matchMedia('(prefers-color-scheme: light)');
    
    // Initial setup
    setThemeFromSystem();

    // Listen for changes in system preference
    prefersDark.addEventListener('change', setThemeFromSystem);

    return () => {
      // Clean up the event listener when the component is destroyed
      prefersDark.removeEventListener('change', setThemeFromSystem);
    };
  });

  function setThemeFromSystem() {
    isDark = prefersDark.matches;
    updateTheme();
  }

  function toggleTheme() {
    isDark = !isDark;
    updateTheme();
  }

  function updateTheme() {
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }
</script>

<div 
  class="fixed bg-transparent flex justify-center items-center bottom-4 left-4 w-9 h-9 rounded-lg border cursor-pointer transition-colors duration-200 ease-in-out"
  class:bg-background={!isDark}
  class:bg-secondary={isDark}
  class:border-border={!isDark}
  class:border-secondary={isDark}
  class:hover:bg-secondary={!isDark}
  class:hover:bg-muted={isDark}
  on:click={toggleTheme}
>
  {#if isDark}
    <Sun class="w-4 h-4 text-foreground" />
  {:else}
    <Moon class="w-4 h-4 text-foreground" />
  {/if}
</div>
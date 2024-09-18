<script lang="ts">
  import { onMount } from "svelte";
  import { Sun, Moon } from "lucide-svelte";

  let isDark = false;
  let prefersDark: any;

  onMount(() => {
    prefersDark = window.matchMedia("(prefers-color-scheme: dark)");

    // Initial setup
    setThemeFromSystem();

    // Listen for changes in system preference
    prefersDark.addEventListener("change", setThemeFromSystem);

    return () => {
      // Clean up the event listener when the component is destroyed
      prefersDark.removeEventListener("change", setThemeFromSystem);
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
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }
</script>

<div
  class="relative bg-transparent flex justify-center items-center rounded-full cursor-pointer transition-colors duration-200 ease-in-out"
  on:click={toggleTheme}
  role="presentation"
>
  {#if isDark}
    <Moon class="w-[18px] h-[18px] text-muted-foreground" />
  {:else}
    <Sun class="w-[18px] h-[18px] text-muted-foreground" />
  {/if}
</div>

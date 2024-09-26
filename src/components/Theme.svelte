<script lang="ts">
  import { onMount } from "svelte";
  import { Sun, Moon } from "lucide-svelte";

  let isDark = false;
  let prefersDark: MediaQueryList;

  onMount(() => {
    prefersDark = window.matchMedia("(prefers-color-scheme: dark)");

    // Check local storage first
    const storedTheme = localStorage.getItem("theme");
    if (storedTheme) {
      isDark = storedTheme === "dark";
    } else {
      // If no stored theme, use system preference
      setThemeFromSystem();
    }

    // Apply the theme
    updateTheme();

    // Listen for changes in system preference
    prefersDark.addEventListener("change", handleSystemThemeChange);

    return () => {
      // Clean up the event listener when the component is destroyed
      prefersDark.removeEventListener("change", handleSystemThemeChange);
    };
  });

  function setThemeFromSystem() {
    isDark = prefersDark.matches;
  }

  function handleSystemThemeChange(e: MediaQueryListEvent) {
    // Only update theme if user hasn't set a preference
    if (!localStorage.getItem("theme")) {
      isDark = e.matches;
      updateTheme();
    }
  }

  function toggleTheme() {
    isDark = !isDark;
    updateTheme();
  }

  function updateTheme() {
    if (isDark) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  }
</script>

<div
  class="relative backdrop-blur-sm text-muted-foreground hover:text-foreground border border-border dark:border-accent p-1.5 bg-accent/30 flex justify-center items-center rounded-full cursor-pointer transition-colors duration-200 ease-in-out"
  on:click={toggleTheme}
  role="presentation"
>
  {#if isDark}
    <Moon class="w-[16px] h-[16px]" />
  {:else}
    <Sun class="w-[16px] h-[16px]" />
  {/if}
</div>
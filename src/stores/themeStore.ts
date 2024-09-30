import { writable } from "svelte/store";

function createThemeStore() {
  const { subscribe, set, update } = writable(false);

  let prefersDark: MediaQueryList;

  if (typeof window !== "undefined") {
    prefersDark = window.matchMedia("(prefers-color-scheme: dark)");
  }

  function init() {
    if (typeof window === "undefined") return;

    const storedTheme = localStorage.getItem("theme");
    if (storedTheme) {
      set(storedTheme === "dark");
    } else {
      setThemeFromSystem();
    }

    prefersDark.addEventListener("change", handleSystemThemeChange);
  }

  function setThemeFromSystem() {
    set(prefersDark.matches);
  }

  function handleSystemThemeChange(e: MediaQueryListEvent) {
    if (!localStorage.getItem("theme")) {
      set(e.matches);
    }
  }

  function toggleTheme() {
    update((isDark) => {
      const newTheme = !isDark;
      updateTheme(newTheme);
      return newTheme;
    });
  }

  function updateTheme(isDark: boolean) {
    if (typeof window === "undefined") return;

    if (isDark) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  }

  return {
    subscribe,
    toggleTheme,
    init,
  };
}

export const themeStore = createThemeStore();

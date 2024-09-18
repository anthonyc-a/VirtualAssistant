<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import Header from "../components/Header.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import Titlebar from "../components/Titlebar.svelte";
  import Sidebar from "../components/Sidebar.svelte";
  import Theme from "../components/Theme.svelte";
  import MessageBar from "../components/MessageBar.svelte";
  import Navigation from "../components/Navigation.svelte";
  import Hover from "../components/hover.svelte";
  import Footer from "../components/Footer.svelte";
  import { browser } from '$app/environment';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';

  let themeColor = '';

  function updateThemeColor() {
    if (browser) {
      const isDark = document.documentElement.classList.contains('dark');
      const backgroundColor = getComputedStyle(document.documentElement).getPropertyValue('--background').trim();
      themeColor = backgroundColor || (isDark ? '#1a202c' : '#ffffff'); // Fallback colors
    }
  }

  onMount(() => {
    updateThemeColor();
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateThemeColor);
    
    // Create a MutationObserver to watch for changes to the 'class' attribute of <html>
    const observer = new MutationObserver(updateThemeColor);
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });

    return () => {
      window.matchMedia('(prefers-color-scheme: dark)').removeEventListener('change', updateThemeColor);
      observer.disconnect();
    };
  });

  let time: any = "";

  onMount(async () => {
    // Initial time fetch
    time = await invoke("get_formatted_local_time");

    // Listen for time updates
    await listen("time-update", (event) => {
      time = event.payload;
    });
  });
</script>

<svelte:head>
  <meta name="theme-color" content={themeColor}>
</svelte:head>

<a href="/" class="fixed top-[20px] left-11 md:left-6 z-[99999] invert-[0.95] dark:invert-0">
  <img src="/logo.svg" alt="Logo" class="w-[17px]" />
</a>
<MessageBar />
<Sidebar />
<Navigation />
<Header />

<div in:fade={{ duration: 300 }} out:fade={{ duration: 300 }} class="md:container bg-background w-full pt-3 px-0 md:px-0 md:w-[calc(100%-40px)] max-w-3xl mx-auto">
  <slot />
  <Footer/>
</div>

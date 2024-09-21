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
  import { browser } from "$app/environment";
  import { fade } from "svelte/transition";
  import { navigating } from "$app/stores";
  import { Mail, MessageCircle, SidebarCloseIcon, X } from "lucide-svelte";
  import Loader from "../components/Loader.svelte";
  import FooterNav from "../components/FooterNav.svelte";

  let themeColor = "";
  let messaging = false;

  // Add this line to create a reactive statement
  $: isNavigating = $navigating !== null;

  function updateThemeColor() {
    if (browser) {
      const isDark = document.documentElement.classList.contains("dark");
      const backgroundColor = getComputedStyle(document.documentElement)
        .getPropertyValue("--background")
        .trim();
      themeColor = backgroundColor || (isDark ? "#1a202c" : "#999"); // Fallback colors
    }
  }

  onMount(() => {
    updateThemeColor();
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", updateThemeColor);

    // Create a MutationObserver to watch for changes to the 'class' attribute of <html>
    const observer = new MutationObserver(updateThemeColor);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });

    return () => {
      window
        .matchMedia("(prefers-color-scheme: dark)")
        .removeEventListener("change", updateThemeColor);
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
  <meta name="theme-color" content={themeColor} />
</svelte:head>

<div
  class="fixed hidden md:block text-sm text-muted-foreground bottom-4 right-4"
>
  ⓒ 2024
</div>

<div
  class="w-full z-[99999999] bg-background max-w-screen-lg mx-auto px-11 py-3 flex justify-between items-center sticky top-0 left-0"
>
  <a href="/" class="relative fade-up invert-[0.85] dark:invert-0">
    <img src="/logo.svg" alt="Logo" class=" w-[16px]" />
  </a>
  <div class="flex animate four items-center gap-2.5">
    <div
      class="w-3.5 h-3.5 pulse flex justify-center items-center mr-2 bg-success/20 rounded-full"
    >
      <div class="w-[5px] h-[5px] bg-success rounded-full"></div>
    </div>

    <Theme />
    <div
      role="presentation"
      class="relative backdrop-blur-sm {messaging
        ? 'invert'
        : ''} text-foreground hover:text-foreground border border-accent p-1.5 bg-accent/30 flex justify-center items-center rounded-full cursor-pointer transition-colors duration-200 ease-in-out"
      on:click={() => (messaging = !messaging)}
    >
      {#if messaging}
        <X class="w-[16px] h-[16px]" />
      {:else}
        <MessageCircle class="w-[16px] h-[16px]" />
      {/if}
    </div>
    <div
      role="presentation"
      class="relative backdrop-blur-sm text-foreground hover:text-foreground border border-accent p-1.5 bg-accent/30 hidden md:flex justify-center items-center rounded-full cursor-pointer transition-colors duration-200 ease-in-out"
      on:click={() => (messaging = !messaging)}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="1em"
        height="1em"
        fill="currentColor"
        viewBox="0 0 256 256"
        class="w-[16px] h-[16px] rotate-180"
      >
        <path
          d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM40,56H80V200H40ZM216,200H96V56H216V200Z"
        ></path>
      </svg>
    </div>
  </div>
</div>

<MessageBar {messaging} />

<Navigation />

<Loader />

<Header />

{#key isNavigating}
  <div
    class="md:container two bg-background w-full pt-3 px-0 md:px-0 md:w-[calc(100%-40px)] max-w-3xl mx-auto"
    in:fade={{ duration: 300 }}
  >
    <slot />
    <FooterNav/>
    <Footer />
  </div>
{/key}

<style>
  .fade-up {
    opacity: 0;
    transform: translateX(-50%) translateY(10px);
    animation: fadeUp 0.5s ease-out forwards;
  }

  .pulse {
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.15);
    }
    100% {
      transform: scale(1);
    }
  }

  @keyframes fadeUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .animate {
    opacity: 0;
    animation: fadeIn 0.75s ease-in-out forwards;
  }
  .one {
    animation-delay: 0.15s;
  }

  .two {
    animation-delay: 1s;
  }

  .three {
    animation-delay: 1.5s;
  }
  .four {
    animation-delay: 2s;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>

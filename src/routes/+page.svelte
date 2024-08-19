<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import {
    FolderIcon,
    FileIcon,
    FileTextIcon,
    FileType2,
    FileImageIcon,
    FileVideoIcon,
    FileMusicIcon,
  } from "lucide-svelte";
  import SystemMonitor from "../components/SystemMonitor.svelte";
  import Headlines from "../components/Headlines.svelte";
  import Pomodoro from "../components/Pomodoro.svelte";
  import Weather from "../components/Weather.svelte";

  interface FileInfo {
    name: string;
    is_dir: boolean;
    extension: string | null;
  }

  let files: FileInfo[] = [];

  function getIconComponent(file: FileInfo) {
    if (file.is_dir) return FolderIcon;
    switch (file.extension?.toLowerCase()) {
      case "txt":
      case "md":
        return FileTextIcon;
      case "js":
        return FileType2;
      case "jpg":
      case "png":
      case "gif":
        return FileImageIcon;
      case "mp4":
      case "avi":
      case "mov":
        return FileVideoIcon;
      case "mp3":
      case "wav":
        return FileMusicIcon;
      default:
        return FileIcon;
    }
  }

  onMount(async () => {
    try {
      files = await invoke("get_documents_files");
    } catch (error) {
      console.error("Failed to fetch files:", error);
    }
  });

  let name = "";
  let greetMsg = "";
  let time: any = "";
  let savedName = "";

  async function saveName() {
    try {
      await invoke("save_name", { name });
      alert("Name saved successfully!");
      loadName();
    } catch (error) {
      console.error("Failed to save name:", error);
      alert("Failed to save name.");
    }
  }

  async function loadName() {
    try {
      savedName = await invoke("get_name");
    } catch (error) {
      console.error("Failed to load name:", error);
      alert("Failed to load name.");
    }
  }

  onMount(loadName);

  onMount(async () => {
    // Initial time fetch
    time = await invoke("get_formatted_local_time");

    // Listen for time updates
    await listen("time-update", (event) => {
      time = event.payload;
    });
  });

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="container w-[calc(100%-40px)] max-w-4xl mt-6 mx-auto py-6">
  <div class="space-y-4 mb-10">
    <h1 class="text-4xl md:text-5xl tracking-tight font-light mx-auto pb-0 w-fit">Good Morning, {savedName}</h1>
    <p class="text-gray-300 text-center tracking-wide">Who am I speaking with?</p>

  </div>
  

  {#if savedName}
  <div class="grid grid-cols-5 gap-12">
    <p
    class="text-gray-200 col-span-3 bg-[#252525] bg-opacity-30   max-w-xl backdrop-blur-sm border border-[#252525] p-6 rounded-2xl text-sm leading-relaxed tracking-wide"
  >
    I apologize, but I'm not able to directly create or embed iframes or
    other web content. However, I can provide you with some information about
    what you're trying to do.
    <br />
    <br />
    Would you like me to elaborate on any of these options or generate more alternatives?
  </p>
  <div
  class="text-gray-200 col-span-2 max-w-xl rounded-2xl text-sm leading-relaxed tracking-wide"
>
  <h3 class="text-lg ">
    Recently Discussed
  </h3>
  <div class="w-full mt-3 p-4 rounded-xl h-12 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525]  flex items-center">
    Something about a calorie counter
  </div>
  <div class="w-full mt-3 p-4 rounded-xl h-12 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525]  flex  items-center">
    A way to bulk edit images
  </div>
</div>
  </div>
 
  {/if}

  <input
    placeholder="Enter your message"
    class="w-full fixed bottom-0 placeholder:text-[#999] left-1/2 -translate-x-1/2 z-[9999] max-w-xl bg-[#252525]  backdrop-blur text-left h-16 p-6 text-sm  border bg-opacity-95  border-[#252525] text-white rounded-tl-2xl rounded-tr-2xl "
  />

  {#if !savedName}
    <p class="text-gray-300">Who am I speaking with?</p>

    <input bind:value={name} placeholder="Enter a name" />
    <button on:click={saveName}>Save Name</button>
  {/if}

  <!-- <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form> -->

  <p>{greetMsg}</p>

  <SystemMonitor />
  <!-- <Weather /> -->
  <div class="grid grid-cols-2">
    <Headlines />
    <Pomodoro />
    <ul class="w-full max-w-md border p-6 rounded-md mx-auto">
      {#each files as file}
        <li class="flex mx-auto items-center justify-between">
          <svelte:component this={getIconComponent(file)} size={24} />
          {file.name}
        </li>
      {/each}
    </ul>
  </div>

  <div
    class="fixed bottom-4 text-sm right-4 border border-[#252525] bg-[#252525] bg-opacity-30 backdrop-blur-sm text-white p-4 rounded-xl"
  >
    {time}
  </div>

  <img src="/logo.svg" alt="Logo" class="fixed top-12 left-6 w-4" />
</div>

<style>
  .logo {
    filter: drop-shadow(0 0 1em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }
 


  @media (prefers-color-scheme: dark) {
    :root {
      color: #fff;
      background-color: #141414;
    }

    a:hover {
      color: #24c8db;
    }
  }
</style>

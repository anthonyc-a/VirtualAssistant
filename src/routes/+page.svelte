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

<div
  class="container mt-12 text-center border mx-auto p-6 rounded-xl space-y-3"
>
<SystemMonitor/>
  <div class="fixed bottom-4 right-4 bg-black text-white p-4 rounded-xl">
    {time}
  </div>

  <h1 class="text-5xl">Good evening, {savedName}</h1>

  <p class="text-gray-300">Who am I speaking with?</p>

  <!-- {#if !savedName} -->
  <input bind:value={name} placeholder="Enter a name" />
  <button on:click={saveName}>Save Name</button>
  <!-- {/if} -->

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>

  <ul class="w-full max-w-md border p-6 rounded-md mx-auto">
    {#each files as file}
      <li class="flex mx-auto items-center justify-between">
        <svelte:component this={getIconComponent(file)} size={24} />
        {file.name}
      </li>
    {/each}
  </ul>
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
    background-color: #f6f6f6;

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

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>

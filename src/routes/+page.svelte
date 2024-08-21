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
  import Headlines from "../components/Headlines.svelte";
  import Pomodoro from "../components/Pomodoro.svelte";
  import ImageGenerator from "../components/ImageGenerator.svelte";
  import Claude from "../components/Claude.svelte";
  import Hero from "../components/Hero.svelte";
  import Video from "../components/Video.svelte";

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

  let message: string = "";

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

<div class="container w-[calc(100%-40px)] max-w-4xl mt-0 mx-auto py-6 pt-4">
  <div class="space-y-2 mb-10">
    {#if savedName !== ""}
      <h1
        class="text-4xl md:text-4xl xl:text-4xl tracking-wide font-medium mx-auto pb-0 w-fit"
      >
        Good Evening, {savedName}
      </h1>
    {/if}
    {#if savedName === ""}
      <h1
        class="text-3xl text-center md:text-3xl max-w-2xl tracking-  font-[525] mx-auto pb-0 w-fit"
      >
        Solving problems and creating digital experiences with clear and
        functional design.
      </h1>
      <p
        class="pt-3 text-center leading-[1.65] tracking-wide font-light text-gray-300 max-w-2xl mx-auto"
      >
        A Designer and Full-Stack Developer with seven+ years experience working
        with small to medium-sized businesses, startups and individuals; helping
        to ensure brand growth with thoughtful design and incisive technical
        execution.
      </p>
    {/if}
  </div>

  {#if savedName}
    <div class="grid grid-cols-5 gap-12">
      <p
        class="text-gray-200 h-fit col-span-3 bg-[#252525] bg-opacity-30 max-w-xl backdrop-blur-sm border border-[#252525] p-6 rounded-xl text-sm leading-relaxed tracking-wide"
      >
        <Claude {message} />
      </p>
      <div
        class="text-gray-200 col-span-2 max-w-xl rounded-2xl text-sm leading-relaxed tracking-wide"
      >
        <h3 class="text-lg font-medium">Recently Discussed</h3>
        <div
          class="w-full mt-3 p-4 rounded-xl hover:border-white h-12 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] flex items-center"
        >
          Something about a calorie counter
        </div>
        <div
          class="w-full mt-3 p-4 rounded-xl hover:border-white h-12 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] flex items-center"
        >
          A way to bulk edit images
        </div>
        <div
          class="w-full mt-3 p-4 rounded-xl hover:border-white h-12 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] flex items-center"
        >
          A way to bulk edit images
        </div>
      </div>
    </div>
  {/if}


  <div
  class="w-3/4 md:w-full flex py-0  items-center  fixed bottom-4  left-1/2 -translate-x-1/2 z-[9999] overflow-hidden max-w-xl bg-[#252525] backdrop-blur text-left h-12 p-6 text- border bg-opacity-80 border-[#666] shadow text-white rounded-2xl"
  >
  <input
  placeholder="Enter your message"
  class="w-full tracking-wide h-full  bg-transparent !outline-none placeholder:text-[#999] text-white "
  bind:value={message}
/>
<button class="absolute pt-1 text-lg text-[#252525] font-medium top-1/2 -translate-y-1/2 right-2 w-8 h-8 rounded-lg bg-white rounded-mf">
  ↗
</button>
  </div>
 

  <!-- {#if !savedName}
    <p class="text-gray-300">Who am I speaking with?</p>

    <input bind:value={name} placeholder="Enter a name" />
    <button on:click={saveName}>Save Name</button>
  {/if} -->

  <!-- <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form> -->

  <p>{greetMsg}</p>

  <!-- <Weather /> -->
  {#if savedName}
    <div class="grid md:grid-cols-2 gap-6">
      <ImageGenerator />
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
  {/if}

  <Hero />

  <Video/>

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

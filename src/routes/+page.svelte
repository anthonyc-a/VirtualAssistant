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
  import { workItems } from "../data/workItems";
  import MessageBar from "../components/MessageBar.svelte";
  import HeroSwiper from "../components/Misc/Hero/HeroSwiper.svelte";
  import { heroItems } from "../data/Hero";
  import Heading from "../components/Heading.svelte";

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

<!-- <HeroSwiper items={heroItems} autoplayInterval={5000} /> -->

<div class="max-w-2xl mx-auto">
  <div class="space-y-2 mb-6">
    {#if savedName !== ""}
      <h1
        class="text-3xl md:text-3xl xl:text-4xl tracking-wide font-medium mx-auto pb-0 w-fit"
      >
        Good Evening, {savedName}
      </h1>

      <p class="w-fit mx-auto text-lg text-gray-300">What should we work on?</p>
    {/if}
    {#if savedName === ""}
      <Heading
        heading="Creating Digital Experiences with<br> Functional Design."
        subheading="I'm a Designer and Full-Stack Developer with over 7 years experience working with small to medium-sized businesses, startups and individuals."
        showBreak={true}
      />
    {/if}
  </div>

  <Hero />

  {#if savedName}
    <div class="grid grid-cols-5 gap-12 mt-12">
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

  <Video />

  {#if savedName === ""}
    <div class="mt-16 px-6 md:px-0 max-w-2xl mx-auto">
      <h3 class="text-2xl  text-center md:text-3xl font-medium">
        What I'm Working On
      </h3>
      <p class="mt-3  text-center text-muted-foreground leading-[1.5] text-[16px]">
        These are projects and companies that I've worked on/with. Some are
        well-established businesses.
      </p>
      <div class="w-full h-[1px] my-6 mb-4 bg-[--color-secondary]"></div>
      {#each workItems.slice(0, 2) as item}
        <a
          href={item.link}
          target="_blank"
          rel="noopener noreferrer"
          class="flex flex-col md:flex-row !text-foreground md:items-center transition-all border border-border md:border-none gap-4 mb-6 md:mb-0 hover:bg-accent/50 p-6 px-5 rounded-3xl"
          >
          <div
          class="w-16 h-16 mr-1.5 md:min-w-20 md:max-w-20 md:h-20 rounded-2xl md:rounded-2xl overflow-hidden r"
          >
            <img
              src={item.image}
              alt="Project Logo"
              class="w-full h-full object-cover"
            />
          </div>
          <div>
            <div class="flex flex-wrap items-center gap-3">
              <h4 class="w-full text-[22px] leading-8 md:w-fit">{item.title}</h4>
              {#each item.tags as tag}
                <span
                class="tag bg-accent px-3 p-1 !rounded-full whitespace-nowrap leading-[1.2]"
                  >{tag}</span
                >
              {/each}
            </div>
            <p class="mt-4 md:mt-2 text-sm traking-[0.1px] text-muted-foreground text- md:text-[16px]">
              {item.description}
            </p>
          </div>
        </a>
      {/each}
    </div>
    <div class="mt-16 px-6 md:px-0 max-w-2xl mx-auto">
      <h3 class="text-2xl font-medium">Previously worked on</h3>
      <div class="w-full h-[1px] bg-[--color-secondary]"></div>
      {#each workItems.slice(2, 6) as item}
        <a
          href={item.link}
          target="_blank"
          rel="noopener noreferrer"
          class="flex flex-col md:flex-row !text-foreground md:items-center transition-all border border-border md:border-none gap-4 mb-6 md:mb-0 hover:bg-accent/50 p-6 px-5 rounded-3xl"
        >
        <div
        class="w-16 h-16 mr-1.5 md:min-w-20 md:max-w-20 md:h-20 rounded-2xl md:rounded-2xl overflow-hidden r"
        >
          <img
            src={item.image}
            alt="Project Logo"
            class="w-full h-full object-cover"
          />
        </div>
        <div>
          <div class="flex flex-wrap items-center gap-3">
            <h4 class="w-full text-[22px] leading-8 md:w-fit">{item.title}</h4>
            {#each item.tags as tag}
              <span
              class="tag bg-accent px-3 p-1 !rounded-full whitespace-nowrap leading-[1.2]"
                >{tag}</span
              >
            {/each}
          </div>
          <p class="mt-4 md:mt-2 tracking-[0.1px] text-sm text-muted-foreground md:text-[16px]">
            {item.description}
          </p>
        </div>
        </a>
      {/each}
    </div>
  {/if}
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
</style>

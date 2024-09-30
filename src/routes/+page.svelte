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
  import WorkItem from "../components/WorkItem.svelte";

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
        heading="Creating Digital Experiences with Functional<br> Design and Expertise"
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
      <h3 class="text-2xl md:text-2xl font-medium">What I'm Working on Now</h3>
      <p
        class="mt-3 text-muted-foreground font-[320] tracking-[0.3px] leading-[1.5] text-[16px] md:text-lg"
      >
        These are projects and companies that I've worked on/with. Some are
        well-established businesses.
      </p>
      <div class="w-full h-[1px] my-6 mb-4 bg-[--color-secondary]"></div>
      <div class="md:space-y-5">
        {#each workItems.slice(0, 2) as item}
          <WorkItem {item} />
        {/each}
      </div>
    </div>
    <div class="mt-16 px-6 md:px-0 max-w-2xl mx-auto space-y-4">
      <h3 class="text-2xl md:text-2xl font-medium">Previously Worked on</h3>
      <div class="w-full h-[1px] bg-[--color-secondary]"></div>
      <div class="md:space-y-10">
        {#each workItems.slice(2, 6) as item}
          <WorkItem {item} />
        {/each}
        <a
          href="/work"
          class="w-fit px-3 p-1.5 flex mx-auto justify-center hover:bg-accent/30 items-center border border-border dark:border-accent rounded-2xl"
        >
          <div class="flex justify-center items-center h-full">
            <button class="text-foreground text-sm tracking-[0.1px]"> + More work </button>
          </div>
        </a>
      </div>
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
</style>

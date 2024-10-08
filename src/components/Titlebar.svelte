<script lang="ts">
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import SystemMonitor from "./SystemMonitor.svelte";
  import { Apple } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  let isMaximized = false;

  async function updateMaximizedStatus() {
    isMaximized = await appWindow.isMaximized();
  }

  function handleMinimize() {
    appWindow.minimize();
  }

  async function handleMaximize() {
    await appWindow.toggleMaximize();
    updateMaximizedStatus();
  }

  function handleClose() {
    appWindow.close();
  }

  onMount(() => {
    updateMaximizedStatus();
    // Listen for changes in window state
    appWindow.onResized(() => updateMaximizedStatus());
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

<div class="relative titlebar bg-foreground text-background font-medium rounded-md z-[999999]">
  <div class="titlebar-drag-region" data-tauri-drag-region></div>
  <div
    class="titlebar-text font-medium flex justify-between w-full items-center gap-2"
    data-tauri-drag-region
  >
    {#if time !== ""}
      <div class="flex items-center gap-2">
        <div class="w-1 h-1 rounded-full bg-green-400"></div>
        Virtual Assistant v1.0
      </div>
    {/if}
    {#if time === ""}
      <div class="flex items-center gap-2">
        <div class="w-[5px] h-[5px] rounded-full bg-green-500"></div>
        Portfolio Website
      </div>
    {/if}
    <div class="opacity-0">.</div>
  </div>
  {#if time !== ""}
    <div data-tauri-drag-region>
      <div class="flex text-[#252525] items-center text-xs gap-1.5">
        <div class="">
          {time}
        </div>

        <div class="mx-6 flex items-center gap-2">
          <Apple class="w-3.5" />
          1280 kcal
        </div>

        <SystemMonitor />
      </div>
    </div>
  {/if}

  <div class="titlebar-controls">
    {#if time !== ""}
      <button class="titlebar-button" on:click={handleMinimize}>
        <svg width="10" height="10" viewBox="0 0 10 10">
          <path d="M0 5h10v1h-10v-1z" />
        </svg>
      </button>
      <button class="titlebar-button" on:click={handleMaximize}>
        {#if isMaximized}
          <svg width="10" height="10" viewBox="0 0 10 10">
            <path
              d="M2 1v2h-2v6h6v-2h2v-6h-6zm-1 7v-4h4v4h-4zm2 -5v-1h4v4h-1v-3h-3z"
            />
          </svg>
        {:else}
          <svg width="10" height="10" viewBox="0 0 10 10">
            <path d="M0 0v10h10v-10h-10zm1 1h8v8h-8v-8z" />
          </svg>
        {/if}
      </button>
    {/if}
    <button
      class="titlebar- rounded-md scale-75 h-6 flex items-center justify-center w-6 mr-1 close-button"
      on:click={handleClose}
    >
      <svg width="10" height="10" viewBox="0 0 10 10">
        <path
          d="M0 0l10 10m-10 0l10 -10"
          stroke="currentColor"
          stroke-width="1"
        />
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 25px;
    user-select: none;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 9999;
  }
  .titlebar-drag-region {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    -webkit-app-region: drag;
  }
  .titlebar-text {
    font-size: 14px;
    font-weight: 500;
    margin-left: 10px;
    z-index: 1;
  }
  .titlebar-controls {
    display: flex;
    z-index: 1;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    background: transparent;
    border: none;
    color: #2c2c2c;
    cursor: pointer;
    -webkit-app-region: no-drag;
  }
  .titlebar-button:hover {
    background: #3e3e3e;
  }
  .close-button {
    background: #e81123;
    color: white;
  }
</style>

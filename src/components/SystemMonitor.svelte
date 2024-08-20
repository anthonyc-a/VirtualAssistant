<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let cpuInfo: any[] = [];
  let averageUsage: number = 0;
  let expanded: boolean = false;

  function expand() {
    expanded = !expanded;
  }

  async function fetchCpuInfo() {
    try {
      cpuInfo = await invoke("get_cpu_info");
      calculateAverageUsage();
    } catch (error) {
      console.error("Failed to get CPU info:", error);
    }
  }

  function calculateAverageUsage() {
    if (cpuInfo.length > 0) {
      const totalUsage = cpuInfo.reduce((sum, cpu) => sum + cpu.usage, 0);
      averageUsage = totalUsage / cpuInfo.length;
    } else {
      averageUsage = 0;
    }
  }

  onMount(() => {
    fetchCpuInfo();
    const interval = setInterval(fetchCpuInfo, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="fixed top-20 z-[9999] bg-[#252525] bg-opacity-30 backdrop-blur-sm p-4 rounded-xl border border-[#252525] text-white right-4 z-50">
  <h2 class="text-sm font-bold" on:click={expand} role="presentation">
    CPU: {averageUsage.toFixed(2)}%
    <div
    class="relative w-full h-1 mt-2 bg-gray-400 rounded-full overflow-hidden"
  >
    <div
      class="absolute transition-all inset-0 bg-green-500 rounded-full shadow-md shadow-[#934cf631]"
      style={`width: ${averageUsage}%; background-color: ${averageUsage > 75 ? "red" : "lightgreen"}`}
    ></div>
  </div>
  </h2>
</div>
{#if expanded}
  <div
    class="grid bg-[#252525] bg-opacity-20 backdrop-blur rounded-lg p-6 h-screen z-50 fixed bottom-0 overflow-scroll w-full right-0 grid-cols-2 gap-4 md:grid-cols-4 md:gap-8 mx-auto"
  >
    {#if cpuInfo.length > 0}
      {#each cpuInfo as cpu, i}
        <div class="border rounded-2xl">
          <div class="bg-card p-4 rounded-lg flex flex-col gap-2">
            <div class="font-bold text-lg">CPU Core {i + 1}</div>
            <div class="text-4xl font-bold">{cpu.usage.toFixed(2)}%</div>
          </div>
          <div class="bg-card p-4 rounded-lg flex flex-col gap-2">
            <div class="text-muted-foreground">Frequency</div>
            <div class="text-2xl">{cpu.frequency}GHz</div>
          </div>
          <div
            class="bg-card p-4 rounded-lg flex flex-col gap-2 col-span-2 md:col-span-1"
          >
            <div class="text-muted-foreground">Usage</div>
            <div
              class="relative w-full h-2 bg-gray-400 rounded-full overflow-hidden"
            >
              <div
                class="absolute transition-all inset-0 bg-blue-500 rounded-full"
                style={`width: ${cpu.usage}%; background-color: ${cpu.usage > 75 ? "red" : "chartreuse"}`}
              ></div>
            </div>
          </div>
        </div>
      {/each}
    {:else}
      <p>Loading CPU information...</p>
    {/if}
  </div>
{/if}

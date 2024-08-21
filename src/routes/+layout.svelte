<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import Header from "../components/Header.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import Cursor from "../components/Cursor.svelte";
  import SystemMonitor from "../components/SystemMonitor.svelte";
  import { Apple } from "lucide-svelte";
  import Titlebar from "../components/Titlebar.svelte";
  import Sidebar from "../components/Sidebar.svelte";

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

<Titlebar />

<div class="fixed flex flex-col items-center gap-3 top-[96px] left-3.5 z-[99999]">
  <div
  class=" flex overflow-hidden  justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
  <div class="home scale-[1.65] w-8 h-8"></div>
</div>
<div
class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
<div class="home scale-[1.65] w-8 h-8"></div>
</div>
<div
  class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
  <div class="home scale-[1.65] w-8 h-8"></div>
</div>
</div>



<Sidebar/>

<!-- <div
  class="fixed flex rounded-md bg-opacity-90 justify-between items-center gap-4 top-0 px-8 z-[9999] w-full text-xs text-center font-medium py-0.5 bg-[#fff] text-[#252525] mb-0"
>
  <div class="flex items-center text-xs gap-1.5">
    <Apple class="w-3.5" />
    1280 kcal

    <div class="ml-4">
      {time}
    </div>
  </div>
  <SystemMonitor />
</div> -->

<Cursor />

<!-- <div class="fixed bottom-4 left-4 w-8 h-8 rounded-md bg-[#ccfc6f]"></div> -->

<Header />
<slot />

<style>
  .home {
    image-rendering: pixelated;
    background-image: url("data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20viewBox%3D%220%200%2065%2065%22%3E%3Cpath%20d%3D%22M%2010%2064.5%20C%204.477%2064.5%200%2064.5%200%2064.5%20L%200%200%20C%200%200%204.477%200%2010%200%20L%2054.5%200%20C%2060.023%200%2064.5%200%2064.5%200%20L%2064.5%2064.5%20C%2064.5%2064.5%2060.023%2064.5%2054.5%2064.5%20Z%22%20fill%3D%22rgb(30%2C30%2C30)%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2027%2039.5%20L%2037.5%2039.5%20C%2038.605%2039.5%2039.5%2038.605%2039.5%2037.5%20L%2039.5%2030%20L%2032.25%2025%20L%2025%2030%20L%2025%2037.5%20C%2025%2038.605%2025.895%2039.5%2027%2039.5%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2030%2035.999%20C%2030%2034.894%2030.895%2033.999%2032%2033.999%20L%2032.499%2033.999%20C%2033.604%2033.999%2034.499%2034.894%2034.499%2035.999%20L%2034.499%2039.499%20L%2030%2039.499%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E");
  }
  .about {
    image-rendering: pixelated;
    background-image: url("data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20viewBox%3D%220%200%2065%2065%22%3E%3Cpath%20d%3D%22M%2010%2064.5%20C%204.477%2064.5%200%2064.5%200%2064.5%20L%200%200%20C%200%200%204.477%200%2010%200%20L%2054.5%200%20C%2060.023%200%2064.5%200%2064.5%200%20L%2064.5%2064.5%20C%2064.5%2064.5%2060.023%2064.5%2054.5%2064.5%20Z%22%20fill%3D%22rgb(30%2C30%2C30)%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2027%2039.5%20L%2037.5%2039.5%20C%2038.605%2039.5%2039.5%2038.605%2039.5%2037.5%20L%2039.5%2030%20L%2032.25%2025%20L%2025%2030%20L%2025%2037.5%20C%2025%2038.605%2025.895%2039.5%2027%2039.5%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2030%2035.999%20C%2030%2034.894%2030.895%2033.999%2032%2033.999%20L%2032.499%2033.999%20C%2033.604%2033.999%2034.499%2034.894%2034.499%2035.999%20L%2034.499%2039.499%20L%2030%2039.499%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E");
  }

  :global(*) {
    cursor: none !important;
  }
</style>

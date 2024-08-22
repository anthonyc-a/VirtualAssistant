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
  import Theme from "../components/Theme.svelte";

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

<Theme/>

<div class="fixed flex flex-col items-center gap-3 top-[96px] left-3.5 z-[99999]">
  <div
  class=" flex overflow-hidden  justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
  <div class="home scale-[1.65] w-8 h-8"></div>
</div>
<div
class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
<svg width="16" height="17" class="w-3.5" viewBox="0 0 16 17" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M7.80078 7.5C9.59578 7.5 11.0508 6.0449 11.0508 4.25C11.0508 2.4551 9.59578 1 7.80078 1C6.00588 1 4.55078 2.4551 4.55078 4.25C4.55078 6.0449 6.00588 7.5 7.80078 7.5Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M2.64801 15.5H12.953C14.0949 15.5 14.9745 14.5181 14.4413 13.5084C13.6568 12.0231 11.8685 10.25 7.80051 10.25C3.73251 10.25 1.94421 12.0231 1.15971 13.5084C0.626514 14.5181 1.50611 15.5 2.64801 15.5Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  </svg>
  
</div>
<div
  class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
<svg width="17" height="17" class="w-3.5" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M1 6C1 4.8954 1.8954 4 3 4H13.5C14.6046 4 15.5 4.8954 15.5 6V13.5C15.5 14.6046 14.6046 15.5 13.5 15.5H3C1.8954 15.5 1 14.6046 1 13.5V6Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M5 3.75V3C5 1.8954 5.8954 1 7 1H9.5C10.6046 1 11.5 1.8954 11.5 3V3.75" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M1.25 9.5H15.25" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M5 8V10.5" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M11.5 8V10.5" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  </svg>
  
</div>
<div
  class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>
<svg width="16" height="15" class="w-3.5" viewBox="0 0 16 15" fill="none" xmlns="http://www.w3.org/2000/svg" id="svg1044175782_710"><path d="M0.601074 1C0.601074 0.44772 1.04879 0 1.60107 0H5.85107C6.95567 0 7.85107 0.89543 7.85107 2V14.5L7.02267 13.6716C6.27247 12.9214 5.25508 12.5 4.19422 12.5H1.60107C1.04879 12.5 0.601074 12.0523 0.601074 11.5V1Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path><path d="M15.1011 1C15.1011 0.44772 14.6534 0 14.1011 0H9.85107C8.74647 0 7.85107 0.89543 7.85107 2V14.5L8.67947 13.6716C9.42967 12.9214 10.4471 12.5 11.508 12.5H14.1011C14.6534 12.5 15.1011 12.0523 15.1011 11.5V1Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path></svg>
  
</div>
<div
  class=" flex overflow-hidden justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 w-9 h-9"
>

<svg width="17" height="17" class="w-3.5" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M8.35156 15.5C12.3556 15.5 15.6016 12.2541 15.6016 8.25C15.6016 4.2459 12.3556 1 8.35156 1C4.34746 1 1.10156 4.2459 1.10156 8.25C1.10156 12.2541 4.34746 15.5 8.35156 15.5Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M11.6016 8.25C11.6016 12.75 9.59416 15.5 8.35156 15.5C7.10896 15.5 5.10156 12.75 5.10156 8.25C5.10156 3.75 7.10896 1 8.35156 1C9.59416 1 11.6016 3.75 11.6016 8.25Z" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M1.35156 8.25H8.35156H15.3516" stroke="#CBCBCB" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
  </svg>
  
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

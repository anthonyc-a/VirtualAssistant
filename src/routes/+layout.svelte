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
  import MessageBar from "../components/MessageBar.svelte";
  import Navigation from "../components/Navigation.svelte";

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
<MessageBar />
<Theme />
<Sidebar />
<Navigation />
<Header />

<img
  src="/logo.svg"
  alt="Logo"
  class="fixed top-12 left-6 w-4 z-[99999] invert dark:invert-0"
/>

<div class="container w-[calc(100%-40px)] max-w-3xl mt-0 mx-auto py-2">
  <slot />
</div>

<style>
  .home {
    image-rendering: pixelated;
    background-image: url("data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20viewBox%3D%220%200%2065%2065%22%3E%3Cpath%20d%3D%22M%2010%2064.5%20C%204.477%2064.5%200%2064.5%200%2064.5%20L%200%200%20C%200%200%204.477%200%2010%200%20L%2054.5%200%20C%2060.023%200%2064.5%200%2064.5%200%20L%2064.5%2064.5%20C%2064.5%2064.5%2060.023%2064.5%2054.5%2064.5%20Z%22%20fill%3D%22rgb(30%2C30%2C30)%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2027%2039.5%20L%2037.5%2039.5%20C%2038.605%2039.5%2039.5%2038.605%2039.5%2037.5%20L%2039.5%2030%20L%2032.25%2025%20L%2025%2030%20L%2025%2037.5%20C%2025%2038.605%2025.895%2039.5%2027%2039.5%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2030%2035.999%20C%2030%2034.894%2030.895%2033.999%2032%2033.999%20L%2032.499%2033.999%20C%2033.604%2033.999%2034.499%2034.894%2034.499%2035.999%20L%2034.499%2039.499%20L%2030%2039.499%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E");
  }
  .about {
    image-rendering: pixelated;
    background-image: url("data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20viewBox%3D%220%200%2065%2065%22%3E%3Cpath%20d%3D%22M%2010%2064.5%20C%204.477%2064.5%200%2064.5%200%2064.5%20L%200%200%20C%200%200%204.477%200%2010%200%20L%2054.5%200%20C%2060.023%200%2064.5%200%2064.5%200%20L%2064.5%2064.5%20C%2064.5%2064.5%2060.023%2064.5%2054.5%2064.5%20Z%22%20fill%3D%22rgb(30%2C30%2C30)%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2027%2039.5%20L%2037.5%2039.5%20C%2038.605%2039.5%2039.5%2038.605%2039.5%2037.5%20L%2039.5%2030%20L%2032.25%2025%20L%2025%2030%20L%2025%2037.5%20C%2025%2038.605%2025.895%2039.5%2027%2039.5%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M%2030%2035.999%20C%2030%2034.894%2030.895%2033.999%2032%2033.999%20L%2032.499%2033.999%20C%2033.604%2033.999%2034.499%2034.894%2034.499%2035.999%20L%2034.499%2039.499%20L%2030%2039.499%20Z%22%20fill%3D%22transparent%22%20stroke-width%3D%221.5%22%20stroke%3D%22%23cbcbcb%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%20stroke-dasharray%3D%22%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E");
  }

  /* :global(*) { */
  /* cursor: none !important; */
  /* } */
</style>

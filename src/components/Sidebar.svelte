<script>
  import { onMount } from "svelte";
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import CaseStudy from "./CaseStudy.svelte";

  let showSidebar = false;
  let hoverAreaWidth;

  let hovered = false;

  const sidebarPosition = tweened(-300, {
    duration: 500,
    easing: cubicOut,
  });

  function toggleSidebar(show) {
    showSidebar = show;
    sidebarPosition.set(show ? 0 : 300);
  }

  function updateHoverAreaWidth() {
    hoverAreaWidth = window.innerWidth / 3;
  }

  onMount(() => {
    updateHoverAreaWidth();
    window.addEventListener("resize", updateHoverAreaWidth);

    return () => {
      window.removeEventListener("resize", updateHoverAreaWidth);
    };
  });
</script>

<div
  class="fixed top-0 right-0 w-1/5 h-1/5 h-full z-50"
  on:mouseenter={() => toggleSidebar(true)}
>
  <!-- Hoverable area -->
</div>

<div class="fixed top-12 right-5 z-40 text-[#999]">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="1em"
    height="1em"
    fill="currentColor"
    viewBox="0 0 256 256"
    class="w-5 h-5 rotate-180"
  >
    <path
      d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM40,56H80V200H40ZM216,200H96V56H216V200Z"
    ></path>
  </svg>
</div>

{#if hovered}
  <div
    class="fixed modal bg-[#252525] bg-opacity-30 p-12 backdrop-blur border-[#252525] z-[9995] top-1/2 right-80 -translate-y-1/2 max-w-sm lg:max-w-5xl w-fit h-[70vh] overflow-scroll border rounded-2xl"
    on:mouseenter={() => toggleSidebar(true)}
  >
    <CaseStudy />
  </div>
{/if}

{#if showSidebar}
  <div
    class="fixed inset-0 bg-black backdrop-blur transition-all bg-opacity-30 z-40"
    on:mouseenter={() => {
      toggleSidebar(false), (hovered = false);
    }}
  ></div>
  <div
    class="fixed top-16 pt-2 rounded-tl-2xl rounded-bl-2xl right-0 origin-top-right h-[calc(100vh-128px)] w-64 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] z-50 shadow-lg"
    style="transform: translateX({$sidebarPosition}px);"
    on:mouseenter={() => toggleSidebar(true)}
  >
    <h2 class="text-lg tracking-wide font-medium p-4">Clients</h2>
    <div
      class="modules w-full grid gap-2 px-2"
      on:click={() => (hovered = !hovered)}
    >
      <div
        class="w-full h-12 rounded-xl bg-[#CFFE70] border border-[#252525]"
      ></div>
      <div
        class="w-full h-12 rounded-xl bg-[#3A5054] border border-[#252525]"
      ></div>
      <div
        class="w-full h-12 rounded-xl bg-[#fff] border border-[#252525]"
      ></div>
      <div
        class="w-full h-12 rounded-xl bg-[#0B1B26] border border-[#252525]"
      ></div>
    </div>
    <h2 class="text-lg tracking-wide font-medium p-4">Personal</h2>
    <div class="w-full grid gap-2 px-2">
      <div
        class="w-full h-12 rounded-xl bg-[#f4f4f4] border border-[#252525]"
      ></div>
      <div
        class="w-full h-12 rounded-xl bg-[#fff] border border-[#252525]"
      ></div>
      <div
        class="w-full h-12 rounded-xl bg-[#000] border border-[#252525]"
      ></div>
    </div>
  </div>
{/if}

<style>
  .modules div:hover {
    height: 64px;
  }

  .modules div {
    transition: 0.35s ease-in-out;
  }
</style>

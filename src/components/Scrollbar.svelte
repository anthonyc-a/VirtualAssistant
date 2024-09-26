<script lang="ts">
  import { onMount } from "svelte";
  import { ArrowUp, Mail } from "lucide-svelte";

  let progress: number = 0;

  function updateProgress() {
    const scrollPosition = window.scrollY;
    const maxScroll =
      document.documentElement.scrollHeight - window.innerHeight;
    progress = (scrollPosition / maxScroll) * 100;
  }

  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  onMount(() => {
    window.addEventListener("scroll", updateProgress);
    return () => {
      window.removeEventListener("scroll", updateProgress);
    };
  });
</script>

<div
  class="relative hidden mb-1 lg:flex transition-all text-muted-foreground p-2 px-3 border border-border dark:border-muted bg-accent/80 backdrop-blur rounded-full items-center space-x-1.5"
  class:hidden={progress === 100}
  class:flex={progress !== 100}

  >
  <div
    class="relative w-0 max-w-2xl me-2 h-px bg-border"
    class:w-32={progress > 0}
    class:me-2={progress > 0}
    class:w-0={progress < 0}
    class:me-0={progress < 0}
  >
    <div
      class="h-full right-0 absolute top-0 bg-foreground"
      style="width: {progress}%"
    ></div>
  </div>
  <button
    on:click={scrollToTop}
    class=" rounded-full bg-background invert text-foreground border-border p-1 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
  >
    <ArrowUp strokeWidth={2.25} class="w-3 h-3" />
  </button>
</div>

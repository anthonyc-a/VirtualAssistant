<script lang="ts">
  import { onMount } from "svelte";
  import { Home, User, Briefcase, Book, Globe } from "lucide-svelte";

  let isExpanded = false;

  onMount(() => {
    const handleMouseMove = (e: any) => {
      const { clientX, clientY } = e;
      const { innerWidth, innerHeight } = window;

      // Check if the cursor is in the left quarter of the top half
      isExpanded = clientX < innerWidth / 4 && clientY < innerHeight / 2;
    };

    document.addEventListener("mousemove", handleMouseMove);
    return () => document.removeEventListener("mousemove", handleMouseMove);
  });

  const navItems = [
    { href: "/", icon: Home, isFirst: true },
    { href: "/about", icon: User },
    { href: "#", icon: Briefcase },
    { href: "#", icon: Book },
    { href: "#", icon: Globe },
  ];
</script>

<div
  class="fixed flex flex-col items-center gap-3 top-[96px] left-3.5 z-[99999]"
>
  {#each navItems as item, index}
    <a
      href={item.href}
      class="nav-item flex overflow-hidden justify-center items-center rounded-full !font-light text-foreground bg-opacity-30 w-9 h-9 transition-all duration-300 ease-in-out"
      class:expanded={true}
    >
      <!-- isExpanded || item.isFirst -->
      <svelte:component this={item.icon} size={18} strokeWidth={1} />
    </a>
  {/each}
</div>

<style>
  .nav-item {
    transform: translateY(0);
    opacity: 0;
    scale: 0;
  }

  .nav-item.expanded {
    transform: translateY(0);
    opacity: 1;
    scale: 1;
  }

  /* Always show the first item */
  .nav-item:first-child {
    opacity: 1;
    scale: 1;
  }
</style>

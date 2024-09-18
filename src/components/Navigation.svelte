<script lang="ts">
  import { onMount } from "svelte";
  import { Home, User, Briefcase, Book, Globe, Folder } from "lucide-svelte";
  import { page } from "$app/stores";
  import { haptic } from "../utils/haptic";

  let isExpanded = false;
  let currentPath = window.location.pathname;
  let isVisible = true;
  let lastScrollY = 0;

  onMount(() => {
    const handlePathChange = () => {
      currentPath = window.location.pathname;
    };

    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      isVisible = currentScrollY < lastScrollY || currentScrollY < 50;
      lastScrollY = currentScrollY;
    };

    window.addEventListener("popstate", handlePathChange);
    window.addEventListener("scroll", handleScroll);

    return () => {
      window.removeEventListener("popstate", handlePathChange);
      window.removeEventListener("scroll", handleScroll);
    };
  });

  onMount(() => {
    const handleMouseMove = (e: any) => {
      const { clientX, clientY } = e;
      const { innerWidth, innerHeight } = window;

      isExpanded = clientX < innerWidth / 4 && clientY < innerHeight / 2;
    };

    document.addEventListener("mousemove", handleMouseMove);
    return () => document.removeEventListener("mousemove", handleMouseMove);
  });

  const navItems = [
    { href: "/", icon: Home, isFirst: true },
    { href: "/about", icon: User },
    { href: "/work", icon: Briefcase },
    { href: "/gallery", icon: Folder },
    { href: "/blog", icon: Book },
    { href: "https://portfolio-2024-ten-umber.vercel.app/", icon: Globe },
  ];
</script>

<div
  class="fixed flex p-1.5 px-3 border border-border rounded-full bg-accent bg-opacity-90 backdrop-blur flex-row items-center gap-3 left-1/2 -translate-x-1/2 z-[99999] transition-all duration-300"
  class:bottom-4={isVisible}
  class:-bottom-20={!isVisible}
  use:haptic={100}
>
  {#each navItems as item, index}
    <a
      href={item.href}
      class="nav-item flex overflow-hidden justify-center border border-border bg-background bg-opacity-90 backdrop-blur items-center rounded-full !font-light w-11 h-11"
      class:expanded={true}
      class:bg-foreground={item.href === $page.url.pathname}
      class:text-background={item.href === $page.url.pathname}
      class:border-none={item.href === $page.url.pathname}
      class:bg-opacity-100={item.href === $page.url.pathname}
    >
      <svelte:component this={item.icon} size={17} strokeWidth={2} />
    </a>
  {/each}
</div>
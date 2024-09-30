<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from 'svelte/transition';
  import {
    Home,
    User,
    Briefcase,
    Book,
    Globe,
    Folder,
    Mail,
  } from "lucide-svelte";
  import { page } from "$app/stores";
  import { haptic } from "../utils/haptic";

  let isExpanded = false;
  let currentPath = window.location.pathname;
  let isVisible = true;
  let lastScrollY = 0;
  let isDesktop = window.matchMedia("(min-width: 1024px)").matches;

  onMount(() => {
    const handlePathChange = () => {
      currentPath = window.location.pathname;
    };

    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      isVisible = currentScrollY < lastScrollY || currentScrollY < 50;
      lastScrollY = currentScrollY;
    };

    const handleResize = () => {
      isDesktop = window.matchMedia("(min-width: 1024px)").matches;
    };

    window.addEventListener("popstate", handlePathChange);
    window.addEventListener("scroll", handleScroll);
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("popstate", handlePathChange);
      window.removeEventListener("scroll", handleScroll);
      window.removeEventListener("resize", handleResize);
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
    { href: "/", icon: Home, tooltip: "Home", isFirst: true },
    { href: "/about", icon: User, tooltip: "About" },
    { href: "/work", icon: Briefcase, tooltip: "Work" },
    { href: "/gallery", icon: Folder, tooltip: "Gallery" },
    { href: "/blog", icon: Book, tooltip: "Blog" },
    { href: "https://portfolio-lime-nine-55.vercel.app/", icon: Globe, tooltip: "Portfolio '23" },
  ];

  // Add the contact link conditionally based on screen size
  $: if (isDesktop) {
    navItems.push({ href: "/contact", icon: Mail, tooltip: "Contact" });
  }
</script>

<div
  class="fixed fade-up z-[99999999] flex p-1.5 px-2 border border-border rounded-full bg-accent bg-opacity-90 backdrop-blur flex-row items-center gap-3 left-1/2 -translate-x-1/2 md:left-1/2 transition-all duration-300"
  class:bottom-3={isVisible}
  class:-bottom-20={!isVisible}
  use:haptic={100}
>
  {#each navItems as item, index}
    {#if index === 3 || (index > 3 && (index - 3) % 2 === 0)}
      <div class="h-8 w-px bg-border"></div>
    {/if}
    <a
      href={item.href}
      class="nav-item flex transition-colors hover:bg-foreground hover:text-background justify-center border border-border bg-background bg-opacity-90 backdrop-blur items-center rounded-full !font-light w-11 h-11 relative group"
      class:expanded={true}
      class:bg-foreground={item.href === $page.url.pathname}
      class:text-background={item.href === $page.url.pathname}
      class:border-none={item.href === $page.url.pathname}
      class:bg-opacity-100={item.href === $page.url.pathname}
      title={item.tooltip}
    >
      <svelte:component this={item.icon} size={17} strokeWidth={2} />
      {#if $page.url.pathname !== item.href}
        <div class="tooltip-container">
          {#key item.tooltip}
            <span 
              class="tooltip absolute font-medium -top-8 left-1/2 transform -translate-x-1/2 bg-foreground text-background text-xs py-1 px-2 rounded-lg pointer-events-none whitespace-nowrap"
              transition:fade={{ duration: 200, y: 10 }}
            >
              {item.tooltip}
            </span>
          {/key}
        </div>
      {/if}
    </a>
  {/each}
</div>

<style>
  .fade-up {
    opacity: 0;
    transform: translateX(-50%) translateY(20px);
    animation: fadeUp 0.75s ease-in-out 2.25s forwards;
  }

  @keyframes fadeUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .tooltip-container {
    position: absolute;
    top: -8px;
    left: 50%;
    transform: translateX(-50%);
    opacity: 0;
    transition: opacity 0.2s;
  }

  .nav-item:hover .tooltip-container {
    opacity: 1;
  }
</style>
<script>
  import { page } from "$app/stores";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";

  const pages = [
    { url: "/", title: "Home" },
    { url: "/about", title: "About" },
    { url: "/work", title: "Work" },
    { url: "/gallery", title: "Gallery" },
    { url: "/blog", title: "Blog" },
  ];

  $: currentPageIndex = pages.findIndex((p) => p.url === $page.url.pathname);
  $: prevPage = currentPageIndex > 0 ? pages[currentPageIndex - 1] : null;
  $: nextPage =
    currentPageIndex < pages.length - 1 ? pages[currentPageIndex + 1] : null;
</script>

<nav
  class="md:w-full w-[calc(100%-64px)] border-t border-border dark:border-accent max-w-3xl mx-auto mt-8 md:mt-16 bg-background text-muted-foreground py-8 flex justify-between items-center"
>
  {#if prevPage}
    <a
      href={prevPage.url}
      class="flex items-center bg-foreground text-background p-1 px-2 pl-1 rounded-full space-x-2 hover:underline transition-colors duration-200"
    >
      <ChevronLeft strokeWidth={1.75} size={16} />
      <span class="uppercase text-xs font-medium">{prevPage.title}</span>
    </a>
  {:else}
    <div></div>
  {/if}
  {#if nextPage}
    <a
      href={nextPage.url}
      class="flex items-center bg-foreground text-background p-1 px-2 pr-1 rounded-full space-x-2 hover:underline transition-colors duration-200"
    >
      <span class="uppercase text-xs font-medium">{nextPage.title}</span>
      <ChevronRight strokeWidth={1.75} size={16} />
    </a>
  {:else}
    <div></div>
  {/if}
</nav>

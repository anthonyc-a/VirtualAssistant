<script>
    import { page } from '$app/stores';
  
    const pages = [
      { url: '/', title: 'Home' },
      { url: '/about', title: 'About' },
      { url: '/work', title: 'Work' },
      { url: '/gallery', title: 'Gallery' },
      { url: '/blog', title: 'Blog' }
    ];
  
    $: currentPageIndex = pages.findIndex(p => p.url === $page.url.pathname);
    $: prevPage = currentPageIndex > 0 ? pages[currentPageIndex - 1] : null;
    $: nextPage = currentPageIndex < pages.length - 1 ? pages[currentPageIndex + 1] : null;
  </script>
  
  <nav class="w-full mt-8 bg-background text-muted-foreground py-4 px-6 flex justify-between items-center">
    {#if prevPage}
      <a href={prevPage.url} class="flex items-center space-x-2 hover:underline transition-colors duration-200">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        <span class="uppercase text-sm font-medium">{prevPage.title}</span>
      </a>
    {:else}
      <div></div>
    {/if}
  
    {#if nextPage}
      <a href={nextPage.url} class="flex items-center space-x-2 hover:underline transition-colors duration-200">
        <span class="uppercase text-sm font-medium">{nextPage.title}</span>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
      </a>
    {:else}
      <div></div>
    {/if}
  </nav>
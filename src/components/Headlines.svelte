<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
  
    let h2Contents: Array<{ content: string }> = [];
    let visibleContents: Array<{ content: string }> = [];
    let loading = false;
    let error = '';
    let url = 'https://bbc.com'; // Default URL
    let expanded = false;
    let visibleCount = 6;
    const initialCount = 6;
    const incrementCount = 6;
  
    async function fetchH2Contents() {
      try {
        loading = true;
        error = '';
        h2Contents = await invoke('scrape_h2_content', { url });
        visibleContents = h2Contents.slice(0, visibleCount);
        expanded = true;
      } catch (e) {
        error = 'Failed to fetch H2 contents: ' + e;
      } finally {
        loading = false;
      }
    }
  
    function showMore() {
      visibleCount = Math.min(visibleCount + incrementCount, h2Contents.length);
      updateVisibleContents();
    }
  
    function showLess() {
      visibleCount = Math.max(visibleCount - incrementCount, initialCount);
      updateVisibleContents();
    }
  
    function updateVisibleContents() {
      visibleContents = h2Contents.slice(0, visibleCount);
    }
  
    $: hasMore = visibleCount < h2Contents.length;
    $: hasLess = visibleCount > initialCount;
  
    onMount(() => {
      // If you want to load content on mount, uncomment the next line
      // fetchH2Contents();
    });
  </script>
  
  <div class="container max-w-3xl mx-auto px-4 py-8">
    <div class="mb-6">
      <input 
        bind:value={url} 
        placeholder="Enter URL to scrape" 
        class="w-full p-2 border border-[#252525] text-white rounded-xl bg-[#252525] bg-opacity-30 backdrop-blur-sm mb-2"
      />
      <button 
        on:click={fetchH2Contents}
        class="w-full bg-[#fff] text-sm hover:bg-blue-600 text-[#252525] font-bold py-2 px-4 rounded-[10px] mt-2 transition duration-300"
      >
        Scrape headlines
      </button>
    </div>
    
    {#if expanded}
      {#if loading}
        <p class="text-center text-gray-600">Loading H2 contents...</p>
      {:else if error}
        <p class="text-center text-red-500">{error}</p>
      {:else}
        <div class="grid grid-cols-1 gap-4">
          {#each visibleContents as content}
            <div class="border text-left border-[#252525] text-white rounded-xl bg-[#252525] bg-opacity-30 backdrop-blur-sm  p-4 hover:shadow-lg transition duration-300">
              <p class="text-white text-sm">{content.content}</p>
            </div>
          {/each}
        </div>
        
        <div class="mt-6 text-center space-x-4">
          {#if hasLess}
            <button 
              on:click={showLess}
              class="bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-2 px-4 rounded-md transition duration-300"
            >
              Show Less
            </button>
          {/if}
          {#if hasMore}
            <button 
              on:click={showMore}
              class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-md transition duration-300"
            >
              Show More
            </button>
          {/if}
        </div>
      {/if}
    {/if}
  </div>
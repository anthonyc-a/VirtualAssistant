<script>
    import { onMount } from 'svelte';
    import { tweened } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';
  
    let showSidebar = false;
    let hoverAreaWidth;
    
    const sidebarPosition = tweened(-300, {
      duration: 300,
      easing: cubicOut
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
      window.addEventListener('resize', updateHoverAreaWidth);
  
      return () => {
        window.removeEventListener('resize', updateHoverAreaWidth);
      };
    });
  </script>
  
  <div 
    class="fixed top-0 right-0 w-1/5 h-1/5  h-full z-50" 
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
  
  {#if showSidebar}
    <div 
      class="fixed  inset-0 bg-black backdrop-blur transition-all bg-opacity-30 z-40"
      on:mouseenter={() => toggleSidebar(false)}
    ></div>
    <div 
      class="fixed top-16 pt-2 rounded-tl-2xl rounded-bl-2xl right-0 origin-top-right h-[calc(100vh-128px)] w-64 bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] z-50 shadow-lg"
      style="transform: translateX({$sidebarPosition}px);"
    on:mouseenter={() => toggleSidebar(true)}
    >
      <h2 class="text-xl tracking-wide font-medium p-4">Projects</h2>
    </div>
  {/if}

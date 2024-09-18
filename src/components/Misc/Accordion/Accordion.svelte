<script lang="ts">
  import { slide } from "svelte/transition";

  export let items: {
    title: string;
    content: string;
  }[] = [];

  let activeIndex: number | null = null;

  function toggleItem(index: number) {
    activeIndex = activeIndex === index ? null : index;
  }
</script>

<div class="space-y-2">
  {#each items as item, index}
    <div class="border border-gray-200 rounded-lg overflow-hidden">
      <button
        on:click={() => toggleItem(index)}
        class="w-full px-4 py-3 text-left bg-white hover:bg-gray-50 focus:outline-none focus:bg-gray-50 transition-colors duration-200 flex justify-between items-center"
      >
        <span class="font-semibold text-gray-700">{item.title}</span>
        <svg
          class="w-5 h-5 text-gray-500 transform transition-transform duration-200 {activeIndex ===
          index
            ? 'rotate-180'
            : ''}"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 9l-7 7-7-7"
          />
        </svg>
      </button>
      {#if activeIndex === index}
        <div transition:slide={{ duration: 300 }} class="px-4 py-3 bg-gray-50">
          <p class="text-gray-600">{item.content}</p>
        </div>
      {/if}
    </div>
  {/each}
</div>

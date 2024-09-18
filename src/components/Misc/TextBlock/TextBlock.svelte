<script lang="ts">
    export let sections: {
      title: string;
      description: string;
      bulletPoints?: string[];
      content: string; // This will be the HTML content for the right side (image, UI, etc.)
    }[] = [];
  </script>
  
  <div class="space-y-4">
    {#each sections as section}
      <div class="bg-gray-900 rounded-lg overflow-hidden shadow-lg">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-6 space-y-4">
            <h2 class="text-2xl font-bold text-white">{section.title}</h2>
            <p class="text-gray-300">{section.description}</p>
            {#if section.bulletPoints}
              <ul class="space-y-2">
                {#each section.bulletPoints as point}
                  <li class="flex items-start">
                    <svg class="w-5 h-5 text-green-500 mr-2 mt-1 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    <span class="text-gray-300">{point}</span>
                  </li>
                {/each}
              </ul>
            {/if}
          </div>
          <div class="bg-gray-800 p-4 flex items-center justify-center">
            {@html section.content}
          </div>
        </div>
      </div>
    {/each}
  </div>
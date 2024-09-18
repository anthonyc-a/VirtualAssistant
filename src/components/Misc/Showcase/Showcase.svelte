<script lang="ts">
    export let title: string;
    export let description: string;
    export let images: { src: string; alt: string }[];
    export let buttonText: string = "Full project";
    export let onButtonClick: () => void = () => {};
  </script>
  
  <div class="bg-white rounded-3xl p-8 shadow-lg">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <div class="space-y-6">
        <h2 class="text-4xl font-bold text-gray-800">{title}</h2>
        <p class="text-gray-600">{description}</p>
        <button
          on:click={onButtonClick}
          class="bg-blue-100 text-blue-800 px-6 py-2 rounded-full font-semibold hover:bg-blue-200 transition-colors"
        >
          {buttonText}
        </button>
      </div>
      <div class="grid grid-cols-2 gap-4">
        {#each images as image, index}
          <div class={`rounded-lg overflow-hidden ${index === 0 ? 'col-span-2' : ''}`}>
            <img src={image.src} alt={image.alt} class="w-full h-full object-cover" />
          </div>
        {/each}
      </div>
    </div>
  </div>
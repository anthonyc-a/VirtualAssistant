<script lang="ts">
    export let title: string = "Enhance your team's productivity with Acme";
    export let subtitle: string = "Write in threads, focus, and collaborate without video calls.";
    export let primaryButtonText: string = "Get started";
    export let secondaryButtonText: string = "See more";
    export let onPrimaryClick: () => void = () => {};
    export let onSecondaryClick: () => void = () => {};
  </script>
  
  <header class="bg-white rounded-md text-black py-6 px-4 sm:px-6 lg:px-8">
    <div class="max-w-7xl mx-auto flex flex-col sm:flex-row justify-between items-start sm:items-center">
      <div class="mb-4 sm:mb-0">
        <h1 class="text-2xl sm:text-3xl font-bold leading-tight">{title}</h1>
        <p class="mt-1 text-gray-400 text-sm sm:text-base">{subtitle}</p>
      </div>
      <div class="flex space-x-4">
        <button
          on:click={onPrimaryClick}
          class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-full text-sm transition duration-300"
        >
          {primaryButtonText}
        </button>
        <button
          on:click={onSecondaryClick}
          class="bg-gray-800 hover:bg-gray-700 text-white font-semibold py-2 px-4 rounded-full text-sm transition duration-300"
        >
          {secondaryButtonText}
        </button>
      </div>
    </div>
  </header>
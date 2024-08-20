<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { writable } from 'svelte/store';
  
    let prompt = "";
    let response: any = null;
    let isLoading = false;
    let error = "";
    let selectedImage: string | null = null;
    
    // Create a writable store for imageUrls
    const imageUrlsStore = writable<any[]>([]);
  
    // Function to save imageUrls to local storage
    function saveImageUrls(urls: any[]) {
      localStorage.setItem('imageUrls', JSON.stringify(urls));
    }
  
    // Function to load imageUrls from local storage
    function loadImageUrls(): any[] {
      const saved = localStorage.getItem('imageUrls');
      return saved ? JSON.parse(saved) : [];
    }
  
    function openImagePreview(imageUrl: string) {
      selectedImage = imageUrl;
    }
  
    function closeImagePreview() {
      selectedImage = null;
    }
  
    onMount(async () => {
      const unlisten = await listen("image_generated", (event: any) => {
        response = event.payload;
        // Append new image to the existing list
        if (event.payload.body.images) {
          imageUrlsStore.update(urls => {
            const newUrls = [...urls, { url: event.payload.body.images }];
            saveImageUrls(newUrls);
            return newUrls;
          });
        }
        isLoading = false;
      });
  
      // Load initial image URLs from local storage
      const initialUrls = loadImageUrls();
      imageUrlsStore.set(initialUrls);
  
      // If local storage is empty, try to get initial URLs from the backend
      if (initialUrls.length === 0) {
        try {
          const backendUrls = await invoke("get_image_urls");
          imageUrlsStore.set(backendUrls);
          saveImageUrls(backendUrls);
        } catch (e) {
          console.error("Error fetching initial image URLs:", e);
        }
      }
  
      return () => {
        unlisten();
      };
    });
  
    async function handleSubmit() {
      if (!prompt.trim()) {
        error = "Please enter a prompt";
        return;
      }
  
      error = "";
      response = null;
      isLoading = true;
      try {
        await invoke("generate_image", { prompt });
      } catch (e) {
        console.error("Error generating image:", e);
        error = `Failed to generate image: ${e}`;
        isLoading = false;
      }
    }
  </script>
  
  <div class="image-generator mt-8">
    <form on:submit|preventDefault={handleSubmit} class="mb-4">
      <input
        type="text"
        bind:value={prompt}
        placeholder="Enter your image prompt"
        class="w-full p-2 border border-[#252525] text-white rounded-xl bg-[#252525] bg-opacity-30 backdrop-blur-sm mb-4"
      />
      <button
        type="submit"
        disabled={isLoading}
        class="w-full px-4 py-2 rounded-full text-sm font-medium bg-white text-black  hover:bg-blue-600 disabled:bg-gray-400"
      >
        {isLoading ? "Generating..." : "Generate Image"}
      </button>
    </form>
  
    {#if error}
      <p class="text-red-500 mb-4">{error}</p>
    {/if}
  
    <div class="mt-8">
      <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
        {#each $imageUrlsStore as url}
          <div class="aspect-w-1 border rounded-md overflow-hidden border-[#252525] aspect-h-1 cursor-pointer" on:click={() => openImagePreview(url.url[0].url)}>
            <img
              src={url.url[0].url}
              alt="Generated"
              class="object-cover rounded shadow-lg"
            />
          </div>
        {/each}
      </div>
    </div>
  </div>
  
  {#if selectedImage}
    <div class="fixed top-0 left-0 w-full h-full z-[9999] inset-0 bg-black bg-opacity-50 backdrop-blur-sm flex justify-center items-center z-50" on:click={closeImagePreview}>
      <div class="max-w-3xl max-h-[90vh] bg-white rounded-lg" on:click|stopPropagation>
        <img src={selectedImage} alt="Preview" class="max-w-full max-h-full object-contain" />
      </div>
    </div>
  {/if}
  
  <style>
    /* Add any additional styles here if needed */
  </style>
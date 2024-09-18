<script>
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  export let items = [];
  export let autoplayInterval = 5000;

  let currentIndex = 0;
  let startX = 0;
  let isDragging = false;
  let scrollY = 0;

  $: currentItem = items[currentIndex];

  function nextSlide() {
    currentIndex = (currentIndex + 1) % items.length;
  }

  function prevSlide() {
    currentIndex = (currentIndex - 1 + items.length) % items.length;
  }

  function goToSlide(index) {
    currentIndex = index;
  }

  function handleDragStart(e) {
    startX = e.type === "touchstart" ? e.touches[0].clientX : e.clientX;
    isDragging = true;
  }

  function handleDragEnd(e) {
    if (!isDragging) return;
    isDragging = false;
    const endX =
      e.type === "touchend" ? e.changedTouches[0].clientX : e.clientX;
    const diffX = startX - endX;
    if (Math.abs(diffX) > 50) {
      if (diffX > 0) {
        nextSlide();
      } else {
        prevSlide();
      }
    }
  }

  onMount(() => {
    const interval = setInterval(nextSlide, autoplayInterval);
    return () => clearInterval(interval);
  });

  $: yOffset = scrollY * 0.3; // Adjust this value to control the parallax intensity
</script>

<svelte:window bind:scrollY />

<div
  class="relative w-full mb-6 h-[400px] overflow-hidden cursor-grab active:cursor-grabbing"
  on:mousedown={handleDragStart}
  on:touchstart={handleDragStart}
  on:mouseup={handleDragEnd}
  on:mouseleave={handleDragEnd}
  on:touchend={handleDragEnd}
  role="presentation"
>
  {#key currentIndex}
    <div class="absolute inset-0" transition:fade={{ duration: 300 }}>
      <div class="absolute inset-0 overflow-hidden">
        <img
          src={currentItem.image}
          alt={currentItem.title}
          class="w-full scale-125 h-full object-cover"
          style="transform: translateY(-{yOffset / 3}px) scale(1.25);"
        />
      </div>
      <div
        class="absolute inset-0 bg-black bg-opacity-50 flex flex-col justify-end p-6"
      >
        <h2 class="text-3xl font-bold text-white mb-2">{currentItem.title}</h2>
        <p class="text-white mb-4">{currentItem.description}</p>
        <div class="flex space-x-2">
          {#each currentItem.tags as tag}
            <span class="bg-blue-500 text-white px-2 py-1 rounded text-sm"
              >{tag}</span
            >
          {/each}
        </div>
      </div>
    </div>
  {/key}

  <button
    class="absolute top-1/2 left-4 transform -translate-y-1/2 bg-white bg-opacity-50 hover:bg-opacity-75 text-black p-2 rounded-full"
    on:click={prevSlide}
  >
    &lt;
  </button>
  <button
    class="absolute top-1/2 right-4 transform -translate-y-1/2 bg-white bg-opacity-50 hover:bg-opacity-75 text-black p-2 rounded-full"
    on:click={nextSlide}
  >
    &gt;
  </button>

  <div
    class="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2"
  >
    {#each items as _, i}
      <button
        class="w-3 h-3 rounded-full bg-white bg-opacity-50 hover:bg-opacity-75 transition-opacity duration-200 {i ===
        currentIndex
          ? 'bg-opacity-100'
          : ''}"
        on:click={() => goToSlide(i)}
      ></button>
    {/each}
  </div>
</div>

<script>
  import { onMount } from "svelte";

  let currentSlide = 0;
  let startX;

  let slides = [
    [
      {
        type: "offer",
        title: "TALES OF FRANCHISE SALE",
        discount: "-34 - 90%",
        endDate: "6 Sep @ 12:00am",
        image: "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1343400/header.jpg?t=1702553711",
      },
      {
        type: "offer",
        title: "The Escapists",
        discount: "Up to -90%",
        endDate: "10 Sep @ 12:00am",
        image: "/api/placeholder/400/200",
      },
      {
        type: "deal",
        title: "GHPC",
        originalPrice: "£23.79",
        discountedPrice: "£19.03",
        discount: "-20%",
        image: "/api/placeholder/200/100",
      },
      {
        type: "deal",
        title: "VENDIR",
        originalPrice: "£12.79",
        discountedPrice: "£8.95",
        discount: "-30%",
        image: "/api/placeholder/200/100",
      },
      { type: "button", text: "BROWSE MORE" },
    ],
    // Add more slides as needed
  ];

  function nextSlide() {
    currentSlide = (currentSlide + 1) % slides.length;
  }

  function prevSlide() {
    currentSlide = (currentSlide - 1 + slides.length) % slides.length;
  }

  function handleTouchStart(e) {
    startX = e.touches[0].clientX;
  }

  function handleTouchEnd(e) {
    const diffX = startX - e.changedTouches[0].clientX;
    if (Math.abs(diffX) > 50) {
      if (diffX > 0) nextSlide();
      else prevSlide();
    }
  }

  onMount(() => {
    const interval = setInterval(nextSlide, 5000);
    return () => clearInterval(interval);
  });
</script>

<div
  class="relative w-full overflow-hidden"
  on:touchstart={handleTouchStart}
  on:touchend={handleTouchEnd}
>
  <div
    class="flex transition-transform duration-300 ease-in-out"
    style="transform: translateX(-{currentSlide * 100}%);"
  >
    {#each slides as slide}
      <div class="w-full flex-shrink-0 p-4">
        <div class="grid grid-cols-3 gap-4">
          {#each slide as item, i}
            {#if item.type === "offer"}
              <div
                class="col-span-3 md:col-span-1 bg-gray-800 rounded-lg overflow-hidden shadow-lg"
              >
                <img
                  src={item.image}
                  alt={item.title}
                  class="w-full h-48 object-cover"
                />
                <div class="p-4">
                  <h3 class="text-xl font-bold text-white mb-2">
                    {item.title}
                  </h3>
                  <p class="text-sm text-gray-400 mb-1">
                    Offer ends {item.endDate}
                  </p>
                  <p class="text-lg font-bold text-green-400">
                    {item.discount}
                  </p>
                </div>
              </div>
            {:else if item.type === "deal"}
              <div
                class="col-span-3 md:col-span-1 bg-gray-800 rounded-lg overflow-hidden shadow-lg"
              >
                <img
                  src={item.image}
                  alt={item.title}
                  class="w-full h-32 object-cover"
                />
                <div class="p-4">
                  <h3 class="text-lg font-bold text-white mb-2">
                    {item.title}
                  </h3>
                  <div class="flex justify-between items-center">
                    <div>
                      <span class="text-gray-400 line-through text-sm"
                        >{item.originalPrice}</span
                      >
                      <span class="text-white font-bold ml-2"
                        >{item.discountedPrice}</span
                      >
                    </div>
                    <span
                      class="bg-green-500 text-white px-2 py-1 rounded text-sm font-bold"
                    >
                      {item.discount}
                    </span>
                  </div>
                </div>
              </div>
            {:else if item.type === "button"}
              <div class="col-span-3 md:col-span-1">
                <button
                  class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition duration-300 w-full h-full"
                >
                  {item.text}
                </button>
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/each}
  </div>

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
</div>

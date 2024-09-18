<script lang="ts">
  import Marquee from "svelte-fast-marquee";
  import { onMount } from "svelte";

  export let title: string = "Customer Reviews";
  export let subtitle: string =
    "See what our customers have to say about their experiences.";
  export let reviews: Array<{
    name: string;
    review: string;
    rating: number;
    avatar: string;
  }> = [];

  let marquees: Marquee[] = [];

  function StarRating({ rating }: { rating: number }) {
    return `
        <div class="flex items-center">
          ${Array(5)
            .fill(0)
            .map(
              (_, i) => `
            <svg class="${i < rating ? "text-yellow-400" : "text-gray-600"} w-4 h-4 fill-current" viewBox="0 0 24 24">
              <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z" />
            </svg>
          `
            )
            .join("")}
        </div>
      `;
  }

  function handleMouseEnter(index: number) {
    marquees[index]?.pause();
  }

  function handleMouseLeave(index: number) {
    marquees[index]?.play();
  }

  onMount(() => {
    marquees.forEach((marquee) => marquee?.play());
  });
</script>

<div class="w-full text-white py-8">
  <div class="max-w-5xl mx-auto text-center mb-8">
    <h2 class="text-4xl font-medium mb-2">{title}</h2>
    <p class="text-gray-500 text-xl">
      {subtitle} Lorem ipsum dolor sit amet consectetur adipisicing elit. Repudiandae
      corporis assumenda.
    </p>
  </div>

  <div class="space-y-4">
    {#each [1, 2, 3] as layer, index}
      <div
        on:mouseenter={() => handleMouseEnter(index)}
        on:mouseleave={() => handleMouseLeave(index)}
        role="presentation"
      >
        <Marquee
          bind:this={marquees[index]}
          speed={30 + layer * 10}
          direction={layer % 2 === 0 ? "left" : "right"}
        >
          {#each reviews as review}
            <div
              class="bg-gray-800 rounded-lg shadow-lg p-6 mx-2 w-80 relative"
            >
              <p class="text-gray-300 mb-4">"{review.review}"</p>
              {@html StarRating({ rating: review.rating })}
              <div class="flex justify-between items-end mt-4">
                <p class="font-semibold">{review.name}</p>
                <img
                  src={review.avatar}
                  alt={review.name}
                  class="w-12 h-12 bg-gray-700 rounded-full object-cover"
                />
              </div>
            </div>
          {/each}
        </Marquee>
      </div>
    {/each}
  </div>
</div>

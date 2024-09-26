<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { ScrollTrigger } from "gsap/ScrollTrigger";

  gsap.registerPlugin(ScrollTrigger);

  export let post: any = {};

  let componentElement: HTMLElement;

  onMount(() => {
    gsap.from(componentElement, {
      x: -12,
      opacity: 0,
      duration: 0.35,
      ease: "cubic-bezier(0.25, 0.46, 0.45, 0.94)",
      scrollTrigger: {
        trigger: componentElement,
        start: "top bottom-=100",
        toggleActions: "play none",
      },
    });
  });

  function handleReadMore(id: any) {
    console.log(`Read more clicked for post ${id}`);
  }
</script>

<svelte:head>
  <script
    src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.11.4/gsap.min.js"
  ></script>
  <script
    src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.11.4/ScrollTrigger.min.js"
  ></script>
</svelte:head>

<a
  href="#"
  bind:this={componentElement}
  class="p-6 block relative border border-border dark:border-accent rounded-3xl px-4 hover:bg-accent/5 transition duration-150 ease-in-out"
>
  <div class="flex items-center space-x-4">
    <div class="flex-shrink-0">
      <div
        class="h-12 w-12 flex items-center justify-center bg-accent rounded-full"
      >
        <svelte:component
          this={post.icon}
          class="w-6 h-6 text-accent-foreground"
        />
      </div>
    </div>
    <div class="flex-1 min-w-0">
      <h2 class="text-xl font-medium md:pb-1 text-foreground truncate">
        {post.title} (In Progress)
      </h2>
      <p class="text-sm md:text-base text-foreground/70 truncate">
        {post.description}
      </p>
    </div>
    <div>
      <button
        on:click={() => handleReadMore(post.id)}
        class="inline-flex items-center px-2 py-1 border border-transparent text-lg leading-4 rounded-full text-background bg-accent hover:bg-accent/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-accent/50"
      >
        +
      </button>
    </div>
  </div>
</a>

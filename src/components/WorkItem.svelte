<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { ScrollTrigger } from "gsap/ScrollTrigger";
  import { Stretch } from "svelte-loading-spinners";

  gsap.registerPlugin(ScrollTrigger);

  export let item: {
    title: string;
    tags: string[];
    description: string;
    image: string;
    link: string;
  } = {
    title: "",
    tags: [],
    description: "",
    image: "",
    link: "",
  };

  let componentElement: HTMLElement;

  onMount(() => {
    gsap.from(componentElement, {
      x: -12,
      opacity: 0,
      duration: 0.35,
      ease: "cubic-bezier(0.25, 0.46, 0.48, 1)",
      scrollTrigger: {
        trigger: componentElement,
        start: "top bottom-=100",
        toggleActions: "play none",
      },
    });
  });
</script>

<a
  bind:this={componentElement}
  href={item.link}
  target="_blank"
  rel="noopener noreferrer"
  class="flex relative shadow animate-item flex-col md:flex-row !text-foreground md:items-center transition-all border border-border dark:border-accent gap-4 mb-6 md:mb-0 hover:bg-accent/50 p-6 px-5 rounded-3xl"
>
  <div
    class="w-16 h-16 border border-border dark:border-accent mr-1.5 md:min-w-20 md:max-w-20 md:h-20 rounded-2xl md:rounded-2xl overflow-hidden"
  >
    <img
      src={item.image}
      alt="Project Logo"
      class="w-full h-full object-cover"
    />
  </div>
  {#if item.tags.includes("In Progress")}
    <div
      class="invert absolute top-4 right-4 pr-6 dark:invert-0 whitespace-nowrap"
    >
      <Stretch size={10} color="#fff" />
    </div>
  {/if}
  <div>
    <div class="flex flex-wrap items-center gap-3">
      <h4 class="w-full font-medium text-[22px] leading-8 md:w-fit">
        {item.title}
      </h4>
      {#each item.tags as tag}
        <span
          class="tag bg-accent px-3 font-[400] p-1 !rounded-full whitespace-nowrap leading-[1.2]"
          >{tag}</span
        >
      {/each}
    </div>
    <p
      class="mt-4 md:mt-2 text-sm font-[320] tracking-[0.3px] text-muted-foreground text- md:text-[16px]"
    >
      {item.description}
    </p>
  </div>
</a>

<script>
  import Heading from "../../components/Heading.svelte";
  import { workItems } from "../../data/workItems";

  let showMore = false;
  const maxItems = 4;

  function toggleShowMore() {
    showMore = !showMore;
  }
</script>

<div class="max-w-2xl mx-auto">
  <Heading
    heading="Collaborating With Clients From Across The Globe"
    subheading="These are the projects and companies that I've worked on/with. Some are well-established
        businesses backed by VCs. "
    showBreak={true}
  />


  <div class="px-6 mt-8 md:px-0">
    {#each workItems.slice(0, showMore ? workItems.length : maxItems) as item}
      <a
        href={item.link}
        target="_blank"
        rel="noopener noreferrer"
        class="flex flex-col md:flex-row md:items-center transition-all border border-accent md:border-none gap-3.5 mb-6 md:mb-0 hover:bg-accent/50 p-6 px-5 rounded-3xl"
      >
        <div
          class="w-16 h-16 mr-1.5 md:min-w-20 md:max-w-20 md:h-20 rounded-2xl md:rounded-2xl overflow-hidden r"
        >
          <img
            src={item.image}
            alt="Project Logo"
            class="w-full h-full object-cover"
          />
        </div>
        <div>
          <div class="flex flex-wrap items-center gap-2">
            <h4 class="w-full md:w-fit text-[23px] leading-8 font-medium">
              {item.title}
            </h4>
            {#each item.tags as tag}
              <span
                class="tag bg-accent px-3 p-1 !rounded-full whitespace-nowrap leading-[1.2]"
                >{tag}</span
              >
            {/each}
          </div>
          <p class="mt-4 md:mt-2 text-muted-foreground text-sm">
            {item.description}
          </p>
        </div>
      </a>
    {/each}

    {#if !showMore && workItems.length > maxItems}
      <button
        class="mx-auto block px-3 p-1 bg-accent rounded-full mt-8"
        on:click={toggleShowMore}
      >
        Show More
      </button>
    {/if}

    {#if showMore}
      <button
        class="mx-auto block px-3 p-1 bg-accent rounded-full mt-8"
        on:click={toggleShowMore}
      >
        Show Less
      </button>
    {/if}
  </div>
</div>

<script>
  import Heading from "../../components/Heading.svelte";
  import { workItems } from "../../data/workItems";
  import WorkItem from "../../components/WorkItem.svelte";

  let showMore = false;
  const maxItems = 4;

  function toggleShowMore() {
    showMore = !showMore;
  }
</script>

<div class="max-w-2xl mx-auto">
  <Heading
    heading="Connecting with Clients From Across the Globe"
    subheading="These are the projects and companies that I've worked on/with. Some are well-established
        businesses backed by VCs. "
    showBreak={true}
  />

  <div class="px-6 space-y-4 mt-6 md:px-0">
    {#each workItems.slice(0, showMore ? workItems.length : maxItems) as item}
      <WorkItem {item} />
    {/each}

    {#if !showMore && workItems.length > maxItems}
      <button
        class="mx-auto block px-3 p-1 bg-transparent border border-border text-foreground rounded-full"
        on:click={toggleShowMore}
      >
        Show More
      </button>
    {/if}

    {#if showMore}
      <button
        class="mx-auto block px-3 p-1 bg-transparent border border-border text-foreground rounded-full"
        on:click={toggleShowMore}
      >
        Show Less
      </button>
    {/if}
  </div>
</div>

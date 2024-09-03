<script>
  import { workItems } from "../../data/workItems";


	let showMore = false;
	const maxItems = 4;

	function toggleShowMore() {
		showMore = !showMore;
	}
</script>

<div class="max-w-2xl mx-auto">
    <h3 class="px-6 md:px-0 text-3xl text-center font-medium">Previous Work & Clients</h3>
    <p class="mt-4 text-[16px] text-center px-6 md:px-0">
        These are the projects and companies that I've worked on/with. Some are well-established
        businesses backed by VCs while others are MVP/initial stage projects I've been able to create and
        contribute to from scratch.
    </p>
    <div class="w-full h-[1px] my-8 mb-6 bg-[--color-secondary]"></div>
    
    <div class="px-6 md:px-0 pb-40">
        {#each workItems.slice(0, showMore ? workItems.length : maxItems) as item}
        <a
        href={item.link}
        target="_blank"
        rel="noopener noreferrer"
        class="flex flex-col md:flex-row md:items-center  transition-all border border-[--color-tertiary] md:border-none gap-4 mb-6 md:mb-0 hover:bg-accent p-4 px-3 rounded-xl"
        >
        <div
            class="w-16 h-16 mr-1.5 md:min-w-20 md:max-w-20 md:h-20 rounded-lg md:rounded-2xl overflow-hidden border border-[--color-tertiary]"
        >
            <img src={item.image} alt="Project Logo" class="w-full h-full object-cover" />
        </div>
        <div>
            <div class="flex flex-wrap items-center gap-3">
                <h4 class="w-full md:w-fit text-[20px] font-medium">{item.title}</h4>
                {#each item.tags as tag}
                    <span class="tag  border border-[--color-tertiary] whitespace-nowrap leading-[1.2]"
                        >{tag}</span
                    >
                {/each}
            </div>
            <p class="mt-4 md:mt-2 text-sm md:text-[16px]">{item.description}</p>
        </div>
        </a>
        {/each}
        
        {#if !showMore && workItems.length > maxItems}
            <button class="mx-auto block text-[--color-primary] mt-8" on:click={toggleShowMore}>
                Show More
            </button>
        {/if}
        
        {#if showMore}
            <button class="mx-auto block text-[--color-primary] mt-8" on:click={toggleShowMore}>
                Show Less
            </button>
        {/if}
    </div>
    
</div>

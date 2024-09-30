<script>
  import Heading from "../../components/Heading.svelte";
  import { Filter, Laptop, Coffee, Brain, Zap, GitBranch } from "lucide-svelte";
  import { blogPosts } from "../../data/BlogPosts";
  import BlogItem from "../../components/BlogItem.svelte";

  // Available categories for filtering
  const categories = [
    { name: "All", icon: Filter },
    { name: "Design", icon: Zap },
    { name: "Technology", icon: Laptop },
    { name: "Productivity", icon: Coffee },
    { name: "Data", icon: GitBranch },
    { name: "Intelligence", icon: Brain },
  ];

  let selectedCategory = "All";

  function handleReadMore(id) {
    console.log(`Read more clicked for post ${id}`);
  }

  function handleCategoryFilter(category) {
    selectedCategory = category;
  }

  $: filteredPosts =
    selectedCategory === "All"
      ? blogPosts
      : blogPosts.filter(
          (post) =>
            post.category.toLowerCase() === selectedCategory.toLowerCase()
        );
</script>

<div class="max-w-2xl mx-auto">
  <Heading
    heading="My Blog & Ideas"
    subheading="Thoughts and insights on product development, design, and technology. Sharing my observations from the evolving world of technology."
    showBreak={false}
  />
</div>

<div class="max-w-2xl w-[calc(100%-40px)] mt-7 mx-auto overflow-hidden">
  <div
    class="mb-4 flex flex-nowrap gap-2 pb-4 overflow-x-scroll lg:overflow-auto"
  >
    {#each categories as category}
      <button
        on:click={() => handleCategoryFilter(category.name)}
        class="inline-flex items-center border border-border dark:border-accent px-3 py-1 rounded-full text-sm font-medium transition-colors duration-200 ease-in-out"
        class:bg-foreground={selectedCategory === category.name}
        class:text-background={selectedCategory === category.name}
        class:text-foreground={selectedCategory !== category.name}
      >
        <svelte:component this={category.icon} class="w-4 h-4 mr-2" />
        {category.name}
      </button>
    {/each}
  </div>

  <ul class="space-y-4 dark:divide-accent">
    {#each filteredPosts as post}
      <BlogItem {post} on:readMore={handleReadMore} />
    {/each}
  </ul>
</div>

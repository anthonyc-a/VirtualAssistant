<script>
  import Heading from "../../components/Heading.svelte";
  import { Filter, Laptop, Coffee, Brain, Zap, GitBranch } from "lucide-svelte";

  // Updated blog posts data with categories
  const blogPosts = [
    {
      id: 1,
      title: "Adaptive UI",
      description:
        "Learn how to create responsive and adaptive user interfaces.",
      category: "design",
      icon: Zap,
    },
    {
      id: 2,
      title: "Aarkyv OS",
      description: "Exploring the future of operating systems and computing.",
      category: "technology",
      icon: Laptop,
    },
    {
      id: 3,
      title: "Workflow 24/25",
      description:
        "Boost your productivity with these workflow optimization tips.",
      category: "productivity",
      icon: Coffee,
    },
    {
      id: 4,
      title: "AI in Design",
      description:
        "How artificial intelligence is revolutionizing the design process.",
      category: "ai",
      icon: Brain,
    },
    {
      id: 5,
      title: "HoloViz",
      description:
        "Learn how to create stunning data visualizations with D3.js.",
      category: "data",
      icon: GitBranch,
    },
    {
      id: 6,
      title: "Machine Learning",
      description: "An introduction to machine learning and its applications.",
      category: "ai",
      icon: Brain,
    },
    {
      id: 7,
      title: "Web Scraping",
      description:
        "Scrape the web and extract useful data with Python and BeautifulSoup.",
      category: "data",
      icon: GitBranch,
    },
    {
      id: 8,
      title: "Natural Language Processing",
      description:
        "An overview of natural language processing and its use cases.",
      category: "ai",
      icon: Brain,
    },
    {
      id: 9,
      title: "Design Systems",
      description:
        "Create a design system to maintain consistency across your projects.",
      category: "design",
      icon: Zap,
    },
    {
      id: 10,
      title: "Web Development",
      description:
        "A beginner's guide to web development and front-end technologies.",
      category: "technology",
      icon: Laptop,
    },
  ];

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
    subheading="Thoughts and insights on product development, design, and technology. Sharing observations from the evolving world of tech."
    showBreak={false}
  />
</div>

<div class="max-w-2xl md:px-5 w-[calc(100%-40px)] mt-6 mx-auto overflow-hidden">
  <div
    class="mb-6 flex flex-nowrap gap-2 pb-4  overflow-scroll"
  >
    {#each categories as category}
      <button
        on:click={() => handleCategoryFilter(category.name)}
        class="inline-flex items-center border border-accent px-3 py-1 rounded-full text-sm font-medium transition-colors duration-200 ease-in-out"
        class:bg-foreground={selectedCategory === category.name}
        class:text-background={selectedCategory === category.name}
        class:opacity-80={selectedCategory !== category.name}
        class:text-foreground={selectedCategory !== category.name}
      >
        {#if category.icon === Filter}
          <svelte:component this={category.icon} class="w-4 h-4 mr-2" />
        {/if}
        {category.name}
      </button>
    {/each}
  </div>

  <ul class="space-y-4 dark:divide-accent">
    {#each filteredPosts as post}
      <li
        class="p-4 border border-accent rounded-3xl px-4 hover:bg-accent/5 transition duration-150 ease-in-out"
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
            <h2 class="text-xl font-medium text-foreground truncate">
              {post.title}
            </h2>
            <p class="text-sm text-foreground/70 truncate">
              {post.description}
            </p>
          </div>
          <div>
            <button
              on:click={() => handleReadMore(post.id)}
              class="inline-flex items-center px-2 py-1 border border-transparent text-lg leading-4 rounded-full text-background bg-foreground hover:bg-accent/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-accent/50"
            >
              +
            </button>
          </div>
        </div>
      </li>
    {/each}
  </ul>
</div>

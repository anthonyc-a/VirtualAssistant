<script>
  import Heading from "../../components/Heading.svelte";

  // Placeholder data for blog posts
  const blogPosts = [
    {
      id: 1,
      title: "10 Tips for Productive Remote Work",
      description:
        "Learn how to stay focused and efficient while working from home.",
      imageUrl: "/api/placeholder/60/60",
    },
    {
      id: 2,
      title: "The Future of Artificial Intelligence",
      description:
        "Exploring the potential impact of AI on various industries.",
      imageUrl: "/api/placeholder/60/60",
    },
    {
      id: 3,
      title: "Healthy Eating Habits for Busy Professionals",
      description: "Quick and nutritious meal ideas for your hectic schedule.",
      imageUrl: "/api/placeholder/60/60",
    },
  ];

  function handleReadMore(id) {
    // Placeholder function for read more button
    console.log(`Read more clicked for post ${id}`);
  }
</script>

<div class="max-w-2xl mx-auto px-2">
  <Heading
    heading="Blog & Thoughts"
    subheading="In progress"
    showBreak={false}
  />
</div>
<div
  class="max-w-xl mx-auto bg-background shadow-md rounded-lg overflow-hidden"
>
  <ul class="divide-y divide-accent/20">
    {#each blogPosts as post}
      <li class="p-4 hover:bg-accent/5 transition duration-150 ease-in-out">
        <div class="flex items-center space-x-4">
          <div class="flex-shrink-0">
            <img
              class="h-12 w-12 bg-accent rounded-full"
              src={post.imageUrl}
              alt={post.title}
            />
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
              class="inline-flex items-center px-3 py-1 border border-transparent text-sm leading-4 font-medium rounded-md text-foreground bg-accent hover:bg-accent/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-accent/50"
            >
              Read More
            </button>
          </div>
        </div>
      </li>
    {/each}
  </ul>
</div>

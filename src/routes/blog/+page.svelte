<script>
  import Heading from "../../components/Heading.svelte";

  // Placeholder data for blog posts
  const blogPosts = [
    {
      id: 1,
      title: "Adaptive UI 1",
      description:
        "Learn how to stay focused and efficient while working from home.",
      imageUrl: "/refract.png",
    },
    {
      id: 2,
      title: "Aarkyv OS",
      description:
        "Exploring the potential impact of AI on various industries.",
        imageUrl: "/refract.png",
      },
    {
      id: 3,
      title: "Workflow 24/25",
      description: "Quick and nutritious meal ideas for your hectic schedule.",
      imageUrl: "/refract.png",
    },
  ];

  function handleReadMore(id) {
    // Placeholder function for read more button
    console.log(`Read more clicked for post ${id}`);
  }
</script>

<div class="max-w-2xl mx-auto px-2">
  <Heading
    heading="My Blog, Thoughts and Implementations"
    subheading="I write about design, development, and productivity."
    showBreak={false}
  />
</div>
<div
  class="max-w-xl w-[calc(100%-40px)] mt-3 mx-auto overflow-hidden"
>
  <ul class="divide-y divide-accent">
    {#each blogPosts as post}
      <li class="p-4 px-1 hover:bg-accent/5 transition duration-150 ease-in-out">
        <div class="flex items-center space-x-4">
          <div class="flex-shrink-0">
            <img
              class="h-12 w-12 shadow bg-accent rounded-full"
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
              In Progress
            </button>
          </div>
        </div>
      </li>
    {/each}
  </ul>
</div>

<script lang="ts">
    import { ChevronUp, Clock, Layout, Zap, LayoutList, TrendingDown } from 'lucide-svelte';
  
    export let category: string = "Management";
    export let title: string = "Agile Project Planning";
    export let subtitle: string = "Drive project success with agile project management capabilities tailored for small teams focused on rapid product development.";
    export let features: {
      icon: typeof ChevronUp;
      title: string;
      description: string;
    }[] = [
      { icon: ChevronUp, title: "Sprint Planning", description: "Plan and execute project tasks efficiently within iterative sprint cycles." },
      { icon: Layout, title: "Kanban Boards", description: "Visualize project workflow and track task progress with customizable Kanban boards." },
      { icon: Zap, title: "Task Prioritization", description: "Prioritize tasks based on urgency and importance to ensure efficient use of resources." },
      { icon: Clock, title: "Collaborative Task Boards", description: "Collaboratively manage tasks and assignments in real-time, fostering teamwork and accountability." },
      { icon: LayoutList, title: "Backlog Management", description: "Maintain a backlog of tasks and user stories, ensuring a steady flow of work for your team." },
      { icon: TrendingDown, title: "Burndown Charts", description: "Monitor project progress and identify potential bottlenecks with easy-to-read burndown charts." }
    ];
  </script>
  
  <div class=" text-white p-8 rounded-lg">
    <div class="text-center mb-8">
      <span class="text-gray-400 text-sm mb-2 inline-block">{category}</span>
      <h2 class="text-3xl font-bold mb-4">{title}</h2>
      <p class="text-gray-300 max-w-2xl mx-auto">{subtitle}</p>
    </div>
  
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
      {#each features as feature}
        <div class="bg-gray-800 p-6 rounded-lg">
          <svelte:component this={feature.icon} class="w-6 h-6 mb-4 text-blue-400" />
          <h3 class="text-lg font-semibold mb-2">{feature.title}</h3>
          <p class="text-gray-400 text-sm">{feature.description}</p>
        </div>
      {/each}
    </div>
  
    <div class="flex justify-center space-x-4">
      <button class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-2 rounded-full transition duration-300">
        Get started
      </button>
      <button class="bg-gray-700 hover:bg-gray-600 text-white px-6 py-2 rounded-full transition duration-300">
        See more
      </button>
    </div>
  </div>
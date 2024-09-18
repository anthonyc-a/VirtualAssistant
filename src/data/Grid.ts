import {
  ChevronUp,
  Clock,
  Layout,
  Zap,
  LayoutList,
  TrendingDown,
} from "lucide-svelte";

export const features = [
  {
    icon: ChevronUp,
    title: "Sprint Planning",
    description:
      "Plan and execute project tasks efficiently within iterative sprint cycles.",
  },
  {
    icon: Layout,
    title: "Kanban Boards",
    description:
      "Visualize project workflow and track task progress with customizable Kanban boards.",
  },
  {
    icon: Zap,
    title: "Task Prioritization",
    description:
      "Prioritize tasks based on urgency and importance to ensure efficient use of resources.",
  },
  {
    icon: Clock,
    title: "Collaborative Task Boards",
    description:
      "Collaboratively manage tasks and assignments in real-time, fostering teamwork and accountability.",
  },
  {
    icon: LayoutList,
    title: "Backlog Management",
    description:
      "Maintain a backlog of tasks and user stories, ensuring a steady flow of work for your team.",
  },
  {
    icon: TrendingDown,
    title: "Burndown Charts",
    description:
      "Monitor project progress and identify potential bottlenecks with easy-to-read burndown charts.",
  },
];

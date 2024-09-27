import { Brain, Coffee, GitBranch, Laptop, Zap } from "lucide-svelte";

export const blogPosts = [
  {
    id: 1,
    title: "Adaptive UI",
    description: "Learn how to create responsive and adaptive user interfaces.",
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
    title: "Working Process",
    description:
      "Boost your productivity with these workflow optimization tips.",
    category: "productivity",
    icon: Coffee,
  },
  {
    id: 4,
    title: "HoloVis",
    description:
      "Rendering 3D visualizations and particle-based holographic projections.",
    category: "data",
    icon: GitBranch,
  },
  {
    id: 5,
    title: "Tailwind Natural Language Processing",
    description:
      "An overview of natural language processing and its use cases.",
    category: "intelligence",
    icon: Brain,
  },
];

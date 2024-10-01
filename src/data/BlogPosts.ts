import { Brain, Coffee, GitBranch, Laptop, Zap } from "lucide-svelte";

export const blogPosts = [
  {
    id: 1,
    title: "Adaptive UI",
    description: "Creating a fluid and responsive interface system.",
    category: "design",
    icon: Zap,
  },
  {
    id: 2,
    title: "Aarkyv OS",
    description: "Exploring the feasability of creating a lightweight OS layer.",
    category: "technology",
    icon: Laptop,
  },
  {
    id: 3,
    title: "Working Process",
    description:
      "My personal workflow and how I manage my time and tasks.",
    category: "productivity",
    icon: Coffee,
  },
  {
    id: 4,
    title: "HoloVis",
    description:
      "Rendering 3D visualisations and particle-based holographic projections.",
    category: "data",
    icon: GitBranch,
  },
  {
    id: 5,
    title: "Tailwind Natural Language Processing",
    description:
      "Creating interfaces with nothing but natural language.",
    category: "intelligence",
    icon: Brain,
  },
];

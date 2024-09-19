<script>
  import Heading from "../../components/Heading.svelte";
  import { Palette, Laptop, Code, Cog } from 'lucide-svelte';

  const items = [
    {
    title: "Branding & Identity",
    description:
      "Extensive experience working in the realm of digital marketing and branding enables me to establish a strong foundation using typography, color and photography to ensure users perceive your brand in a true and consistent manner.",
    icon: Palette,
  },
  {
    title: "Digital Design",
    description:
      "A designer by nature - I have an eye for detail and a passion for creating functional, satisfying user experiences. With a strong understanding of the design process, I can help point you in the right direction no matter what stage you're at.",
    icon: Laptop,
  },
  {
    title: "Full-Stack Development",
    description:
      "My expertise as a full-stack developer allows me to rapidly create custom end-to-end solutions for clients. I use the latest industry-standard technologies to create fast, responsive and secure websites and apps that are built to scale.",
    icon: Code,
  },
  {
    title: "Automation and Efficiency",
    description:
      "With a deep understanding of workflow optimization, I implement cutting-edge automation solutions that streamline processes, reduce manual tasks, and boost productivity. From simple scripts to complex AI-driven systems, I create tailored automation strategies that drive efficiency and innovation in your business operations.",
    icon: Cog,
  },
  ];
</script>

<div class="max-w-2xl mx-auto px-2">
  <Heading
    heading="Senior Interface Designer &  Full-Stack Developer"
    subheading="Embracing an iterative approach to design and development, creating flexible products that evolve with client needs and market demands."
    showBreak={false}
  />

  <div
    class="flex flex-col gap-6 mt-8 px-4 md:p-8 md:border border-border rounded-3xl"
  >
    {#each items as item}
      <div class="p-6 px-8 border border-border dark:border-accent rounded-3xl flex flex-col gap-3 items-start">
        <svelte:component this={item.icon} strokeWidth={1.65} class="w-8 h-8 mr-4 flex-shrink-0 text-muted-foreground" />
        <div>
          <h4 class="text-[23px] leading-8 font-medium">{item.title}</h4>
          <p
            class="mt-3 text-[16px] leading-[1.65] tracking-wide font-light text-muted-foreground"
          >
            {item.description}
          </p>
        </div>
      </div>
    {/each}
  </div>
</div>
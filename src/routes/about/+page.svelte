<script>
  const items = [
    {
      title: "Branding and Identity",
      description:
        "Extensive experience working in the realm of digital marketing and branding enables me to establish a strong foundation using typography, color and photography to ensure users perceive your brand in a true and consistent manner.",
    },
    {
      title: "Digital Design",
      description:
        "A designer by nature - I have an eye for detail and a passion for creating functional, satisfying user experiences. With a strong understanding of the design process, I can help point you in the right direction no matter what stage you're at.",
    },
    {
      title: "Full-Stack Development",
      description:
        "My expertise as a full-stack developer allows me to rapidly create custom end-to-end solutions for clients. I use the latest industry-standard technologies to create fast, responsive and secure websites and apps that are built to scale.",
    },
  ];
</script>

<h3 class="px-6 md:px-0">Designer & Full-Stack Developer</h3>

<p class="mt-4 text-[16px] px-6 md:px-0">
  I work best in goal-oriented, teamworking environments where I can present
  thoughts and ideas to entire teams; usually focussing on an iterative approach
  to design and development where form and function live side-by-side.
</p>

<div
  class="flex flex-col gap-4 mt-8 px-6 md:p-8 md:border border-[--color-tertiary] rounded-xl"
>
  {#each items as item}
    <div class="p-6 px-8 border border-[--color-tertiary] rounded-xl">
      <h4>{item.title}</h4>
      <p class="mt-3 text-[16px]">
        {item.description}
      </p>
    </div>
  {/each}
</div>

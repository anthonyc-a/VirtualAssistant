<script>
  import { Stretch } from "svelte-loading-spinners";
</script>

<div
  class="max-w-3xl shadow-md dark:shadow-none shadow-foreground/5 hover:!border-border w-[calc(100%-60px)] md:w-full flex flex-col md:flex-row items-center gap-7 md:gap-10 p-6 md:!py-2 transition-all border border-border hover:bg-accent/30 dark:border-accent rounded-3xl min-h-[224px] md:min-h-[232px] mx-auto"
>
  <div
    class="min-w-[180px] md:min-h-[170px] w-full md:w-[unset] h-full flex justify-center items-center bg-white border border-border dark:border-accent rounded-3xl"
  >
    <img
      src="/refract-tm.png"
      alt="Logo"
      class="w-[140px] lg:scale-110 md:w-32 h-[140px] md:h-32 my-0"
    />
  </div>

  <div>
    <div
      class="flex flex-col md:flex-row md:items-center flex-wrap font-medium mb-2 md:mb-2 gap-2.5 md:gap-3.5"
    >
      <h3 class="inline-block text-[26px] md:text-[26px] leading-[1.4]">
        Refract Agency
      </h3>
      <div class="md:flex hidden gap-3 flex-wrap">
        <span
          class="tag-l w-fit bg-accent tracking-[0.1px] font-[450] text-foreground px-2.5 text-[15px] rounded-2xl p-0.5"
          >Digital Marketing</span
        >
      </div>
    </div>
    <div>
      <p
        class="text-[16px] md:text-[17px] leading-[1.6] font-[310] tracking-[0.2px] dark:text-muted-foreground"
      >
        My marketing agency made to empower businesses and accept/manage client work.
      </p>
    </div>
    <div class="flex gap-2 mt-4">
      <button
        class="px-4 p-2 flex border text-sm bg-foreground text-background font-[500] rounded-full"
        on:click={() => window.open("https://refract-agency.com")}
        ><div class="dark:invert whitespace-nowrap">
          <Stretch size={10} color="#fff" />
        </div>
        <span class="pl-6"> In progress </span>
      </button>
      <button
        class="bg-transparent px-4 p-2 text-sm rounded-full border border-[#555] hover:border-white"
        on:click={() =>
          (window.location.href = "mailto:info@refract-agency.com")}
        >Get in touch</button
      >
    </div>
  </div>
</div>

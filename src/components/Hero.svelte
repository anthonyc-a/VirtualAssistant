<div
  class="max-w-2xl hover:!border-border w-[calc(100%-64px)] flex flex-col md:flex-row items-center gap-7 md:gap-10 p-6 transition-all border border-border hover:bg-accent/30 dark:border-accent rounded-3xl min-h-[224px] mx-auto"
>
  <div
    class="min-w-[180px] w-full h-full flex justify-center items-center bg-white border border-accent dark:border-accent rounded-3xl"
  >
    <img
      src="https://www.refract-agency.com/favicon.png"
      alt="Logo"
      class="w-[100px] md:w-32 h-[100px] md:h-32 my-4"
    />
  </div>

  <div class="md:mt-4 md:my-0">
    <div class="flex flex-col md:flex-row md:items-center flex-wrap font-medium mb-2 md:mb-2 gap-2.5 md:gap-3">
      <h3 class="inline-block text-[26px] leading-[1.4]">Refract Agency</h3>
      <div class="md:flex hidden gap-3 flex-wrap">
        <span
        class="tag-l w-fit bg-foreground text-background px-2 text-[13px] rounded-lg p-0.5"
        >Marketing Agency</span
      >
      <span
        class="tag-l w-fit bg-foreground text-background px-2 text-[13px] rounded-lg p-0.5"
        >Founder</span
      >
      </div>

    </div>
    <div>
      <p
        class="text-[15px] leading-[1.55] font-[310] tracking-[0.2px] dark:text-muted-foreground"
      >
        My design, development and visualisation studio for creative businesses
        
      </p>
    </div>
    <div class="flex gap-2 mt-4">
      <button
        class="px-4 p-2 border text-sm bg-foreground text-background font-[500] rounded-full"
        on:click={() => window.open("https://refract-agency.com")}
        >Visit Website</button
      >
      <button
        class="bg-transparent px-4 p-2 text-sm rounded-full border border-[#555] hover:border-white"
        on:click={() => (window.location.href = "mailto:hello@aarkyv.com")}
        >Contact Us</button
      >
    </div>
  </div>
</div>

<div class="max-w-2xl flex items-center gap-10 p-6  transition-all border border-muted-accent hover:border-foreground dark:border-accent rounded-3xl h-[224px] mx-auto">
  <div class="min-w-[180px] h-full flex justify-center items-center bg-white border border-muted-accent dark:border-accent rounded-3xl">
    <img src="https://www.refract-agency.com/favicon.png" alt="Logo" class="w-32 h-32">
  </div>

  <div class="mt-4 md:my-0">
    <div class="flex items-center flex-wrap font-medium mb-4 md:mb-2 gap-3">
        <h3 class="inline-block text-[27px] leading-[1.4] ">Refract Agency</h3>
        <span class="tag-l bg-foreground text-background px-2 text-[13px] rounded-lg p-0.5">Marketing Agency</span>
        <span class="tag-l bg-foreground text-background px-2 text-[13px] rounded-lg p-0.5">Founder</span>
    </div>
    <div>
        <p class="text-[15px] leading-[1.65] font-[300] tracking-wide dark:text-muted-foreground">
            My design, development and visualisation studio for creative businesses and ambitious
            startups.
        </p>
    </div>
    <div class="flex gap-2  mt-4">
        <button class="px-4 p-2 border text-sm bg-foreground text-background font-[500] rounded-full" on:click={() => window.open('https://aarkyv.com/studio', '_blank')}
            >Visit Website</button
        >
        <button
            class="bg-transparent px-4 p-2 text-sm  rounded-full border border-[#555] hover:border-white"
            on:click={() => (window.location.href = 'mailto:hello@aarkyv.com')}>Contact Us</button
        >
    </div>
</div>
</div>

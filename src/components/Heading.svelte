<script>
    export let heading = "";
    export let subheading = "";
    export let showBreak = false;
  </script>
  
  <section class="heading-section space-y-1 pb-2 px-7 md:px-0">
    <h1
      class="text-[27px] leading-[35px] md:leading-[42px] text-center md:text-center mx-auto md:text-[30px] max-w-2xl xl:max-w-3xl tracking-[0.1px] font-[500] w-fit"
    >
      {#if showBreak}
        {@html heading.replace('<br>', '<br class="hidden xl:inline" />')}
      {:else}
        {heading}
      {/if}
    </h1>
    <p
      class="pt-3 md:pt-4 text-center md:text-center mx-auto md:text-[18px] leading-[1.65] md:leading-[33px] tracking-[0.3px] font-[320] text-muted-foreground  mx-auto"
    >
      {subheading}
    </p>
  </section>
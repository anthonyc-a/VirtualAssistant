<script>
    export let heading = "";
    export let subheading = "";
    export let showBreak = false;
  </script>
  
  <section class="heading-section px-7">
    <h1
      class="text-[27px] leading-[36px] text-center md:text-[32px] max-w-2xl xl:max-w-3xl tracking-[0.1px] font-[500] mx-auto w-fit"
    >
      {#if showBreak}
        {@html heading.replace('<br>', '<br class="hidden xl:inline" />')}
      {:else}
        {heading}
      {/if}
    </h1>
    <p
      class="pt-3.5 text-center leading-[1.65] tracking-[0.2px] font-light text-muted-foreground max-w-2xl mx-auto"
    >
      {subheading}
    </p>
  </section>
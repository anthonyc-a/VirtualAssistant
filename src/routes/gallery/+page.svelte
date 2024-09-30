<script lang="ts">
  const images = [
    {
      src: "/architecture.png",
      alt: "Gallery Pic",
      chroma: true,
    },
    {
      src: "/archviz.png",
      alt: "Gallery Pic",
      chroma: false,
    },
    {
      src: "/waveform.png",
      alt: "Gallery Pic",
      chroma: true,
    },
    {
      src: "/ve-go.png",
      alt: "Gallery Pic",
      chroma: true,
    },

    {
      src: "/vizcom.png",
      alt: "Gallery Pic",
      chroma: true,
    },
    {
      src: "/epoch.png",
      alt: "Gallery Pic",
      chroma: false,
    },
    {
      src: "/revolut.png",
      alt: "Gallery Pic",
      chroma: true,
    },
    {
      src: "/studio-pb.png",
      alt: "Gallery Pic",
      chroma: false,
    },

    {
      src: "/museum.png",
      alt: "Gallery Pic",
      chroma: false,
    },
    {
      src: "/daydream.png",
      alt: "Gallery Pic",
      chroma: true,
    },

    {
      src: "/space.png",
      alt: "Gallery Pic",
      chroma: false,
    },
    {
      src: "/pinque.png",
      alt: "Gallery Pic",
      chroma: false,
    },
  ];

  function preload(src: any) {
    return new Promise(function (resolve) {
      let img = new Image();
      img.onload = resolve;
      img.src = src;
    });
  }
</script>

<div class="grid md:grid-cols-2 max-w-2xl mx-auto gap-4 px-6 md:px-0">
  {#each images as image}
    <div
      class="relative w-full h-56 shadow border border-border dadrk:border-accent rounded-2xl overflow-hidden"
    >
      {#await preload(image.src) then _}
        <img
          src={image.src}
          alt={image.alt}
          class="w-full h-full object-cover"
        />
      {/await}
      {#if image.chroma}
        <a
          href="hhttps://chroma.co.in"
          target="_blank"
          rel="noreferrer"
          class="hover:brightness-75 transition-all"
        >
          <img
            src="/app-icon.png"
            alt="Chroma"
            class="w-6 h-6 absolute bottom-3 right-3 rounded-md"
          />
        </a>
      {/if}
    </div>
  {/each}
</div>

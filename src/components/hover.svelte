<script>
  import { spring } from "svelte/motion";
  import { onMount } from "svelte";

  let glowIntensity = spring(0, {
    stiffness: 0.1,
    damping: 0.25,
  });

  let container;

  onMount(() => {
    const handleMouseMove = (event) => {
      const { clientX } = event;
      const { width } = container.getBoundingClientRect();
      const distance = width - clientX;
      const maxDistance = width / 3; // Adjust this value to change the active area
      const intensity = Math.max(0, 1 - distance / maxDistance);
      glowIntensity.set(intensity);
    };

    const handleMouseLeave = () => {
      glowIntensity.set(0);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseleave", handleMouseLeave);

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseleave", handleMouseLeave);
    };
  });
</script>

<div bind:this={container} class="glow-container z-[9999]">
  <div
    class="glow-effect"
    style="opacity: {$glowIntensity}; 
             background: linear-gradient(to left, 
               rgba(155, 155, 155, {$glowIntensity * 0.5}), 
               transparent);"
  ></div>
  <slot></slot>
</div>

<style>
  .glow-container {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .glow-effect {
    position: absolute;
    top: 0;
    right: 0;
    width: 33%; /* Adjust this to change the width of the glow */
    height: 100%;
    pointer-events: none;
    transition: opacity 0.2s ease;
  }
</style>

<script>
  import { fly, fade, scale } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { CheckCircle } from "lucide-svelte";
  import { onMount } from "svelte";

  export let isVisible;
</script>

<div
  in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: cubicOut }}
  out:fade={{ duration: 300 }}
  class="fixed whitespace-nowrap left-1/2 -translate-x-1/2 z-[99999999] bg-green-500 text-white px-4 py-2 rounded-full flex items-center space-x-2"
  class:bottom-24={isVisible}
  class:bottom-5={!isVisible}
>
  <CheckCircle size={18} />
  <span>Message sent successfully!</span>
</div>

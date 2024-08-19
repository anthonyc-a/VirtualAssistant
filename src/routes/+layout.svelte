<script>
  import "../app.css";
  import Header from "../components/Header.svelte";
</script>


<div class="fixed flex justify-center items-center rounded-xl !font-light text-white border border-[#252525] top-24 left-4 w-10 h-10 ">
  +
</div>

<div class=" top-0 text-center w-full text-xs font-medium py-1 bg-white text-[#252525] mb-0">
  Hello, and welcome to the app!
</div>
<Header/>
<slot />

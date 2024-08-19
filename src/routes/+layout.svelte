<script>
  import "../app.css";
  import Header from "../components/Header.svelte";
</script>

<div
  class="fixed flex justify-center items-center rounded-full !font-light text-white border border-[#252525] bg-[#252525] bg-opacity-30 top-[90px] left-3.5 w-9 h-9"
>
  +
</div>

<div class="fixed top-12 right-4 text-[#999]">
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="1em"
    height="1em"
    fill="currentColor"
    viewBox="0 0 256 256"
    class="w-5 h-5"
    ><path
      d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM40,56H80V200H40ZM216,200H96V56H216V200Z"
    ></path></svg
  >
</div>

<div
  class=" top-0 px-12 w-full text-xs text-center font-medium py-1 bg-[#fff] text-[#252525] mb-0"
>
  Hello, and welcome to the app!
</div>

<Header />
<slot />

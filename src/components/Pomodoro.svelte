<!-- src/routes/Pomodoro.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let timer = {
    remaining: 25 * 60,
    duration: 25 * 60,
    is_active: false,
    is_break: false,
    completed_pomodoros: 0,
  };

  let intervalId: number;

  $: progress = 100 - (timer.remaining / timer.duration) * 100;
  $: timeString = formatTime(timer.remaining);
  $: strokeDasharray = `${progress} ${100 - progress}`;

  function formatTime(seconds: number): string {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes.toString().padStart(2, "0")}:${remainingSeconds.toString().padStart(2, "0")}`;
  }

  async function startTimer() {
    timer = await invoke("start_timer");
  }

  async function pauseTimer() {
    timer = await invoke("pause_timer");
  }

  async function playTimer() {
    timer.is_active
      ? (timer = await invoke("pause_timer"))
      : (timer = await invoke("start_timer"));
  }

  async function resetTimer() {
    timer = await invoke("reset_timer");
  }

  async function updateTimer() {
    timer = await invoke("get_timer_state");
  }

  onMount(() => {
    intervalId = setInterval(updateTimer, 1000) as unknown as number;
  });

  onDestroy(() => {
    clearInterval(intervalId);
  });
</script>

<div
  class="fixed bottom-24 right-4 cursor-pointer w-20 h-20 hover:scale-105 z-[9999] transition-all"
  on:click={playTimer}
  role="presentation"
  style={`${!timer.is_active && "opacity: 0.5;"}`}
>
  <svg class="w-full h-full" viewBox="0 0 100 100">
    <!-- Background circle -->
    <circle
      class="text-[#252525] "
      stroke-width="5"
      stroke="currentColor"
      fill="transparent"
      r="45"
      cx="50"
      cy="50"
    />
    <!-- Timer progress -->
    <circle
      class="{timer.is_break
        ? 'text-green-500'
        : 'text-red-500'} transform -rotate-90 origin-center"
      stroke-width="5"
      stroke-dasharray={strokeDasharray}
      stroke-linecap="round"
      stroke="currentColor"
      fill="transparent"
      r="45"
      cx="50"
      cy="50"
    />
    <!-- Timer text -->
    <text
      x="50"
      y="50"
      fill="white"
      font-size="20"
      text-anchor="middle"
      alignment-baseline="central"
      class="font-bold"
    >
      {timeString}
    </text>
  </svg>
</div>

<div class="container mx-auto px-4 py-8 max-w-md">
  <div class="bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] text-white rounded-2xl  p-6 mb-6">
    <div class="relative w-64 h-64 mx-auto mb-4">
      <svg class="w-full h-full" viewBox="0 0 100 100">
        <!-- Background circle -->
        <circle
        class="text-[#252525] "
        stroke-width="5"
          stroke="currentColor"
          fill="transparent"
          r="45"
          cx="50"
          cy="50"
        />
        <!-- Timer progress -->
        <circle
          class="{timer.is_break
            ? 'text-green-500'
            : 'text-red-500'} transform -rotate-90 origin-center"
          stroke-width="5"
          stroke-dasharray={strokeDasharray}
          stroke-linecap="round"
          stroke="currentColor"
          fill="transparent"
          r="45"
          cx="50"
          cy="50"
        />
        <!-- Timer text -->
        <text
          x="50"
          y="50"
          font-size="16"
          fill="white"
          text-anchor="middle"
          alignment-baseline="central"
          class="font-bold"
        >
          {timeString}
        </text>
      </svg>
    </div>
    <div class="flex justify-center space-x-4">
      {#if !timer.is_active}
        <button
          on:click={startTimer}
          class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-full transition duration-300"
        >
          Start
        </button>
      {:else}
        <button
          on:click={pauseTimer}
          class="bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-2 px-4 rounded-full transition duration-300"
        >
          Pause
        </button>
      {/if}
      <button
        on:click={resetTimer}
        class="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded-full transition duration-300"
      >
        Reset
      </button>
    </div>
  </div>

  <div class="text-center">
    <p class="text-xl font-semibold">
      Completed Pomodoros: {timer.completed_pomodoros}
    </p>
  </div>
</div>

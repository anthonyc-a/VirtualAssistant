<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let weather: any = null;
  let lat = 33.44;
  let lon = -94.04;
  let error = "";

  async function fetchWeather() {
    try {
      weather = await invoke("get_weather", { lat, lon });
      error = "";
    } catch (e:any) {
      error = e.toString();
      weather = null;
    }
  }

  onMount(fetchWeather);
</script>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-medium mb-6 text-center">Weather Dashboard</h1>

  <div class="max-w-md mx-auto mb-4">
    <input
      bind:value={lat}
      type="number"
      placeholder="Latitude"
      class="w-full p-2 border border-[#252525]   bg-[#252525] bg-opacity-30 backdrop-blur-sm rounded-lg mb-2"
    />
    <input
      bind:value={lon}
      type="number"
      placeholder="Longitude"
      class="w-full p-2  bg-[#252525] bg-opacity-30 backdrop-blur-sm border border-[#252525] rounded-lg mb-2"
    />
    <button
      on:click={fetchWeather}
      class="w-full bg-[#934CF6] shadow-md shadow-[#934cf631] text-white p-2 rounded-[10px] hover:bg-white hover:text-black hover:shadow-none transition-all duration-300"
    >
      Get Weather
    </button>
  </div>

  {#if error}
    <p class="text-red-500 text-center">{error}</p>
  {:else if weather}
    <div class="bg-white shadow-lg rounded-lg p-6">
      <h2 class="text-2xl font-semibold mb-4">
        Weather for {weather.timezone}
      </h2>
      <p class="text-4xl font-bold mb-2">{weather.current.temp.toFixed(1)}°C</p>
      <p class="text-lg capitalize mb-2">
        {weather.current.weather[0].description}
      </p>
      <p>Feels like: {weather.current.feels_like.toFixed(1)}°C</p>
      <p>Humidity: {weather.current.humidity}%</p>
      <p>Wind Speed: {weather.current.wind_speed.toFixed(1)} m/s</p>
    </div>
  {:else}
    <p class="text-center">Loading weather data...</p>
  {/if}
</div>

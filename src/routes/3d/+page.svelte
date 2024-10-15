<script>
    import { onMount } from 'svelte';
    import Cube from "../../components/Misc/Cube.svelte";
    import StlViewer from "../../components/Misc/STLViewer.svelte";
  
    let selectedModel = 'shell';
    $: stlPath = `/${selectedModel}.stl`;
    $: texturePaths = {
      diffuse: `/textures/${selectedModel}/base.jpg`,
      normal: `/textures/${selectedModel}/normal.jpg`,
      roughness: `/textures/${selectedModel}/roughness.jpg`,
    };
  </script>
  
  <div class="w-[calc(100%-32px)] rounded-2xl overflow-hidden max-w-2xl mx-auto">
    <div class="mb-4">
      <label for="model-select" class="mr-2">Select Model:</label>
      <select id="model-select" class="bg-background" bind:value={selectedModel}>
        <option value="shell">Shell</option>
        <option value="chair">Chair</option>
        <option value="choc">Splash</option>
      </select>
    </div>
  
    {#key selectedModel}
      <StlViewer
        {stlPath}
        backgroundColor="#e0e0e0"
        {texturePaths}
        fallbackColor="#ffffff"
        initialRotation={{ x: 90, y: 180, z: 180 }}
      />
    {/key}
  </div>
<script>
    import { onMount } from 'svelte';
    import Cube from "../../components/Misc/Cube.svelte";
    import StlViewer from "../../components/Misc/STLViewer.svelte";
  
    let selectedModel = 'shell';
    let uploadedFile = null;
    let stlPath = '';
    let texturePaths = {};
    let isUsingPredefined = true;
  
    $: {
      if (uploadedFile && !isUsingPredefined) {
        stlPath = URL.createObjectURL(uploadedFile);
        texturePaths = {}; // Clear texture paths for uploaded files
      } else {
        stlPath = `/${selectedModel}.stl`;
        texturePaths = {
          diffuse: `/textures/${selectedModel}/base.jpg`,
          normal: `/textures/${selectedModel}/normal.jpg`,
          roughness: `/textures/${selectedModel}/roughness.jpg`,
        };
      }
    }
  
    function handleFileUpload(event) {
      const file = event.target.files[0];
      if (file && file.name.endsWith('.stl')) {
        uploadedFile = file;
        // Store file in local storage
        const reader = new FileReader();
        reader.onload = (e) => {
          localStorage.setItem('uploadedSTL', e.target.result);
        };
        reader.readAsDataURL(file);
        isUsingPredefined = false;
      } else {
        alert('Please upload a valid STL file.');
      }
    }
  
    function toggleMode() {
      isUsingPredefined = !isUsingPredefined;
      if (isUsingPredefined) {
        uploadedFile = null;
        localStorage.removeItem('uploadedSTL');
      }
    }
  
    onMount(() => {
      // Check if there's a stored STL file in local storage
      const storedSTL = localStorage.getItem('uploadedSTL');
      if (storedSTL) {
        const blob = dataURLtoBlob(storedSTL);
        uploadedFile = new File([blob], 'uploaded.stl', { type: 'application/vnd.ms-pki.stl' });
        isUsingPredefined = false;
      }
    });
  
    function dataURLtoBlob(dataurl) {
      var arr = dataurl.split(','), mime = arr[0].match(/:(.*?);/)[1],
          bstr = atob(arr[1]), n = bstr.length, u8arr = new Uint8Array(n);
      while(n--){
          u8arr[n] = bstr.charCodeAt(n);
      }
      return new Blob([u8arr], {type:mime});
    }
  </script>
  
  <div class="w-[calc(100%-32px)] rounded-2xl overflow-hidden max-w-2xl mx-auto">
    <div class="mb-4 flex justify-between items-center">
      <button 
        class="px-4 py-2 rounded {isUsingPredefined ? 'bg-blue-500 text-white' : 'bg-gray-200'}"
        on:click={toggleMode}
      >
        Pre-defined Models
      </button>
      <button 
        class="px-4 py-2 rounded {!isUsingPredefined ? 'bg-blue-500 text-white' : 'bg-gray-200'}"
        on:click={toggleMode}
      >
        Upload STL File
      </button>
    </div>
  
    {#if isUsingPredefined}
      <div class="mb-4">
        <label for="model-select" class="mr-2">Select Model:</label>
        <select id="model-select" class="bg-background" bind:value={selectedModel}>
          <option value="shell">Shell</option>
          <option value="chair">Chair</option>
          <option value="choc">Splash</option>
        </select>
      </div>
    {:else}
      <div class="mb-4">
        <label for="file-upload" class="mr-2">Upload STL File:</label>
        <input type="file" id="file-upload" accept=".stl" on:change={handleFileUpload} />
      </div>
    {/if}
  
    {#key stlPath}
      <StlViewer
        {stlPath}
        backgroundColor="#e0e0e0"
        {texturePaths}
        fallbackColor="#ffffff"
        initialRotation={{ x: 90, y: 180, z: 180 }}
      />
    {/key}
  </div>
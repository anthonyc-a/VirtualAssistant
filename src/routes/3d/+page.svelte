<script>
  import Cube from "../../components/Misc/Cube.svelte";
  import StlViewer from "../../components/Misc/STLViewer.svelte";
</script>

<div class="w-full rounded-2xl overflow-hidden max-w-2xl mx-auto">
  <StlViewer
    stlPath="/choc.stl"
    texturePaths={{
      normal: "/textures/splash/Chocolate_Splash_Free_002_norm.tga",
      ao: "/textures/splash/Chocolate_Splash_Free_002_ao.tga",
      roughness: "/textures/splash/Chocolate_Splash_Free_002_rough.tga",
      metalness: "/textures/splash/Chocolate_Splash_Free_002_metal.tga",
    }}
    fallbackColor="#8B4513"
  />
  <StlViewer
    stlPath="/chair.stl"
    texturePaths={{
  normal: "/textures/splash/Chocolate_Splash_Free_002_norm.tga",
      ao: "/textures/splash/Chocolate_Splash_Free_002_ao.tga",
      roughness: "/textures/splash/Chocolate_Splash_Free_002_rough.tga",
      metalness: "/textures/splash/Chocolate_Splash_Free_002_metal.tga",
    }}
    fallbackColor="#8B4513"
  />
</div>

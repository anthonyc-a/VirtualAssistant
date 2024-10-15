<script>
  import Cube from "../../components/Misc/Cube.svelte";
  import StlViewer from "../../components/Misc/STLViewer.svelte";
</script>

<div class="w-full max-w-2xl mx-auto">
  <StlViewer
    stlPath="/choc.stl"
    texturePaths={{
      diffuse: "/textures/splash/Chocolate_Splash_Free_002_bcolor.tga",
      normal: "/textures/splash/Chocolate_Splash_Free_002_norm.tga",
      ao: "/textures/splash/Chocolate_Splash_Free_002_ao.tga",
      roughness: "/textures/splash/Chocolate_Splash_Free_002_rough.tga",
      metalness: "/textures/splash/Chocolate_Splash_Free_002_metal.tga",
    }}
  />
</div>
